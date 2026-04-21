defmodule Ravel.Coordination.FreshnessTracker do
  @moduledoc """
  Tracks SHA-256 content hashes of daemon-read files so that subsequent
  writes can detect external modifications. Backs `Ravel.FileIO`'s
  read-before-write discipline per sub-D spec §3.2.

  The GenServer owns a named, public ETS table keyed by absolute path.
  Reads go directly through `:ets.lookup/2` without message-passing;
  writes go through the GenServer so table updates are serialized and
  survive individual actor crashes. Table contents do not survive a
  daemon restart — that is fine, every boot re-reads the vault.
  """

  use GenServer

  @table :ravel_freshness

  @type hash :: binary()

  @spec start_link(keyword()) :: GenServer.on_start()
  def start_link(opts \\ []) do
    GenServer.start_link(__MODULE__, :ok, Keyword.put_new(opts, :name, __MODULE__))
  end

  @doc """
  Record the SHA-256 hash of `content` for `path`. Called after every
  daemon file read so later writes have a baseline to compare against.
  """
  @spec record(Path.t(), binary()) :: :ok
  def record(path, content) when is_binary(path) and is_binary(content) do
    GenServer.call(__MODULE__, {:record, path, :crypto.hash(:sha256, content)})
  end

  @doc """
  Compare `path`'s current on-disk content hash against the recorded hash.

    * `:unchanged` — file exists, hash matches the record.
    * `{:changed, current_hash}` — file exists, hash differs from the record.
    * `:untracked` — no record for this path. Caller decides what to do
      (typically: proceed with the write and record afterward).
    * `{:error, reason}` — file missing or unreadable. `:enoent` covers the
      "deleted externally" case that `FileIO.write_safe/3` handles per spec §3.3.
  """
  @spec check(Path.t()) ::
          :unchanged
          | {:changed, hash()}
          | :untracked
          | {:error, File.posix()}
  def check(path) when is_binary(path) do
    case :ets.lookup(@table, path) do
      [] ->
        :untracked

      [{^path, stored_hash}] ->
        case File.read(path) do
          {:ok, content} ->
            current_hash = :crypto.hash(:sha256, content)
            if current_hash == stored_hash, do: :unchanged, else: {:changed, current_hash}

          {:error, reason} ->
            {:error, reason}
        end
    end
  end

  @impl true
  def init(:ok) do
    :ets.new(@table, [:named_table, :public, read_concurrency: true])
    {:ok, nil}
  end

  @impl true
  def handle_call({:record, path, hash}, _from, state) do
    :ets.insert(@table, {path, hash})
    {:reply, :ok, state}
  end
end
