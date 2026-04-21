defmodule Ravel.VaultDirectoryTest do
  use ExUnit.Case, async: false

  import Ravel.Test.VaultFixtures

  alias Ravel.Coordination.FreshnessTracker
  alias Ravel.VaultDirectory

  setup do
    start_supervised!(FreshnessTracker)

    base =
      Path.join(
        System.tmp_dir!(),
        "ravel_vault_directory_test_#{System.unique_integer([:positive])}"
      )

    File.mkdir_p!(base)
    on_exit(fn -> File.rm_rf!(base) end)

    %{base: base, vault: new_vault(base)}
  end

  describe "load/1 — happy paths" do
    test "returns struct with vault path, one plan, no experts for minimal vault",
         %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Plan hierarchy and actor model scope for Ravel.")

      assert {:ok, %VaultDirectory{} = vd} = VaultDirectory.load(vault)
      assert vd.vault_path == vault
      assert vd.experts == %{}
      assert vd.errors == []
      assert Map.keys(vd.plans) == ["Ravel/project-root"]
    end

    test "computes path-based qualified id matching strip_prefix(plan_path, vault/projects/)",
         %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")
      add_plan(vault, "Ravel/project-root/sub-F-hierarchy", "Sub-F scope.")

      assert {:ok, vd} = VaultDirectory.load(vault)

      assert Map.keys(vd.plans) |> Enum.sort() ==
               ["Ravel/project-root", "Ravel/project-root/sub-F-hierarchy"]

      plan = vd.plans["Ravel/project-root/sub-F-hierarchy"]
      assert plan.qualified_id == "Ravel/project-root/sub-F-hierarchy"
      assert plan.project == "Ravel"
      assert plan.description == "Sub-F scope."
    end

    test "assigns parent_qualified_id to nearest ancestor plan-state.md", %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")
      add_plan(vault, "Ravel/project-root/sub-A", "Sub-A scope.")
      add_plan(vault, "Ravel/project-root/sub-A/sub-A-alpha", "Alpha scope.")

      assert {:ok, vd} = VaultDirectory.load(vault)
      assert vd.plans["Ravel/project-root"].parent_qualified_id == nil
      assert vd.plans["Ravel/project-root/sub-A"].parent_qualified_id == "Ravel/project-root"

      assert vd.plans["Ravel/project-root/sub-A/sub-A-alpha"].parent_qualified_id ==
               "Ravel/project-root/sub-A"
    end

    test "skips organizational directories without plan-state.md", %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")
      project_ravel = Path.join([Path.dirname(vault), "sources", "Ravel", "ravel"])
      File.mkdir_p!(Path.join([project_ravel, "project-root", "notes", "archive"]))

      add_plan(vault, "Ravel/project-root/notes/archive/leaf", "Leaf plan nested under notes/.")

      assert {:ok, vd} = VaultDirectory.load(vault)

      assert Map.keys(vd.plans) |> Enum.sort() ==
               ["Ravel/project-root", "Ravel/project-root/notes/archive/leaf"]

      leaf = vd.plans["Ravel/project-root/notes/archive/leaf"]
      assert leaf.parent_qualified_id == "Ravel/project-root"
    end

    test "enumerates multiple projects independently", %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Ravel root scope.")
      add_project_with_root(vault, "APIAnyware-MacOS", "APIAnyware root scope.")

      assert {:ok, vd} = VaultDirectory.load(vault)

      assert Map.keys(vd.plans) |> Enum.sort() ==
               ["APIAnyware-MacOS/project-root", "Ravel/project-root"]
    end

    test "enumerates expert declarations from <vault>/experts/*.md", %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")
      add_expert(vault, "rust-expert", "Rust language, ownership, lifetimes, cargo tooling.")
      add_expert(vault, "research-expert", "Literature synthesis and citation tracing scope.")

      assert {:ok, vd} = VaultDirectory.load(vault)
      assert Map.keys(vd.experts) |> Enum.sort() == ["research-expert", "rust-expert"]

      rust = vd.experts["rust-expert"]
      assert rust.expert_id == "rust-expert"
      assert rust.description =~ "Rust"
    end
  end

  describe "load/1 — vault-level invariants (§9.3)" do
    test "hard error when <vault>/projects/<name> is not a symlink", %{vault: vault} do
      File.mkdir_p!(Path.join([vault, "projects", "NotASymlink"]))

      assert {:error, err} = VaultDirectory.load(vault)
      assert err.reason == :non_symlink_project_entry
      assert err.path =~ "projects/NotASymlink"
    end

    test "hard error when project symlink is broken", %{vault: vault} do
      File.ln_s!(
        "/tmp/definitely-does-not-exist-#{System.unique_integer([:positive])}",
        Path.join([vault, "projects", "Ghost"])
      )

      assert {:error, err} = VaultDirectory.load(vault)
      assert err.reason == :broken_project_symlink
      assert err.path =~ "projects/Ghost"
    end

    test "hard error when project symlink target does not end in /ravel/", %{vault: vault} do
      sources = Path.join(Path.dirname(vault), "sources/WrongShape/not-ravel")
      File.mkdir_p!(sources)
      File.ln_s!(sources, Path.join([vault, "projects", "WrongShape"]))

      assert {:error, err} = VaultDirectory.load(vault)
      assert err.reason == :project_symlink_wrong_shape
      assert err.path =~ "WrongShape"
    end

    test "hard error when <project>/ravel/knowledge/ is missing", %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")
      File.rm_rf!(Path.join([Path.dirname(vault), "sources", "Ravel", "ravel", "knowledge"]))

      assert {:error, err} = VaultDirectory.load(vault)
      assert err.reason == :missing_project_knowledge_dir
      assert err.path =~ "Ravel/ravel/knowledge"
    end

    test "hard error when <project>/ravel/project-root/ is missing", %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")

      File.rm_rf!(Path.join([Path.dirname(vault), "sources", "Ravel", "ravel", "project-root"]))

      assert {:error, err} = VaultDirectory.load(vault)
      assert err.reason == :missing_project_root
      assert err.path =~ "Ravel/ravel/project-root"
    end
  end

  describe "load/1 — plan invariants (§1.5, §9.2)" do
    test "hard error on malformed YAML frontmatter", %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")

      write_raw_plan_state(vault, "Ravel/project-root/sub-bad", """
      ---
      schema-version: 1
      description: [unterminated
      ---

      # Bad plan
      """)

      assert {:error, err} = VaultDirectory.load(vault)
      assert err.reason == :frontmatter_parse_error
      assert err.path =~ "sub-bad/plan-state.md"
    end

    test "hard error when plan-state.md has no YAML frontmatter fence", %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")

      write_raw_plan_state(
        vault,
        "Ravel/project-root/sub-no-fm",
        "# No frontmatter at all\n"
      )

      assert {:error, err} = VaultDirectory.load(vault)
      assert err.reason == :missing_frontmatter
      assert err.path =~ "sub-no-fm/plan-state.md"
    end

    test "hard error when description field is missing", %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")

      write_raw_plan_state(vault, "Ravel/project-root/sub-nodesc", """
      ---
      schema-version: 1
      ---

      # No description
      """)

      assert {:error, err} = VaultDirectory.load(vault)
      assert err.reason == :missing_description
      assert err.qualified_id == "Ravel/project-root/sub-nodesc"
    end

    test "hard error when description exceeds 120 characters", %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")

      oversize = String.duplicate("x", 121)
      add_plan(vault, "Ravel/project-root/sub-oversize", oversize)

      assert {:error, err} = VaultDirectory.load(vault)
      assert err.reason == :description_too_long
      assert err.qualified_id == "Ravel/project-root/sub-oversize"
    end

    test "description of exactly 120 characters is accepted", %{vault: vault} do
      exact = String.duplicate("x", 120)
      add_project_with_root(vault, "Ravel", exact)

      assert {:ok, vd} = VaultDirectory.load(vault)
      assert vd.plans["Ravel/project-root"].description == exact
    end

    for placeholder <- ["TODO", "FIXME", "...", "tbd", "TBD", "Tbd", "todo"] do
      test "hard error when description is placeholder #{inspect(placeholder)}", %{vault: vault} do
        add_project_with_root(vault, "Ravel", unquote(placeholder))

        assert {:error, err} = VaultDirectory.load(vault)
        assert err.reason == :placeholder_description
        assert err.qualified_id == "Ravel/project-root"
      end
    end

    test "hard error on nested directory named project-root (invariant 4/8)", %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")
      add_plan(vault, "Ravel/project-root/project-root", "Illegally nested root-named plan.")

      assert {:error, err} = VaultDirectory.load(vault)
      assert err.reason == :reserved_name_violation
      assert err.qualified_id == "Ravel/project-root/project-root"
    end

    test "hard error when project-root is deleted while children exist", %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")
      add_plan(vault, "Ravel/project-root/sub-orphan", "Orphan child scope.")

      project_root_marker =
        Path.join([
          Path.dirname(vault),
          "sources",
          "Ravel",
          "ravel",
          "project-root",
          "plan-state.md"
        ])

      File.rm!(project_root_marker)

      assert {:error, err} = VaultDirectory.load(vault)
      assert err.reason == :orphaned_children
      assert err.qualified_id == "Ravel/project-root"
    end

    test "hard error when plan-state.md appears inside knowledge/ (invariant 6)",
         %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")

      File.mkdir_p!(Path.join([Path.dirname(vault), "sources", "Ravel", "ravel", "knowledge"]))

      File.write!(
        Path.join([
          Path.dirname(vault),
          "sources",
          "Ravel",
          "ravel",
          "knowledge",
          "plan-state.md"
        ]),
        """
        ---
        schema-version: 1
        description: Knowledge dir should never host a plan marker.
        ---
        """
      )

      assert {:error, err} = VaultDirectory.load(vault)
      assert err.reason == :knowledge_isolation_violation
      assert err.path =~ "Ravel/ravel/knowledge/plan-state.md"
    end
  end

  describe "load/1 — expert invariants (F-8 extends to experts)" do
    test "hard error when expert declaration is missing description", %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")

      File.write!(Path.join([vault, "experts", "bad.md"]), """
      ---
      schema-version: 1
      ---
      """)

      assert {:error, err} = VaultDirectory.load(vault)
      assert err.reason == :missing_description
      assert err.expert_id == "bad"
    end

    test "hard error when expert description exceeds 120 characters", %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")
      add_expert(vault, "oversize", String.duplicate("x", 121))

      assert {:error, err} = VaultDirectory.load(vault)
      assert err.reason == :description_too_long
      assert err.expert_id == "oversize"
    end
  end

  describe "load!/1" do
    test "raises VaultDirectory.Error on any failure", %{vault: vault} do
      File.mkdir_p!(Path.join([vault, "projects", "NotASymlink"]))

      assert_raise VaultDirectory.Error, ~r/not a symlink/, fn ->
        VaultDirectory.load!(vault)
      end
    end

    test "returns struct on success", %{vault: vault} do
      add_project_with_root(vault, "Ravel", "Root scope.")
      assert %VaultDirectory{} = VaultDirectory.load!(vault)
    end

    test "empty vault (no adopted projects) is valid", %{vault: vault} do
      assert %VaultDirectory{plans: plans, experts: experts} = VaultDirectory.load!(vault)
      assert plans == %{}
      assert experts == %{}
    end
  end
end
