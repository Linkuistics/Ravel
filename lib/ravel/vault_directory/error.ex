defmodule Ravel.VaultDirectory.Error do
  @moduledoc """
  Raised (or returned as `{:error, %__MODULE__{}}`) when vault walk
  encounters an invariant violation or I/O failure.

  Every instance must carry a `:reason` atom (machine-readable),
  a human-readable `:message`, and at least one of the locating
  fields — `:path`, `:qualified_id`, or `:expert_id` — so diagnostics
  name the offending file or actor per spec §9.3.
  """

  defexception [:reason, :message, :path, :qualified_id, :expert_id]

  @type reason ::
          :no_projects_dir
          | :no_projects
          | :non_symlink_project_entry
          | :broken_project_symlink
          | :project_symlink_wrong_shape
          | :missing_project_knowledge_dir
          | :missing_project_root
          | :unexpected_project_entry
          | :missing_frontmatter
          | :frontmatter_parse_error
          | :missing_description
          | :description_too_long
          | :placeholder_description
          | :reserved_name_violation
          | :orphaned_children
          | :knowledge_isolation_violation
          | :unexpected_symlink

  @type t :: %__MODULE__{
          reason: reason(),
          message: String.t(),
          path: Path.t() | nil,
          qualified_id: String.t() | nil,
          expert_id: String.t() | nil
        }
end
