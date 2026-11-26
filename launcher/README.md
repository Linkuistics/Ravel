# Ravel

Ravel runs LLM sessions in agent harnesses, such as Claude Code and Codex, as
supervised parts of another program. A caller, such as an orchestrator, asks
for work by kind. The Rhai configuration script of the owner chooses the command to
run, which is normally a harness. Ravel runs that command under supervision.
After cleanup succeeds, Ravel returns the result that the session sent.

A harness can run under the subscription of its user. Thus, a caller can use
that subscription for LLM work instead of paid API access. The cost can be much
lower for work that the subscription covers.

An interactive harness waits for more input when the LLM completes its task.
Thus, the LLM runs `ravel end-session` to send its result and ask Ravel to end
the session. This lets an interactive harness work as one part of the process
of the orchestrator.

The contract is loose. The configuration script selects a program, Ravel runs it, and a
program in the session sends one value with `end-session`. Ravel needs nothing
more. The book expects a harness, but any program that runs `end-session` can
stand where a harness stands.

The Ravel CLI embeds Rhai and starts a second copy of itself to evaluate the
configuration script. Its local runner supervises the selected process. These
parts are built and versioned together as one tool.

Ravel starts and ends sessions with `ravel start-session` and
`ravel end-session RESULT`. The command `ravel skills resolve` prints the skill directories that a
session receives. Ravel 2.0.0 adds the directories of the owner's local skill
sources to the prompt of each session, and the agent reads the skills that
apply.

Homebrew installs Ravel with `brew install linkuistics/taps/ravel`.
[Chapter 3](docs/book/src/03-a-complete-session.md) gives the complete
installation command and explains each part. The versioned formula `ravel@2`
served the transition to 2.0.0, and Homebrew now marks it as deprecated.

The book is the guide to Ravel. It says what Ravel is for, how to write a
configuration script and how to run sessions. It also explains the parts of Ravel, how
to develop it and how to deliver it.

The book is in this repository checkout under `docs/book/`. A release archive
does not have it. Each release puts its book on
[GitHub Pages](https://linkuistics.github.io/Ravel/).

To read the book from a checkout, run `task setup` one time. Then
run `task docs:book` to build the book and serve it on localhost. You can also
read the Markdown in `docs/book/src/`. Start at
[chapter 1](docs/book/src/01-what-ravel-is-for.md).

These chapters explain Ravel from the inside, and how to change and release it:

- [Chapter 7, The parts of Ravel](docs/book/src/07-the-parts-of-ravel.md)
- [Chapter 8, Follow a session through the code](docs/book/src/08-follow-a-session-through-the-code.md)
- [Chapter 9, Jobs, signals and the terminal](docs/book/src/09-jobs-signals-and-the-terminal.md)
- [Chapter 10, Develop Ravel](docs/book/src/10-develop-ravel.md)
- [Chapter 11, Deliver Ravel](docs/book/src/11-deliver-ravel.md)

Ravel supports no separate component. Its runner, `keyed-launch`, is a private
package of the Ravel CLI, which supervises one session and then exits. The
runner can change process-wide signal state, and its return does not restore
that state. A program that needs supervised harness sessions invokes the Ravel
CLI as a child process.

An installation contains `bin/ravel`, three Rhai examples in
`share/ravel/examples/`, and Rust dependency notices at
`share/ravel/notices/THIRD-PARTY-NOTICES.md`. From a checkout, `task notices`
generates those notices at `target/notices/THIRD-PARTY-NOTICES.md`.
The release carries no JavaScript runtime or SDK.

## Retired project

The former actor-daemon project is preserved in this repository under
`retired/actor-daemon-2026-10-12`. The preserved project includes the README edits of the owner.
[Retrieval instructions and the exact revision](docs/retired-project.md) remain
part of this tree. Its daemon sources, design documents and agent-state files
are retired history.

Licensed under [Apache-2.0](LICENSE).
