defmodule Ravel.VaultGit do
  @moduledoc """
  Sequential wrapper for daemon-triggered vault git operations. All calls
  are human-mediated in v1 per sub-D spec §4.1 — the daemon never commits
  autonomously. `commit/3` is invoked only when a user explicitly approves
  an action (e.g. accepting a routing-rule suggestion in the TUI).

  The module is deliberately thin: a `git add` followed by `git commit`
  via `System.cmd/3`, with one structured failure code for `index.lock`
  contention and a generic failure tuple for everything else. No retry —
  the spec (§4.2) requires the user to retry explicitly if git's own
  lock was held.
  """

  require Logger

  @type failure_reason ::
          :index_lock_held
          | {:git_failed, output :: String.t()}

  @spec commit(Path.t(), [Path.t()], String.t()) :: :ok | {:error, failure_reason()}
  def commit(vault_path, paths, message)
      when is_binary(vault_path) and is_list(paths) and is_binary(message) do
    with :ok <- run_git(vault_path, ["add" | paths]),
         :ok <- run_git(vault_path, ["commit", "-m", message]) do
      :ok
    end
  end

  defp run_git(cwd, args) do
    case System.cmd("git", args, cd: cwd, stderr_to_stdout: true) do
      {_output, 0} ->
        :ok

      {output, _code} ->
        classify_git_failure(args, output)
    end
  end

  defp classify_git_failure(args, output) do
    if String.contains?(output, "index.lock") do
      emit_error("git operation failed: index.lock held", %{args: args})
      {:error, :index_lock_held}
    else
      emit_error("git operation failed: #{String.trim(output)}", %{args: args, output: output})
      {:error, {:git_failed, output}}
    end
  end

  defp emit_error(message, metadata) do
    Logger.error(message, Map.to_list(Map.put(metadata, :target, "ravel.coordination")))
  end
end
