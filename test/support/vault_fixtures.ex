defmodule Ravel.Test.VaultFixtures do
  @moduledoc false

  # Builds a minimal but realistic vault tree for VaultDirectory tests.
  #
  # Layout created per spec §1.1:
  #
  #   <root>/vault/
  #     projects/<project>   -> symlink to <root>/sources/<project>/ravel
  #     experts/
  #   <root>/sources/<project>/ravel/
  #     knowledge/.keep
  #     project-root/plan-state.md
  #     project-root/.../plan-state.md   (nested plans)

  @spec new_vault(Path.t()) :: Path.t()
  def new_vault(base_dir) do
    vault = Path.join(base_dir, "vault")
    File.mkdir_p!(Path.join(vault, "projects"))
    File.mkdir_p!(Path.join(vault, "experts"))
    vault
  end

  @spec add_project_with_root(Path.t(), String.t(), String.t()) :: Path.t()
  def add_project_with_root(vault, project_name, root_description) do
    sources_root =
      vault
      |> Path.dirname()
      |> Path.join(["sources"])

    project_ravel = Path.join([sources_root, project_name, "ravel"])
    File.mkdir_p!(Path.join(project_ravel, "knowledge"))
    File.write!(Path.join([project_ravel, "knowledge", ".keep"]), "")
    File.mkdir_p!(Path.join(project_ravel, "project-root"))

    File.write!(
      Path.join([project_ravel, "project-root", "plan-state.md"]),
      plan_state_content(root_description)
    )

    File.ln_s!(project_ravel, Path.join([vault, "projects", project_name]))

    project_ravel
  end

  # qualified_id is the full relative path beneath projects/,
  # e.g. "Ravel/project-root/sub-A". The directory beneath project-root/
  # is the plan's basename.
  @spec add_plan(Path.t(), String.t(), String.t()) :: Path.t()
  def add_plan(vault, qualified_id, description) do
    [project | rest] = String.split(qualified_id, "/")

    sources_root =
      vault
      |> Path.dirname()
      |> Path.join(["sources"])

    plan_dir =
      [sources_root, project, "ravel" | rest]
      |> Path.join()

    File.mkdir_p!(plan_dir)
    File.write!(Path.join(plan_dir, "plan-state.md"), plan_state_content(description))
    plan_dir
  end

  @spec add_expert(Path.t(), String.t(), String.t()) :: Path.t()
  def add_expert(vault, expert_id, description) do
    path = Path.join([vault, "experts", "#{expert_id}.md"])

    File.write!(
      path,
      """
      ---
      schema-version: 1
      description: #{description}
      ---

      # Expert declaration for #{expert_id}
      """
    )

    path
  end

  @spec write_raw_plan_state(Path.t(), String.t(), String.t()) :: Path.t()
  def write_raw_plan_state(vault, qualified_id_or_rel_path, content) do
    parts = String.split(qualified_id_or_rel_path, "/")
    [project | rest] = parts

    sources_root =
      vault
      |> Path.dirname()
      |> Path.join(["sources"])

    plan_dir = [sources_root, project, "ravel" | rest] |> Path.join()
    File.mkdir_p!(plan_dir)
    path = Path.join(plan_dir, "plan-state.md")
    File.write!(path, content)
    path
  end

  defp plan_state_content(description) do
    """
    ---
    schema-version: 1
    description: #{description}
    ---

    # Plan state
    """
  end
end
