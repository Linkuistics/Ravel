defmodule Ravel.Coordination.SingletonLock do
  @moduledoc """
  System-wide singleton lock enforcing the v1 single-daemon-per-machine
  constraint (sub-D spec §2). Uses `flock(2)` via the `flock(1)` CLI so
  the lock is kernel-enforced and auto-released on process death — no
  stale-lock cleanup is required regardless of how the prior daemon
  exited (clean shutdown, SIGKILL, panic, OOM).

  The lock file lives at `<runtime_dir>/ravel/daemon.lock`, outside any
  vault, so Obsidian never sees it. The file's content is the daemon's
  OS PID and the absolute vault path it serves, newline-separated — this
  lets a contending second daemon report a fully-informative error.

  ## Acquisition mechanism

  We spawn `flock -xn <lock_path> cat` as a port-linked child, using
  `:spawn_executable` (no intermediate shell). `flock` opens the file,
  attempts a non-blocking exclusive lock, and on success execs `cat`
  reading from the inherited stdin pipe. The port is held open by the
  daemon: when the daemon exits (any reason), BEAM closes the port's
  pipes, `cat` sees EOF on stdin and exits cleanly, `flock` then exits
  and the kernel releases the lock. This pattern avoids the orphan-
  child problem of `sleep infinity` — `cat` reacts to stdin EOF without
  needing a signal to propagate.

  On contention, `flock` exits immediately with status 1, which surfaces
  as `{:exit_status, 1}` on the port. We then read the lock file to
  report the incumbent PID and vault path.

  ## Required external tool

  `flock(1)` from util-linux. Preinstalled on Linux; on macOS install via
  `brew install flock`. The module hard-errors at `acquire/1` time if
  `flock` is not on `PATH`.
  """

  require Logger

  @type lock_handle :: port()
  @type opts :: [lock_dir: Path.t()]
  @type acquire_result ::
          {:ok, lock_handle()}
          | {:error,
             {:daemon_running, incumbent_pid :: pos_integer() | nil,
              incumbent_vault_path :: String.t() | nil}}
          | {:error, {:flock_missing, String.t()}}

  @acquire_probe_timeout_ms 200

  @spec acquire(Path.t(), opts()) :: acquire_result()
  def acquire(vault_path, opts \\ []) when is_binary(vault_path) do
    dir = resolve_lock_dir(opts)

    with {:ok, flock_bin} <- find_flock(),
         :ok <- File.mkdir_p(dir) do
      do_acquire(flock_bin, Path.join(dir, "daemon.lock"), vault_path)
    end
  end

  @spec release(lock_handle()) :: :ok
  def release(port) when is_port(port) do
    try do
      Port.close(port)
    rescue
      ArgumentError -> :ok
    end

    emit(:info, "lock_released", %{target: "ravel.lock"})
    :ok
  end

  @doc """
  Absolute path of the lock file. Exposed so bootstrap subcommands can
  read the incumbent daemon's PID and vault path without going through
  `acquire/2`.
  """
  @spec lock_path(opts()) :: Path.t()
  def lock_path(opts \\ []), do: Path.join(resolve_lock_dir(opts), "daemon.lock")

  defp resolve_lock_dir(opts) do
    case Keyword.fetch(opts, :lock_dir) do
      {:ok, dir} -> dir
      :error -> default_lock_dir()
    end
  end

  defp default_lock_dir do
    base = System.get_env("XDG_RUNTIME_DIR") || System.tmp_dir!()
    Path.join(base, "ravel")
  end

  defp find_flock do
    case System.find_executable("flock") do
      nil ->
        emit(:error, "lock_flock_missing", %{target: "ravel.lock"})

        {:error,
         {:flock_missing,
          "flock(1) not on PATH (install util-linux flock; macOS: brew install flock)"}}

      path ->
        {:ok, path}
    end
  end

  defp do_acquire(flock_bin, lock_path, vault_path) do
    port =
      Port.open(
        {:spawn_executable, flock_bin},
        [:binary, :exit_status, {:args, ["-xn", lock_path, "cat"]}]
      )

    case await_verdict(port, @acquire_probe_timeout_ms) do
      :acquired ->
        daemon_pid = System.pid()
        File.write!(lock_path, "#{daemon_pid}\n#{vault_path}\n")

        emit(:info, "lock_acquired", %{
          target: "ravel.lock",
          vault_path: vault_path,
          pid: daemon_pid
        })

        {:ok, port}

      {:contended, _status} ->
        {incumbent_pid, incumbent_vault} = read_incumbent(lock_path)

        emit(:warn, "lock_contended", %{
          target: "ravel.lock",
          incumbent_pid: incumbent_pid,
          incumbent_vault_path: incumbent_vault
        })

        {:error, {:daemon_running, incumbent_pid, incumbent_vault}}
    end
  end

  defp await_verdict(port, timeout_ms) do
    receive do
      {^port, {:exit_status, status}} -> {:contended, status}
    after
      timeout_ms -> :acquired
    end
  end

  defp read_incumbent(lock_path) do
    case File.read(lock_path) do
      {:ok, content} -> parse_lock_content(content)
      {:error, _} -> {nil, nil}
    end
  end

  defp parse_lock_content(content) do
    case String.split(content, "\n", trim: true) do
      [pid_str, vault_path | _] ->
        {parse_pid(pid_str), vault_path}

      [pid_str] ->
        {parse_pid(pid_str), nil}

      [] ->
        {nil, nil}
    end
  end

  defp parse_pid(pid_str) do
    case Integer.parse(pid_str) do
      {n, _} when n > 0 -> n
      _ -> nil
    end
  end

  defp emit(level, event, metadata) do
    message = "ravel.lock.#{event}"

    case level do
      :info -> Logger.info(message, Map.to_list(metadata))
      :warn -> Logger.warning(message, Map.to_list(metadata))
      :error -> Logger.error(message, Map.to_list(metadata))
    end
  end
end
