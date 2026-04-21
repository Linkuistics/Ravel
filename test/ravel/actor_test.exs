defmodule Ravel.ActorTest do
  use ExUnit.Case, async: true

  alias Ravel.Actor

  describe "behaviour definition" do
    test "declares the expected callback set with correct arities" do
      callbacks = Actor.behaviour_info(:callbacks)

      assert {:actor_id, 1} in callbacks
      assert {:actor_type, 1} in callbacks
      assert {:handle_actor_message, 2} in callbacks
      assert {:snapshot, 1} in callbacks
      assert {:restore, 1} in callbacks
    end

    test "callback set is exactly the five shared callbacks — nothing more" do
      callbacks = Actor.behaviour_info(:callbacks)
      assert length(callbacks) == 5
    end

    test "callback set does not constrain sub-N's ExpertRetrieval (retrieve/3) or persona" do
      # The backlog (Task 2) explicitly forbids retrieval or expert-internal
      # callbacks on the shared Actor behaviour so sub-N can supply plug-
      # gable retrieval strategies independently. Locked by sub-N's
      # ExpertRetrieval behaviour signature.
      names = Actor.behaviour_info(:callbacks) |> Enum.map(fn {n, _} -> n end)

      refute :retrieve in names
      refute :persona in names
      refute :knowledge_scope in names
      refute :absorb_candidate in names
    end
  end

  describe "compliant implementations" do
    defmodule TestActor do
      @moduledoc false
      @behaviour Ravel.Actor

      @impl true
      def actor_id(state), do: state.id

      @impl true
      def actor_type(_state), do: :plan

      @impl true
      def handle_actor_message({:dispatch, msg}, state) do
        {:reply, :ack, %{state | inbox: state.inbox ++ [msg]}}
      end

      def handle_actor_message({:query, q}, state) do
        {:reply, {:answer, "answer:#{q}"}, state}
      end

      @impl true
      def snapshot(state), do: state

      @impl true
      def restore(snapshot), do: {:ok, snapshot}
    end

    test "a minimal implementor satisfies actor_id/1 and actor_type/1" do
      state = %{id: "TestProject/project-root", inbox: []}

      assert TestActor.actor_id(state) == "TestProject/project-root"
      assert TestActor.actor_type(state) == :plan
    end

    test "handle_actor_message/2 can return {:reply, :ack, new_state} for Dispatch" do
      state = %{id: "p", inbox: []}
      assert {:reply, :ack, new_state} = TestActor.handle_actor_message({:dispatch, :t}, state)
      assert new_state.inbox == [:t]
    end

    test "handle_actor_message/2 can return {:reply, {:answer, _}, state} for Query" do
      state = %{id: "p", inbox: []}

      assert {:reply, {:answer, "answer:hi"}, ^state} =
               TestActor.handle_actor_message({:query, "hi"}, state)
    end

    test "snapshot/1 and restore/1 round-trip opaque actor state" do
      state = %{id: "p", inbox: [:t1, :t2]}
      snap = TestActor.snapshot(state)
      assert {:ok, restored} = TestActor.restore(snap)
      assert restored == state
    end
  end
end
