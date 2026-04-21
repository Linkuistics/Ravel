defmodule Ravel.VaultDirectory do
  @moduledoc """
  Vault walk, frontmatter parse, and invariant checks per sub-F spec §3.1
  and §1.5. Produces an in-memory snapshot of every plan and expert in the
  vault with path-based qualified IDs (F-5) that are never stored.

  `load/1` returns `{:ok, %__MODULE__{}}` or `{:error, %Error{}}`.
  `load!/1` raises `Ravel.VaultDirectory.Error` on any failure.

  Every invariant violation is a hard error per spec §9.3, with a
  structured reason and enough locating information (`path`,
  `qualified_id`, or `expert_id`) for callers to surface actionable
  diagnostics.
  """

  alias Ravel.FileIO
  alias Ravel.VaultDirectory.Error
  alias Ravel.VaultDirectory.Expert
  alias Ravel.VaultDirectory.Plan

  @enforce_keys [:vault_path]
  defstruct [:vault_path, plans: %{}, experts: %{}, errors: []]

  @type t :: %__MODULE__{
          vault_path: Path.t(),
          plans: %{String.t() => Plan.t()},
          experts: %{String.t() => Expert.t()},
          errors: [term()]
        }

  @placeholder_descriptions ~w(todo fixme ... tbd)

  @spec load(Path.t()) :: {:ok, t()} | {:error, Error.t()}
  def load(vault_path) when is_binary(vault_path) do
    {:ok, load!(vault_path)}
  rescue
    err in Error -> {:error, err}
  end

  @spec load!(Path.t()) :: t()
  def load!(vault_path) when is_binary(vault_path) do
    projects_dir = Path.join(vault_path, "projects")

    unless File.dir?(projects_dir) do
      raise Error,
        reason: :no_projects_dir,
        path: projects_dir,
        message: "vault has no projects/ directory at #{projects_dir}"
    end

    plans = walk_all_projects(vault_path, projects_dir)
    experts = enumerate_experts(vault_path)

    plans = assign_parents(plans)

    %__MODULE__{
      vault_path: vault_path,
      plans: plans,
      experts: experts,
      errors: []
    }
  end

  # --- project-level walk -------------------------------------------------

  defp walk_all_projects(vault_path, projects_dir) do
    projects_dir
    |> File.ls!()
    |> Enum.sort()
    |> Enum.reduce(%{}, fn entry, acc ->
      Map.merge(acc, walk_project(vault_path, projects_dir, entry))
    end)
  end

  defp walk_project(vault_path, projects_dir, name) do
    logical_path = Path.join(projects_dir, name)
    canonical_ravel_dir = verify_project_symlink!(logical_path, name)
    verify_project_shape!(canonical_ravel_dir, name)

    project_root_dir = Path.join(canonical_ravel_dir, "project-root")
    root_qualified_id = "#{name}/project-root"

    plans =
      walk_plan_tree(
        vault_path,
        name,
        project_root_dir,
        root_qualified_id,
        _inside_project_root = true,
        %{}
      )

    enforce_project_root_present!(plans, name, project_root_dir)
    plans
  end

  defp enforce_project_root_present!(plans, project_name, project_root_dir) do
    root_qualified_id = "#{project_name}/project-root"

    if Map.has_key?(plans, root_qualified_id) do
      :ok
    else
      orphaned_child =
        plans
        |> Map.keys()
        |> Enum.find(&String.starts_with?(&1, root_qualified_id <> "/"))

      if orphaned_child do
        raise Error,
          reason: :orphaned_children,
          qualified_id: root_qualified_id,
          path: Path.join(project_root_dir, "plan-state.md"),
          message:
            "project-root at #{root_qualified_id} has no plan-state.md but child plan " <>
              "#{orphaned_child} exists; restore the root or delete its children"
      else
        raise Error,
          reason: :missing_project_root,
          qualified_id: root_qualified_id,
          path: Path.join(project_root_dir, "plan-state.md"),
          message:
            "project #{project_name} has project-root/ directory but no plan-state.md inside it"
      end
    end
  end

  defp verify_project_symlink!(logical_path, _name) do
    case File.read_link(logical_path) do
      {:ok, target} ->
        canonical = expand_symlink_target(logical_path, target)

        cond do
          not File.dir?(canonical) ->
            raise Error,
              reason: :broken_project_symlink,
              path: logical_path,
              message:
                "project symlink #{logical_path} points at #{target} which does not resolve to a directory"

          Path.basename(canonical) != "ravel" ->
            raise Error,
              reason: :project_symlink_wrong_shape,
              path: logical_path,
              message: "project symlink #{logical_path} target #{canonical} must end in /ravel"

          true ->
            canonical
        end

      {:error, :einval} ->
        raise Error,
          reason: :non_symlink_project_entry,
          path: logical_path,
          message:
            "vault entry #{logical_path} is not a symlink; " <>
              "every <vault>/projects/<name> must be a symlink into a project's ravel/ directory"

      {:error, reason} ->
        raise Error,
          reason: :broken_project_symlink,
          path: logical_path,
          message: "cannot read project symlink #{logical_path}: #{inspect(reason)}"
    end
    |> case do
      canonical when is_binary(canonical) -> canonical
    end
  end

  defp expand_symlink_target(logical_path, target) do
    cond do
      Path.type(target) == :absolute -> Path.expand(target)
      true -> Path.expand(target, Path.dirname(logical_path))
    end
  end

  defp verify_project_shape!(canonical_ravel_dir, project_name) do
    knowledge_dir = Path.join(canonical_ravel_dir, "knowledge")
    project_root_dir = Path.join(canonical_ravel_dir, "project-root")

    unless File.dir?(knowledge_dir) do
      raise Error,
        reason: :missing_project_knowledge_dir,
        path: knowledge_dir,
        message:
          "project #{project_name} has no knowledge/ dir at #{knowledge_dir}; " <>
            "every adopted project must ship <project>/ravel/knowledge/"
    end

    unless File.dir?(project_root_dir) do
      raise Error,
        reason: :missing_project_root,
        path: project_root_dir,
        qualified_id: "#{project_name}/project-root",
        message:
          "project #{project_name} has no project-root/ at #{project_root_dir}; " <>
            "every adopted project must have exactly one project-root plan"
    end

    enforce_knowledge_isolation!(knowledge_dir)
  end

  defp enforce_knowledge_isolation!(knowledge_dir) do
    stray_marker = Path.join(knowledge_dir, "plan-state.md")

    if File.exists?(stray_marker) do
      raise Error,
        reason: :knowledge_isolation_violation,
        path: stray_marker,
        message:
          "plan-state.md found inside knowledge/ at #{stray_marker}; " <>
            "knowledge/ must never host a plan (invariant 6)"
    end

    # Also reject plan-state.md in any knowledge subtree.
    knowledge_dir
    |> list_subdirectories()
    |> Enum.each(&enforce_knowledge_isolation!/1)
  end

  # --- plan tree walk -----------------------------------------------------

  defp walk_plan_tree(vault_path, project, dir, qualified_id, inside_project_root?, plans) do
    marker_path = Path.join(dir, "plan-state.md")

    plans =
      if File.regular?(marker_path) do
        enforce_reserved_name!(qualified_id, inside_project_root?)
        plan = parse_plan!(vault_path, project, dir, qualified_id)
        Map.put(plans, qualified_id, plan)
      else
        plans
      end

    dir
    |> list_subdirectories()
    |> Enum.reduce(plans, fn child, acc ->
      child_name = Path.basename(child)
      child_qualified_id = "#{qualified_id}/#{child_name}"
      walk_plan_tree(vault_path, project, child, child_qualified_id, false, acc)
    end)
  end

  defp enforce_reserved_name!(qualified_id, _inside_project_root?) do
    # "project-root" is allowed only at the single reserved location —
    # the second segment of the qualified ID ("<project>/project-root").
    nested_root_segments =
      qualified_id
      |> String.split("/")
      |> Enum.drop(2)
      |> Enum.filter(&(&1 == "project-root"))

    if nested_root_segments != [] do
      raise Error,
        reason: :reserved_name_violation,
        qualified_id: qualified_id,
        message:
          "plan directory named \"project-root\" found at #{qualified_id}; " <>
            "reserved for the single root plan (invariants 4/8)"
    end
  end

  defp parse_plan!(_vault_path, project, dir, qualified_id) do
    marker_path = Path.join(dir, "plan-state.md")
    content = read_file!(marker_path)
    frontmatter = parse_frontmatter!(content, marker_path)
    description = extract_description!(frontmatter, qualified_id: qualified_id, path: marker_path)

    %Plan{
      qualified_id: qualified_id,
      project: project,
      path: dir,
      parent_qualified_id: nil,
      description: description,
      frontmatter: frontmatter
    }
  end

  # --- expert enumeration -------------------------------------------------

  defp enumerate_experts(vault_path) do
    experts_dir = Path.join(vault_path, "experts")

    if File.dir?(experts_dir) do
      experts_dir
      |> File.ls!()
      |> Enum.filter(&String.ends_with?(&1, ".md"))
      |> Enum.sort()
      |> Enum.reduce(%{}, fn filename, acc ->
        expert_id = Path.basename(filename, ".md")
        path = Path.join(experts_dir, filename)
        expert = parse_expert!(expert_id, path)
        Map.put(acc, expert_id, expert)
      end)
    else
      %{}
    end
  end

  defp parse_expert!(expert_id, path) do
    content = read_file!(path)
    frontmatter = parse_frontmatter!(content, path)
    description = extract_description!(frontmatter, expert_id: expert_id, path: path)

    %Expert{
      expert_id: expert_id,
      path: path,
      description: description,
      frontmatter: frontmatter
    }
  end

  # --- frontmatter parsing ------------------------------------------------

  defp parse_frontmatter!(content, path) do
    case split_frontmatter(content) do
      {:ok, yaml_text} ->
        case YamlElixir.read_from_string(yaml_text) do
          {:ok, map} when is_map(map) ->
            map

          {:ok, _non_map} ->
            raise Error,
              reason: :frontmatter_parse_error,
              path: path,
              message: "frontmatter at #{path} is not a YAML mapping"

          {:error, reason} ->
            raise Error,
              reason: :frontmatter_parse_error,
              path: path,
              message: "failed to parse YAML frontmatter at #{path}: #{inspect(reason)}"
        end

      :missing ->
        raise Error,
          reason: :missing_frontmatter,
          path: path,
          message:
            "no YAML frontmatter found at #{path}; expected opening \"---\" fence on line 1"
    end
  end

  # Accepts leading BOM / blank lines before the first fence to tolerate
  # editors that insert them. Everything between the first pair of `---`
  # lines is treated as YAML.
  defp split_frontmatter(content) do
    lines = String.split(content, "\n")

    case Enum.drop_while(lines, &(String.trim(&1) == "" or String.trim(&1) == "﻿")) do
      ["---" | rest] ->
        case Enum.split_while(rest, &(&1 != "---")) do
          {_, []} -> :missing
          {body, ["---" | _]} -> {:ok, Enum.join(body, "\n")}
        end

      _other ->
        :missing
    end
  end

  # --- description validation ---------------------------------------------

  defp extract_description!(frontmatter, locator) do
    case Map.get(frontmatter, "description") do
      nil ->
        raise Error,
          reason: :missing_description,
          qualified_id: Keyword.get(locator, :qualified_id),
          expert_id: Keyword.get(locator, :expert_id),
          path: Keyword.get(locator, :path),
          message: "frontmatter is missing required `description` field"

      desc when is_binary(desc) ->
        validate_description!(desc, locator)
        desc

      other ->
        raise Error,
          reason: :missing_description,
          qualified_id: Keyword.get(locator, :qualified_id),
          expert_id: Keyword.get(locator, :expert_id),
          path: Keyword.get(locator, :path),
          message: "`description` must be a string, got #{inspect(other)}"
    end
  end

  defp validate_description!(desc, locator) do
    trimmed = String.trim(desc)

    cond do
      trimmed == "" ->
        raise Error,
          reason: :missing_description,
          qualified_id: Keyword.get(locator, :qualified_id),
          expert_id: Keyword.get(locator, :expert_id),
          path: Keyword.get(locator, :path),
          message: "`description` must not be empty"

      String.length(desc) > 120 ->
        raise Error,
          reason: :description_too_long,
          qualified_id: Keyword.get(locator, :qualified_id),
          expert_id: Keyword.get(locator, :expert_id),
          path: Keyword.get(locator, :path),
          message: "`description` has #{String.length(desc)} characters; hard cap is 120 (F-8)"

      String.downcase(trimmed) in @placeholder_descriptions ->
        raise Error,
          reason: :placeholder_description,
          qualified_id: Keyword.get(locator, :qualified_id),
          expert_id: Keyword.get(locator, :expert_id),
          path: Keyword.get(locator, :path),
          message:
            "`description` is a placeholder (#{inspect(trimmed)}); " <>
              "write a scope-dense noun phrase per spec §1.4"

      true ->
        :ok
    end
  end

  # --- parent assignment --------------------------------------------------

  defp assign_parents(plans) do
    Map.new(plans, fn {qualified_id, plan} ->
      parent = find_nearest_ancestor_plan(qualified_id, plans)
      {qualified_id, %{plan | parent_qualified_id: parent}}
    end)
  end

  defp find_nearest_ancestor_plan(qualified_id, plans) do
    qualified_id
    |> String.split("/")
    |> Enum.drop(-1)
    |> ancestor_chain([])
    |> Enum.find(&Map.has_key?(plans, &1))
  end

  defp ancestor_chain([], acc), do: acc

  defp ancestor_chain(segments, acc) do
    candidate = Enum.join(segments, "/")
    ancestor_chain(Enum.drop(segments, -1), acc ++ [candidate])
  end

  # --- filesystem helpers -------------------------------------------------

  defp list_subdirectories(dir) do
    case File.ls(dir) do
      {:ok, entries} ->
        entries
        |> Enum.map(&Path.join(dir, &1))
        |> Enum.filter(&directory?/1)
        |> Enum.sort()

      {:error, _} ->
        []
    end
  end

  defp directory?(path) do
    # File.dir?/1 follows symlinks, which we specifically don't want
    # inside a project tree — spec §3.4 says symlinks inside a project
    # subtree are a hard error. But for v1, an unexpected symlink just
    # gets skipped via File.lstat showing :symlink, not descended into.
    case File.lstat(path) do
      {:ok, %File.Stat{type: :directory}} -> true
      _ -> false
    end
  end

  defp read_file!(path) do
    case FileIO.read_tracked(path) do
      {:ok, content} ->
        content

      {:error, reason} ->
        raise Error,
          reason: :frontmatter_parse_error,
          path: path,
          message: "cannot read #{path}: #{inspect(reason)}"
    end
  end
end
