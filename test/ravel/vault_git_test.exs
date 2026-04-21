defmodule Ravel.VaultGitTest do
  use ExUnit.Case, async: false

  import ExUnit.CaptureLog

  alias Ravel.VaultGit

  setup do
    tmp_dir =
      Path.join(
        System.tmp_dir!(),
        "ravel_vault_git_test_#{System.unique_integer([:positive])}"
      )

    File.mkdir_p!(tmp_dir)

    git!(tmp_dir, ["init", "--quiet", "--initial-branch=main"])
    git!(tmp_dir, ["config", "user.email", "ravel-test@example.com"])
    git!(tmp_dir, ["config", "user.name", "Ravel Test"])
    git!(tmp_dir, ["config", "commit.gpgsign", "false"])

    File.write!(Path.join(tmp_dir, "seed.txt"), "seed\n")
    git!(tmp_dir, ["add", "seed.txt"])
    git!(tmp_dir, ["commit", "--quiet", "-m", "seed"])

    on_exit(fn -> File.rm_rf!(tmp_dir) end)

    %{repo: tmp_dir}
  end

  describe "commit/3" do
    test "stages and commits the given paths on success", %{repo: repo} do
      path = Path.join(repo, "routing.ex")
      File.write!(path, "defmodule Ravel.UserRouting do\nend\n")

      assert :ok = VaultGit.commit(repo, ["routing.ex"], "add routing rule")

      {log, 0} = System.cmd("git", ["log", "--format=%s", "-1"], cd: repo)
      assert String.trim(log) == "add routing rule"

      {status, 0} = System.cmd("git", ["status", "--porcelain"], cd: repo)
      assert status == ""
    end

    test "returns {:error, :index_lock_held} when .git/index.lock is held", %{repo: repo} do
      lock_path = Path.join([repo, ".git", "index.lock"])
      File.write!(lock_path, "")

      path = Path.join(repo, "routing.ex")
      File.write!(path, "x\n")

      log =
        capture_log(fn ->
          assert {:error, :index_lock_held} =
                   VaultGit.commit(repo, ["routing.ex"], "should fail")
        end)

      assert log =~ "index.lock held"

      {log_out, 0} = System.cmd("git", ["log", "--format=%s"], cd: repo)
      refute String.contains?(log_out, "should fail")
    after
      _ = File.rm(Path.join([repo, ".git", "index.lock"]))
    end

    test "returns {:error, {:git_failed, _}} on other git failures", %{repo: repo} do
      assert {:error, {:git_failed, output}} =
               VaultGit.commit(repo, ["does-not-exist.txt"], "will fail")

      assert is_binary(output)
      assert String.length(output) > 0
    end

    test "does not retry after a failure", %{repo: repo} do
      lock_path = Path.join([repo, ".git", "index.lock"])
      File.write!(lock_path, "")

      path = Path.join(repo, "a.txt")
      File.write!(path, "a\n")

      capture_log(fn ->
        assert {:error, :index_lock_held} = VaultGit.commit(repo, ["a.txt"], "msg")
      end)

      assert File.exists?(lock_path), "lock untouched (no retry attempted)"
    after
      _ = File.rm(Path.join([repo, ".git", "index.lock"]))
    end
  end

  defp git!(cwd, args) do
    {_, 0} = System.cmd("git", args, cd: cwd, stderr_to_stdout: true)
    :ok
  end
end
