defmodule Ravel.FileIOTest do
  use ExUnit.Case, async: false

  import ExUnit.CaptureLog

  alias Ravel.Coordination.FreshnessTracker
  alias Ravel.FileIO

  setup do
    start_supervised!(FreshnessTracker)

    tmp_dir =
      Path.join(
        System.tmp_dir!(),
        "ravel_file_io_test_#{System.unique_integer([:positive])}"
      )

    File.mkdir_p!(tmp_dir)
    on_exit(fn -> File.rm_rf!(tmp_dir) end)

    %{tmp_dir: tmp_dir}
  end

  describe "read_tracked/1" do
    test "reads file content and records its hash", %{tmp_dir: dir} do
      path = Path.join(dir, "src.md")
      File.write!(path, "hello")

      assert {:ok, "hello"} = FileIO.read_tracked(path)
      assert FreshnessTracker.check(path) == :unchanged
    end

    test "returns {:error, :enoent} for a missing file without recording", %{tmp_dir: dir} do
      path = Path.join(dir, "missing.md")

      assert {:error, :enoent} = FileIO.read_tracked(path)
      assert FreshnessTracker.check(path) == :untracked
    end
  end

  describe "write_safe/3 :exclusive" do
    test "writes without freshness check even after external modification", %{tmp_dir: dir} do
      path = Path.join(dir, "plan-catalog.md")
      File.write!(path, "old")
      :ok = FreshnessTracker.record(path, "old")

      File.write!(path, "externally-changed")

      log = capture_log(fn -> assert :ok = FileIO.write_safe(path, "new", :exclusive) end)
      assert log == ""

      assert File.read!(path) == "new"
      assert FreshnessTracker.check(path) == :unchanged
    end

    test "creates parent directories as needed", %{tmp_dir: dir} do
      path = Path.join([dir, "nested", "runtime", "staging.bin"])

      assert :ok = FileIO.write_safe(path, "payload", :exclusive)
      assert File.read!(path) == "payload"
    end
  end

  describe "write_safe/3 :phase_owned" do
    test "writes with no diagnostic when file is unchanged since last read", %{tmp_dir: dir} do
      path = Path.join(dir, "memory.md")
      File.write!(path, "baseline")
      {:ok, _} = FileIO.read_tracked(path)

      log = capture_log(fn -> assert :ok = FileIO.write_safe(path, "updated", :phase_owned) end)
      assert log == ""

      assert File.read!(path) == "updated"
    end

    test "overwrites external modification and emits a warn diagnostic", %{tmp_dir: dir} do
      path = Path.join(dir, "memory.md")
      File.write!(path, "baseline")
      {:ok, _} = FileIO.read_tracked(path)

      File.write!(path, "human-edited")

      log =
        capture_log(fn -> assert :ok = FileIO.write_safe(path, "phase-update", :phase_owned) end)

      assert log =~ "external modification"
      assert log =~ "overwritten by phase cycle"
      assert log =~ "[warning]"

      assert File.read!(path) == "phase-update"
    end

    test "recreates a file that was deleted externally and emits a warn diagnostic",
         %{tmp_dir: dir} do
      path = Path.join(dir, "session-log.md")
      File.write!(path, "prior")
      {:ok, _} = FileIO.read_tracked(path)

      File.rm!(path)

      log = capture_log(fn -> assert :ok = FileIO.write_safe(path, "recreated", :phase_owned) end)

      assert log =~ "deleted externally"
      assert log =~ "recreated by phase cycle"
      assert log =~ "[warning]"

      assert File.read!(path) == "recreated"
    end

    test "proceeds when path is untracked (never read-then-written)", %{tmp_dir: dir} do
      path = Path.join(dir, "fresh-memory.md")

      log = capture_log(fn -> assert :ok = FileIO.write_safe(path, "content", :phase_owned) end)
      assert log == ""

      assert File.read!(path) == "content"
    end
  end

  describe "write_safe/3 :human_editable" do
    test "writes with no diagnostic when file is unchanged since last read", %{tmp_dir: dir} do
      path = Path.join(dir, "routing.ex")
      File.write!(path, "defmodule R, do: nil")
      {:ok, _} = FileIO.read_tracked(path)

      log =
        capture_log(fn -> assert :ok = FileIO.write_safe(path, "new rules", :human_editable) end)

      assert log == ""

      assert File.read!(path) == "new rules"
    end

    test "discards write on external modification and emits an info diagnostic",
         %{tmp_dir: dir} do
      path = Path.join(dir, "knowledge.md")
      File.write!(path, "baseline")
      {:ok, _} = FileIO.read_tracked(path)

      File.write!(path, "human-edited")

      log =
        capture_log(fn ->
          assert {:conflict, :external_modification} =
                   FileIO.write_safe(path, "daemon-version", :human_editable)
        end)

      assert log =~ "external modification"
      assert log =~ "daemon write discarded"
      assert log =~ "[info]"

      assert File.read!(path) == "human-edited",
             "human's version must remain authoritative"
    end

    test "rebaselines hash so a second call does not re-flag the same external version",
         %{tmp_dir: dir} do
      path = Path.join(dir, "knowledge.md")
      File.write!(path, "baseline")
      {:ok, _} = FileIO.read_tracked(path)

      File.write!(path, "human-edited")

      capture_log(fn ->
        assert {:conflict, :external_modification} =
                 FileIO.write_safe(path, "d1", :human_editable)
      end)

      second_log =
        capture_log(fn ->
          assert :ok = FileIO.write_safe(path, "d2", :human_editable)
        end)

      assert second_log == "",
             "after rebaseline, a write with the now-current baseline must proceed cleanly"

      assert File.read!(path) == "d2"
    end

    test "skips write on external deletion and emits a warn diagnostic", %{tmp_dir: dir} do
      path = Path.join(dir, "knowledge.md")
      File.write!(path, "baseline")
      {:ok, _} = FileIO.read_tracked(path)

      File.rm!(path)

      log =
        capture_log(fn ->
          assert {:conflict, :external_modification} =
                   FileIO.write_safe(path, "daemon-version", :human_editable)
        end)

      assert log =~ "deleted externally"
      assert log =~ "daemon write skipped"
      assert log =~ "[warning]"

      refute File.exists?(path), "file must remain absent"
    end

    test "proceeds when path is untracked (first ingestion write)", %{tmp_dir: dir} do
      path = Path.join(dir, "new-knowledge.md")

      log =
        capture_log(fn -> assert :ok = FileIO.write_safe(path, "content", :human_editable) end)

      assert log == ""

      assert File.read!(path) == "content"
    end
  end

  describe "atomic write safety" do
    test "target file is untouched when the underlying write fails mid-operation",
         %{tmp_dir: dir} do
      path = Path.join(dir, "target.md")
      File.write!(path, "original")
      {:ok, _} = FileIO.read_tracked(path)

      File.chmod!(dir, 0o555)

      try do
        assert_raise File.Error, fn ->
          FileIO.write_safe(path, "new", :phase_owned)
        end
      after
        File.chmod!(dir, 0o755)
      end

      assert File.read!(path) == "original",
             "failed atomic write must leave the target at its prior content"
    end

    test "no partial-content file is left at the target path after a rename failure",
         %{tmp_dir: dir} do
      path = Path.join(dir, "target.md")
      File.write!(path, "original")
      {:ok, _} = FileIO.read_tracked(path)

      File.chmod!(dir, 0o555)

      try do
        assert_raise File.Error, fn -> FileIO.write_safe(path, "new", :phase_owned) end
      after
        File.chmod!(dir, 0o755)
      end

      entries = File.ls!(dir)

      refute Enum.any?(entries, &String.starts_with?(&1, ".ravel.target.md.tmp.")),
             "stale tmp files in the target dir indicate a cleanup-path bug"
    end

    test "repeated writes produce a file whose content is always one full, complete version",
         %{tmp_dir: dir} do
      path = Path.join(dir, "atomic.md")
      [first | _] = versions = for i <- 1..50, do: String.duplicate("v#{i}\n", 100)

      File.write!(path, first)
      {:ok, _} = FileIO.read_tracked(path)

      for version <- versions do
        :ok = FileIO.write_safe(path, version, :exclusive)
        observed = File.read!(path)
        assert observed in versions, "observed partial or corrupt content: #{inspect(observed)}"
      end
    end
  end
end
