defmodule Ravel.VaultDirectory.Plan do
  @moduledoc """
  An entry in `Ravel.VaultDirectory.plans` describing one loaded plan.

  `qualified_id` is the authoritative logical identifier
  (spec §F-5: `strip_prefix(plan_path, "<vault>/projects/")`) and is
  never stored in frontmatter. `path` is the canonical (symlink-resolved)
  filesystem path used for all subsequent I/O.
  """

  @enforce_keys [:qualified_id, :project, :path, :description, :frontmatter]
  defstruct [:qualified_id, :project, :path, :parent_qualified_id, :description, :frontmatter]

  @type t :: %__MODULE__{
          qualified_id: String.t(),
          project: String.t(),
          path: Path.t(),
          parent_qualified_id: String.t() | nil,
          description: String.t(),
          frontmatter: %{String.t() => term()}
        }
end
