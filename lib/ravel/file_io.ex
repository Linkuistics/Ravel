defmodule Ravel.FileIO do
  @moduledoc """
  Mandatory file I/O layer for all daemon reads and writes to plan,
  knowledge, and runtime files. Enforces the read-before-write +
  content-hash freshness check + atomic temp-then-rename discipline
  per sub-D spec §3.3.

  Every daemon read must go through `read_tracked/1` so that later
  `write_safe/3` calls have a baseline to compare against.

  `write_safe/3` dispatches by file category:

    * `:exclusive` — no freshness check, atomic overwrite. For
      machine-generated files (`plan-catalog.md`, `runtime/*`) whose
      external edits are meaningless.
    * `:phase_owned` — daemon-authoritative. On external modification,
      the daemon's write proceeds; a warn-level diagnostic is emitted.
      For phase-cycle files (`plan-state.md`, `memory.md`, `backlog.md`,
      `session-log.md`) during an active phase.
    * `:human_editable` — human-authoritative. On external modification,
      the daemon's write is discarded; an info-level diagnostic is emitted
      and `{:conflict, :external_modification}` is returned. For
      `routing.ex`, `knowledge/**/*.md`, `experts/*.md`, `daemon.toml`,
      `ravel.toml`.
  """

  require Logger

  alias Ravel.Coordination.FreshnessTracker

  @type category :: :exclusive | :phase_owned | :human_editable

  @spec read_tracked(Path.t()) :: {:ok, binary()} | {:error, File.posix()}
  def read_tracked(path) when is_binary(path) do
    case File.read(path) do
      {:ok, content} ->
        :ok = FreshnessTracker.record(path, content)
        {:ok, content}

      {:error, _reason} = err ->
        err
    end
  end

  @spec write_safe(Path.t(), binary(), category()) ::
          :ok | {:conflict, :external_modification}
  def write_safe(path, content, :exclusive)
      when is_binary(path) and is_binary(content) do
    atomic_write!(path, content)
    :ok = FreshnessTracker.record(path, content)
    :ok
  end

  def write_safe(path, content, category)
      when is_binary(path) and is_binary(content) and
             category in [:phase_owned, :human_editable] do
    case FreshnessTracker.check(path) do
      verdict when verdict in [:untracked, :unchanged] ->
        atomic_write!(path, content)
        :ok = FreshnessTracker.record(path, content)
        :ok

      {:changed, _current_hash} ->
        handle_external_modification(path, content, category)

      {:error, :enoent} ->
        handle_deleted_externally(path, content, category)

      {:error, reason} ->
        raise File.Error,
          action: "check freshness",
          path: IO.chardata_to_string(path),
          reason: reason
    end
  end

  defp handle_external_modification(path, content, :phase_owned) do
    atomic_write!(path, content)
    :ok = FreshnessTracker.record(path, content)

    emit_diagnostic(
      :warn,
      "external modification to #{path} overwritten by phase cycle",
      %{path: path, category: :phase_owned, kind: :overwrite}
    )

    :ok
  end

  defp handle_external_modification(path, _content, :human_editable) do
    # Re-read to re-baseline the stored hash to the external version.
    # Prevents infinite discard: without a hash update, every subsequent
    # write_safe call would re-detect the same external modification.
    rebaseline_hash(path)

    emit_diagnostic(
      :info,
      "external modification to #{path} detected; daemon write discarded",
      %{path: path, category: :human_editable, kind: :discard}
    )

    {:conflict, :external_modification}
  end

  defp handle_deleted_externally(path, content, :phase_owned) do
    atomic_write!(path, content)
    :ok = FreshnessTracker.record(path, content)

    emit_diagnostic(
      :warn,
      "file #{path} deleted externally; recreated by phase cycle",
      %{path: path, category: :phase_owned, kind: :recreate}
    )

    :ok
  end

  defp handle_deleted_externally(path, _content, :human_editable) do
    emit_diagnostic(
      :warn,
      "file #{path} deleted externally; daemon write skipped",
      %{path: path, category: :human_editable, kind: :delete_skip}
    )

    {:conflict, :external_modification}
  end

  defp rebaseline_hash(path) do
    case File.read(path) do
      {:ok, current} -> :ok = FreshnessTracker.record(path, current)
      {:error, _} -> :ok
    end
  end

  defp atomic_write!(path, content) do
    dir = Path.dirname(path)
    File.mkdir_p!(dir)

    tmp_path =
      Path.join(
        dir,
        ".ravel.#{Path.basename(path)}.tmp.#{System.unique_integer([:positive])}"
      )

    try do
      File.write!(tmp_path, content)
      File.rename!(tmp_path, path)
    rescue
      e ->
        _ = File.rm(tmp_path)
        reraise e, __STACKTRACE__
    end
  end

  # Diagnostic emission boundary. Sub-M will eventually subscribe to a
  # `[:ravel, :coordination, :*]` telemetry stream per spec §3.4; for
  # now we log structured metadata. Swapping to `:telemetry.execute/3`
  # is a single-site change when sub-M lands.
  defp emit_diagnostic(level, message, metadata) do
    tagged = Map.put(metadata, :target, "ravel.coordination")

    case level do
      :warn -> Logger.warning(message, Map.to_list(tagged))
      :info -> Logger.info(message, Map.to_list(tagged))
    end
  end
end
