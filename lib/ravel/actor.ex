defmodule Ravel.Actor do
  @moduledoc """
  Shared behaviour for both actor types (`Ravel.PlanActor` and
  `Ravel.ExpertActor`) per sub-F spec §4.1.

  Every actor is a `GenServer` that additionally implements this
  behaviour so the router, supervisor, and dispatch processors can
  reason about plans and experts uniformly without caring which kind
  they are dealing with.

  The callback set is deliberately minimal — anything specific to plans
  (phase cycle, backlog file I/O, harness session management) or to
  experts (persona rendering, knowledge retrieval, scope matching)
  belongs in the implementation modules. In particular, sub-N's
  `Ravel.ExpertRetrieval` behaviour (`retrieve/3`) is independent of
  this one; adding a retrieval callback here would force every plan
  actor to carry a retrieval strategy it does not use.

  Concrete message and response structs (`Ravel.Message.Dispatch`,
  `Ravel.Message.Query`, `Ravel.Message.QueryAnswer`,
  `Ravel.RejectionReason`, `Ravel.ActorSnapshot`) land in later sub-F
  tasks; for now the type aliases below are intentionally wide so
  callers can compile against the behaviour before those structs
  exist.
  """

  @typedoc "Logical identifier. Plans: path-based qualified ID (F-5). Experts: filename stem."
  @type actor_id :: String.t()

  @type actor_type :: :plan | :expert

  # The concrete message/response structs are placeholders until the
  # corresponding modules land. Narrow these when they do — every
  # implementor will re-verify against the new shapes at that point.
  @type dispatch_message :: term()
  @type query_message :: term()
  @type query_answer :: term()
  @type rejection_reason :: term()
  @type actor_snapshot :: term()

  @type message :: {:dispatch, dispatch_message()} | {:query, query_message()}

  @type response ::
          :ack
          | {:answer, query_answer()}
          | {:rejected, rejection_reason()}

  @doc "Return this actor's qualified ID from its current state."
  @callback actor_id(state :: term()) :: actor_id()

  @doc "Return this actor's type (`:plan` or `:expert`)."
  @callback actor_type(state :: term()) :: actor_type()

  @doc """
  Handle a Dispatch or Query received via the router.

  Returns `{:reply, response, new_state}` when the actor has something
  to say back to the router (the common case: `:ack` for Dispatch,
  `{:answer, _}` or `{:rejected, _}` for Query), or `{:noreply, new_state}`
  when the response will be produced asynchronously via the harness
  tool-call boundary (sub-C).
  """
  @callback handle_actor_message(message(), state :: term()) ::
              {:reply, response(), new_state :: term()}
              | {:noreply, new_state :: term()}

  @doc """
  Produce a serialisable snapshot of actor state for crash recovery.

  Actors are expected to keep their snapshot in sync with filesystem
  state per spec §4.4 — the snapshot is a convenience, not the
  authoritative store. The filesystem is the durable substrate.
  """
  @callback snapshot(state :: term()) :: actor_snapshot()

  @doc "Rehydrate actor state from a snapshot produced by `snapshot/1`."
  @callback restore(actor_snapshot()) :: {:ok, state :: term()}
end
