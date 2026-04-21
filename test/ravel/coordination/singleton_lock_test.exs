defmodule Ravel.Coordination.SingletonLockTest do
  use ExUnit.Case, async: false

  import ExUnit.CaptureLog

  alias Ravel.Coordination.SingletonLock

  setup do
    lock_dir =
      Path.join(
        System.tmp_dir!(),
        "ravel_singleton_lock_test_#{System.unique_integer([:positive])}"
      )

    on_exit(fn -> File.rm_rf!(lock_dir) end)

    %{lock_dir: lock_dir, vault_path: "/tmp/fake-vault-#{System.unique_integer([:positive])}"}
  end

  describe "acquire/2 — success" do
    test "acquires the lock and writes PID + vault path", %{
      lock_dir: lock_dir,
      vault_path: vault_path
    } do
      capture_log(fn ->
        assert {:ok, port} = SingletonLock.acquire(vault_path, lock_dir: lock_dir)
        on_exit(fn -> SingletonLock.release(port) end)

        assert is_port(port)

        lock_path = Path.join(lock_dir, "daemon.lock")
        assert File.exists?(lock_path)

        content = File.read!(lock_path)
        assert content == "#{System.pid()}\n#{vault_path}\n"
      end)
    end

    test "emits a lock_acquired diagnostic", %{lock_dir: lock_dir, vault_path: vault_path} do
      log =
        capture_log(fn ->
          assert {:ok, port} = SingletonLock.acquire(vault_path, lock_dir: lock_dir)
          on_exit(fn -> SingletonLock.release(port) end)
        end)

      assert log =~ "ravel.lock.lock_acquired"
      assert log =~ "[info]"
    end

    test "creates the lock directory if absent", %{lock_dir: lock_dir, vault_path: vault_path} do
      refute File.exists?(lock_dir)

      capture_log(fn ->
        assert {:ok, port} = SingletonLock.acquire(vault_path, lock_dir: lock_dir)
        on_exit(fn -> SingletonLock.release(port) end)
      end)

      assert File.dir?(lock_dir)
    end
  end

  describe "acquire/2 — contention" do
    test "second acquire returns {:error, {:daemon_running, pid, vault_path}}", %{
      lock_dir: lock_dir,
      vault_path: vault_path
    } do
      capture_log(fn ->
        {:ok, first} = SingletonLock.acquire(vault_path, lock_dir: lock_dir)
        on_exit(fn -> SingletonLock.release(first) end)

        assert {:error, {:daemon_running, pid, incumbent_vault}} =
                 SingletonLock.acquire("/tmp/other-vault", lock_dir: lock_dir)

        assert pid == String.to_integer(System.pid())
        assert incumbent_vault == vault_path
      end)
    end

    test "contention emits a lock_contended diagnostic with incumbent info", %{
      lock_dir: lock_dir,
      vault_path: vault_path
    } do
      capture_log(fn ->
        {:ok, first} = SingletonLock.acquire(vault_path, lock_dir: lock_dir)
        on_exit(fn -> SingletonLock.release(first) end)
      end)

      log =
        capture_log(fn ->
          assert {:error, {:daemon_running, _, _}} =
                   SingletonLock.acquire("/tmp/other", lock_dir: lock_dir)
        end)

      assert log =~ "ravel.lock.lock_contended"
      assert log =~ "[warning]"
    end
  end

  describe "release/1 — graceful shutdown" do
    test "releases the lock so a second acquire succeeds", %{
      lock_dir: lock_dir,
      vault_path: vault_path
    } do
      capture_log(fn ->
        {:ok, first} = SingletonLock.acquire(vault_path, lock_dir: lock_dir)
        :ok = SingletonLock.release(first)

        wait_for_lock_release(lock_dir)

        assert {:ok, second} = SingletonLock.acquire(vault_path, lock_dir: lock_dir)
        on_exit(fn -> SingletonLock.release(second) end)
      end)
    end

    test "emits a lock_released diagnostic", %{lock_dir: lock_dir, vault_path: vault_path} do
      capture_log(fn ->
        {:ok, port} = SingletonLock.acquire(vault_path, lock_dir: lock_dir)

        log = capture_log(fn -> :ok = SingletonLock.release(port) end)

        assert log =~ "ravel.lock.lock_released"
        assert log =~ "[info]"
      end)
    end

    test "release/1 on an already-closed port is idempotent", %{
      lock_dir: lock_dir,
      vault_path: vault_path
    } do
      capture_log(fn ->
        {:ok, port} = SingletonLock.acquire(vault_path, lock_dir: lock_dir)
        :ok = SingletonLock.release(port)
        assert :ok = SingletonLock.release(port)
      end)
    end
  end

  describe "acquire/2 — crash recovery (kernel releases flock on process death)" do
    test "second acquire succeeds after the holder's OS process tree is SIGKILL'd", %{
      lock_dir: lock_dir,
      vault_path: vault_path
    } do
      capture_log(fn ->
        {:ok, first} = SingletonLock.acquire(vault_path, lock_dir: lock_dir)

        flock_pid = Port.info(first, :os_pid) |> elem(1)
        kill_process_tree(flock_pid)
        wait_for_lock_release(lock_dir)

        assert {:ok, second} = SingletonLock.acquire(vault_path, lock_dir: lock_dir)
        on_exit(fn -> SingletonLock.release(second) end)
      end)
    end
  end

  describe "acquire/2 — missing flock" do
    test "returns {:error, {:flock_missing, _}} when flock is not on PATH", %{
      lock_dir: lock_dir,
      vault_path: vault_path
    } do
      original_path = System.get_env("PATH")
      System.put_env("PATH", "/nonexistent")

      try do
        capture_log(fn ->
          assert {:error, {:flock_missing, reason}} =
                   SingletonLock.acquire(vault_path, lock_dir: lock_dir)

          assert reason =~ "flock"
        end)
      after
        System.put_env("PATH", original_path)
      end
    end
  end

  describe "lock_path/1" do
    test "returns <lock_dir>/daemon.lock", %{lock_dir: lock_dir} do
      assert SingletonLock.lock_path(lock_dir: lock_dir) == Path.join(lock_dir, "daemon.lock")
    end
  end

  defp wait_for_lock_release(lock_dir) do
    path = Path.join(lock_dir, "daemon.lock")

    Enum.reduce_while(1..40, nil, fn _, _ ->
      case System.cmd("flock", ["-xn", path, "-c", "true"], stderr_to_stdout: true) do
        {_, 0} -> {:halt, :ok}
        {_, _} -> Process.sleep(25) && {:cont, :waiting}
      end
    end)
  end

  defp kill_process_tree(pid) when is_integer(pid) do
    _ = System.cmd("pkill", ["-9", "-P", "#{pid}"], stderr_to_stdout: true)
    _ = System.cmd("kill", ["-9", "#{pid}"], stderr_to_stdout: true)
  end
end
