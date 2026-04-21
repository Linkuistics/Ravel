defmodule Ravel.VaultDirectory.Expert do
  @moduledoc """
  An entry in `Ravel.VaultDirectory.experts` describing one expert
  declaration at `<vault>/experts/<expert-id>.md`.

  Only the fields F cares about are populated here — persona,
  retrieval strategy, and knowledge scope (sub-N) are read from
  `frontmatter` by sub-N's loader.
  """

  @enforce_keys [:expert_id, :path, :description, :frontmatter]
  defstruct [:expert_id, :path, :description, :frontmatter]

  @type t :: %__MODULE__{
          expert_id: String.t(),
          path: Path.t(),
          description: String.t(),
          frontmatter: %{String.t() => term()}
        }
end
