defmodule Ravel.Coordination.FreshnessTrackerTest do
  use ExUnit.Case, async: false

  alias Ravel.Coordination.FreshnessTracker

  setup do
    start_supervised!(FreshnessTracker)

    tmp_dir =
      Path.join(
        System.tmp_dir!(),
        "ravel_freshness_test_#{System.unique_integer([:positive])}"
      )

    File.mkdir_p!(tmp_dir)
    on_exit(fn -> File.rm_rf!(tmp_dir) end)

    %{tmp_dir: tmp_dir}
  end

  describe "check/1" do
    test "returns :untracked for a path that was never recorded", %{tmp_dir: dir} do
      path = Path.join(dir, "never_seen.txt")
      File.write!(path, "hello")

      assert FreshnessTracker.check(path) == :untracked
    end

    test "returns :unchanged when the file still matches its recorded hash", %{tmp_dir: dir} do
      path = Path.join(dir, "stable.txt")
      content = "hello world"
      File.write!(path, content)
      :ok = FreshnessTracker.record(path, content)

      assert FreshnessTracker.check(path) == :unchanged
    end

    test "returns {:changed, current_hash} when the file was modified externally", %{tmp_dir: dir} do
      path = Path.join(dir, "edited.txt")
      File.write!(path, "original")
      :ok = FreshnessTracker.record(path, "original")

      File.write!(path, "modified")

      assert {:changed, current_hash} = FreshnessTracker.check(path)
      assert current_hash == :crypto.hash(:sha256, "modified")
    end

    test "returns {:error, :enoent} when a tracked file was deleted", %{tmp_dir: dir} do
      path = Path.join(dir, "gone.txt")
      File.write!(path, "bye")
      :ok = FreshnessTracker.record(path, "bye")

      File.rm!(path)

      assert FreshnessTracker.check(path) == {:error, :enoent}
    end
  end

  describe "record/2" do
    test "a second record/2 call for the same path replaces the prior hash", %{tmp_dir: dir} do
      path = Path.join(dir, "updated.txt")
      File.write!(path, "v1")
      :ok = FreshnessTracker.record(path, "v1")

      File.write!(path, "v2")
      :ok = FreshnessTracker.record(path, "v2")

      assert FreshnessTracker.check(path) == :unchanged
    end
  end
end
