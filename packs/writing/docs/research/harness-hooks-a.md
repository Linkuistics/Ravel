# What each harness allows for an always-on skill

## Scope and confidence

This survey asks what Claude Code, Codex, Pi and Gemini CLI each offer for
making one skill, `technical-communication`, **always on**: an instruction at
session start, a short reminder before each prompt, a check that runs after the
model writes a prose file and hands its findings back, a log record for every
reply, and one switch that turns all of it off. The fallback it must beat is
the skill telling the model to run the checker itself.

**Every claim names the version it is about.** The versions of record are the
ones installed here on 2026-10-18, except Gemini CLI, which is not installed:

| Harness | Version of record | How it was read |
|---|---|---|
| Claude Code | 2.1.302 (native binary, `~/.local/share/claude/versions/2.1.302`) | Strings in the binary's embedded bundle, the current docs (whose changelog's newest entry is 2.1.302), this session's own transcript files |
| Codex | codex-cli 0.163.0 (Homebrew cask) | Source at `openai/codex` tag `rust-v0.163.0` (commit `83086188ae06`), the binary, probes against a scratch `CODEX_HOME` that made no model call |
| Pi | 0.84.2 (`@earendil-works/pi-coding-agent`, first on `PATH`) | The installed package's `dist/` source, `docs/`, `examples/` and `CHANGELOG.md` |
| Gemini CLI | 0.64.0 (npm `latest`, released 2026-10-13) | Source at `google-gemini/gemini-cli` tag `v0.64.0` (commit `4a0749aceac5`) and the packed release, never run |

**No model session of any harness was started.** What a hook *does* in a live
turn is therefore known from source and docs, not from a run; the always-on
leaves run each mechanism for real. Where the source and the docs disagree, the
source for the version of record wins and the disagreement is stated.

**Evidence classes**, marked on claims that carry a design decision:
**[S]** read in source or in the shipped bundle; **[L]** observed on the
installed version without a model call (a probe, this session's transcript);
**[D]** official documentation; **[R]** an issue, release note or third-party
report; **[I]** inference, with its basis named. An issue's state is given
where it matters. On Pi's tracker, many issues read "closed" because a bot
closes issues from new contributors, so *closed* there does not mean *fixed*.

**Citation forms.** Claude Code is closed source: a citation in quotes is a
string that `grep -a` finds in the 2.1.302 binary, and doc citations name a
page under `https://code.claude.com/docs/en/` and its section. Codex
citations are paths under `codex-rs/` at `rust-v0.163.0`. Pi citations are
paths inside the 0.84.2 package. Gemini citations are paths at `v0.64.0`, or
`bundle/docs/…` for the documentation shipped in that release. Issues are
`owner/repo#N`.

**The corpus is biased toward failure**, as the brief asked: hooks that slow
every turn, loop, fail silently, change shape between versions, or are
blocked by sandboxes and trust prompts.

## Findings at a glance

1. **Only on Claude Code does the route that delivers skills today already
   deliver hooks.** A `hooks/hooks.json` in the `linkuistics` plugin fires
   from session load in every session where the plugin is enabled, with no
   prompt. Auto-update delivers new hooks to the next session, to every user
   of the plugin. Pi also loads code with no prompt, but only from a link that
   nothing installs today.

2. **Codex has all the events, but no hook runs until a person trusts it.**
   Trust is recorded per handler as a hash of its normalised configuration,
   keyed by position. Plugin hooks need it too. There is no supported way for
   an installer to grant it, and `codex exec` skips an untrusted hook without a
   word. The hash covers the command string, not the script's contents, so a
   stable thin entry keeps its trust while the script behind it changes.

3. **Pi fits all four layers into one in-process extension**, discovered from
   a symlink in `~/.pi/agent/extensions/` with no trust prompt. A skill
   directory cannot carry code, so `plugins/install.sh`'s existing skill link
   is not enough. **An extension that fails to load stops Pi from starting, in
   every mode.**

4. **Gemini CLI is not installed here, and its future here is a question for
   the human.** It still ships (0.64.0, nightly builds daily), but its own
   README says that for unpaid-tier and Google One users it was replaced by
   Antigravity CLI on 2026-06-18. Where it is used, an **extension** is the
   only hook channel that survives an untrusted folder, and a hook that fails
   with any exit code other than 0 or 1 *blocks* rather than failing open.

5. **"Fail open and quiet" is not any harness's default; each hook script has
   to engineer it.** The recipe differs: on Claude Code and Codex, exit 0 with
   nothing on stdout; on Pi, catch every error inside each handler and keep the
   extension's factory trivial; on Gemini, wrap the command in
   `… 2>/dev/null || true`. On all four, **the checker's own exit code must
   never pass through**: it exits 1 on hard findings and 2 on a usage error,
   and 2 means *block* on three of the four harnesses.

6. **The hook that sees the finished reply can also force another turn**, on
   every harness that has one: Claude Code's `Stop`, Codex's `Stop`, Gemini's
   `AfterAgent`, and a follow-up queued from Pi's `agent_end`. On Claude Code
   even well-meant `additionalContext` from `Stop` continues the turn. A reply
   log must print nothing and decide nothing. Codex has no cap on Stop
   continuations at all.

7. **The document gate has the same quiet channel everywhere**: findings
   appended to the write tool's result (`additionalContext` on Claude Code,
   Codex and Gemini; a patched `tool_result` on Pi). The write stays on disk.
   The *blocking* channels on Claude Code and Gemini instead show the user an
   error or tell the model the write failed. **No harness's gate sees files
   written through the shell** (`cat >`, `sed -i`, heredocs).

8. **The reply text reaches the log differently on each harness.** Claude Code
   and Codex pass the final message's text (`last_assistant_message`). Gemini
   passes all the model's text for the prompt, joined. Pi passes the run's
   messages. None says whether the human explicitly exempted a reply.

9. **One switch can reach every hook layer only as a flag file the scripts
   check.** The built-in switches are each specific to one harness, and most
   are too broad: Claude Code's `disableAllHooks` and Codex's
   `features.hooks = false` also stop the user's other hooks, and disabling
   the Claude Code plugin removes every `linkuistics` skill. Only Codex
   (`[hooks.state."<key>"] enabled = false`) and Gemini
   (`hooksConfig.disabled`) can turn off one hook by itself. On Codex, a
   terminal environment variable does not reach hooks of a TUI session hosted
   by the shared background daemon. Text primed into `AGENTS.md`, or a Gemini
   context file, is outside any switch a hook can read.

10. **Subagent coverage is uneven.** Claude Code and Codex fire tool hooks
    inside subagents and offer `SubagentStart` to inject context into them.
    Gemini fires only tool and model hooks in subagents. Pi has no built-in
    subagents, and the popular `pi-subagents` package runs foreground children
    without the parent's extensions.

11. **The two skill-delivery routes overlap.** Pi and Gemini both read
    `~/.agents/skills`, which Grove's Codex provisioning writes, as well as
    their own skill directories, so each shared skill is found twice. Pi keeps
    the first and notes a collision; Gemini warns at every start.

12. **The prior art most like this skill has bugs the evidence predicts.**
    Superpowers' Pi extension loses its bootstrap from the second prompt on
    (obra/superpowers#2083, open), and its Claude-format `SessionStart` hook
    never fires on Gemini, whose lifecycle matchers are exact strings.

## How skills reach each harness today

| Harness | Skills arrive by | Code can arrive by |
|---|---|---|
| Claude Code | The `linkuistics` marketplace plugin (`.claude-plugin/marketplace.json`), at a commit-SHA version | The same plugin: `plugins/linkuistics/hooks/hooks.json` |
| Codex | Grove's startup provisioning into `~/.agents/skills` (a snapshot embedded in the `grove` binary); `plugins/install.sh` into `~/.codex/skills` | A Codex plugin, `~/.codex/hooks.json`, or `-c` session flags. Not a skill. |
| Pi | `plugins/install.sh` into `~/.pi/agent/skills`; Pi also reads `~/.agents/skills` | A link or file in `~/.pi/agent/extensions/`, or `pi install` |
| Gemini CLI | `plugins/install.sh` into `~/.gemini/skills`, skipped while `~/.gemini` is missing; Gemini also reads `~/.agents/skills` | `gemini extensions install` or `gemini extensions link` |

- **Neither `plugins/install.sh` nor Grove's provisioning delivers code today.**
  `install.sh` links skill directories and nothing else
  (`plugins/install.sh`, `harness_targets`). The provisioning spec places
  skill links in `~/.agents/skills` and says Grove "does not write Claude's
  skill directories or plugin configuration"
  (`docs/specs/codex-skill-provisioning.md`). [S]
- **On Codex, `$CODEX_HOME/skills` is now the deprecated root**; `~/.agents/skills`
  is a current one (`ext/skills/src/host_roots.rs:55-130`). [S]
- **Pi loads `~/.pi/agent/skills` first, then `~/.agents/skills`.** The first
  copy of a name wins; a second copy with a different real path becomes a
  `collision` diagnostic, not an error (`dist/core/package-manager.js:1999-2008`,
  `dist/core/skills.js:299-325`). [S] Here, the winner is `install.sh`'s link to
  the main checkout, and the loser is Grove's snapshot.
- **Gemini reads `~/.gemini/skills` and its alias `~/.agents/skills`**, and warns
  `Skill conflict detected` for each duplicated name at every start
  (`packages/core/src/skills/skillManager.ts:54-146`). [S] Because Grove already
  fills `~/.agents/skills` on this machine, an `install.sh` pass into
  `~/.gemini/skills` would produce one warning per shared skill. [I]

## 1. Claude Code 2.1.302

### Delivery and trust

- **Plugin hooks are always on while the plugin is enabled.** "A plugin's hooks
  don't wait for one of the plugin's skills or commands to be used. Claude Code
  registers them when a session loads the plugin" (plugins › components). [D]
  The file is `hooks/hooks.json` at the plugin root, with a top-level `"hooks"`
  wrapper; a `hooks` key in `plugin.json` also works, and both load
  (plugins-reference). [D]
- **The plugin-level file is the right place, not skill frontmatter.** A
  frontmatter hook (as `guardrail` uses) registers only when the skill is
  invoked (hooks › hook locations), and how long it then lasts is
  inconsistent: the docs say the rest of the session; anthropics/claude-code#102858
  (open, 2.1.295) reports it gone on the next turn; `guardrail`'s own
  `SKILL.md` says until the skill is no longer active. [D][R]
- **Observed working in 2.1.302.** This session's transcript holds one
  `hook_additional_context` attachment for `SessionStart`, carrying the
  explanatory-style plugin's text (1,018 characters, 63 ms) and superpowers'
  (3,405 characters, 93 ms). [L]
- **No per-hook approval.** The only gate is the install-time warning to trust
  a plugin (plugins › security). [D] **Workspace trust gates every hook**,
  plugin hooks included: "Skipping ${…} hook execution - workspace trust not
  accepted" runs before matchers are collected; `-p` treats the folder as
  trusted (hooks › security). [S][D] In this session, launched by Grove into a
  fresh jj workspace, plugin hooks ran, so the gate did not stop them here;
  whether that holds for every new workspace path is not established. [L]
- **Auto-update ships hooks silently.** This user's `linkuistics` marketplace
  has `"autoUpdate": true` and the plugin carries no `version`, so every push
  to `main` reaches the next session, new hooks included, with no prompt
  (`~/.claude/plugins/known_marketplaces.json`; plugins › loading). [L][D] Every
  user with `linkuistics` enabled gets the hooks whether or not they want
  controlled English. [I]
- **A plugin cannot ship `CLAUDE.md` or rules.** "A `CLAUDE.md` at the plugin
  root isn't loaded as context, and `claude plugin validate` warns"
  (plugins-reference). [D] Plugin `settings.json` honours only `agent` and
  `subagentStatusLine`. [D] A forced plugin output style
  (`force-for-plugin: true`) would put text in the system prompt, but it
  overrides the user's style choice, drops Claude Code's software-engineering
  section unless `keep-coding-instructions: true`, and does not shape
  subagents (output-styles). [D] It is too invasive for this purpose. [I]

### Layer by layer

| Layer | Event | Input that matters | What to return |
|---|---|---|---|
| (a) instruction | `SessionStart` (no matcher: fires on `startup`, `resume`, `clear`, `compact`, `fork`) | `source` | `hookSpecificOutput.additionalContext` |
| (a) in subagents | `SubagentStart` | `agent_id`, `agent_type` | `hookSpecificOutput.additionalContext`, "added to the subagent's context" |
| (b) reminder | `UserPromptSubmit` | `prompt` | `hookSpecificOutput.additionalContext` |
| (c) gate | `PostToolUse`, matcher `Write\|Edit\|NotebookEdit` | `tool_input.file_path` (absolute since 2.1.89; `notebook_path` for NotebookEdit) | `hookSpecificOutput.additionalContext` |
| (d) reply log | `Stop`; `SubagentStop` in subagents | `last_assistant_message`, `session_id`, `agent_id` | **nothing**, exit 0 |

Sources: the input and output schemas in the bundle (for example
`last_assistant_message:o().optional().describe("Text content of the last
assistant message before stopping…`), and hooks › each event. [S][D]

- **(a) The instruction is capped at 10,000 characters per string.** Beyond
  that, Claude Code saves the text to a file and passes the path and a
  2,000-character preview; "this cap has no setting or environment variable to
  raise it", and Claude is not asked to read the file (hooks › output). The
  bundle has the threshold constant (`Rzs=1e4`) and the message
  "Output too large (…). Full output saved to: …". [S][D] Resumes are
  de-duplicated by content since 2.1.295 (changelog). [D]
- **(a) Phrase it as facts, not commands.** "Text framed as out-of-band system
  commands can trigger Claude's prompt-injection defenses, which causes Claude
  to surface the text to you instead of treating it as context" (hooks ›
  additional context). [D] `<system-reminder>` tags in hook output are escaped
  since 2.1.292 (changelog). [D]
- **(a) `SessionStart` context does not reach a non-fork subagent**, which
  starts from its own system prompt, its task, `CLAUDE.md`, git status and
  preloaded skills (sub-agents). [D] `SubagentStart` (since 2.0.43) fills the
  gap; this session's subagents received another plugin's `SubagentStart`
  context. [L]
- **(b) The reminder persists and costs tokens on every later turn.** Injected
  text is saved to the transcript and replayed on resume (hooks › additional
  context). [D] Per-prompt and `SessionStart` context is written to the prompt
  cache again in each session (anthropics/claude-code#100709, open). [R] The
  default timeout is 30 seconds, and a timed-out hook's context is dropped.
  [S][D]
- **(b) A reminder hook cannot tell a human prompt from any other turn.** The
  docs and schema describe a `source` field, but the 2.1.302 payload builders
  compile it away: `hook_event_name:"UserPromptSubmit",prompt:r,...!1`. [S]
  `UserPromptSubmit` also fires for scheduled tasks, background-subagent
  reports and other sessions' messages (hooks › UserPromptSubmit). [D]
- **(c) `additionalContext` is the quiet channel.** It reaches the model as
  "${hookName} hook additional context: …" beside the tool result. The
  alternatives, top-level `decision:"block"` and exit 2, render as an error
  line to the user, and the block form shows the model the hook's command line:
  "${hookName} hook blocking error from command: …". [S] The write stays on
  disk in every case (hooks-guide). [D]
- **(c) `MultiEdit` is no longer a tool** (tools-reference); listing it in a
  matcher is harmless. [D] **Writes through Bash do not fire a `Write|Edit`
  hook** (hooks › PostToolUse). [D]
- **(d) `last_assistant_message` holds the final message's text only**, joined
  and trimmed, and is absent when that message has no text; it arrived in
  2.1.47 (changelog). [S][D] Text the model wrote before its tool calls in the
  same turn is not in it. [I] Use it rather than the transcript, which "may
  lag the in-memory conversation" (hooks › common input); a one-turn lag was
  measured in anthropics/claude-code#81825 (open, 2.1.220) and not reproduced
  by a maintainer on Linux. [D][R]
- **(d) Any output from `Stop` continues the turn.** Exit 2,
  `decision:"block"`, and even `additionalContext` ("the conversation
  continues so the model can act on it", bundle schema) all resend the turn,
  up to `CLAUDE_CODE_STOP_HOOK_BLOCK_CAP??8` consecutive times. [S] Stop gates
  are costly in practice: in anthropics/claude-code#97601 (open, 2.1.283), 3.8%
  of turns were bounced and the user had already seen the rejected text. [R]
- **(d) In auto mode a subagent's real report is a tool call**:
  `last_assistant_message` on `SubagentStop` holds only its closing text, and
  the report is `tool_input.message` of `SubagentHandback`, reachable from
  `PostToolUse` (hooks › SubagentStop, 2.1.271 and later). [D]

### Subagents and headless runs

- **Tool hooks fire inside subagents**, with `agent_id` and `agent_type` in the
  input (hooks; sub-agents). [D] This session's subagent transcript records the
  user's `PreToolUse` and `PostToolUse` hooks firing over a hundred times. [L]
  `UserPromptSubmit` does not fire per subagent turn. [L]
- **Headless `claude -p` loads installed plugins and runs hooks**, treating the
  folder as trusted (discover-plugins; hooks › security). [D] **`--bare` skips
  hooks, plugins, skills and `CLAUDE.md`**, "is the recommended mode for
  scripted and SDK calls, and will become the default for `-p` in a future
  release" (headless). [D] `--safe-mode` also disables plugins and hooks
  (env-vars). [D] Async hooks still running at `-p` teardown are killed
  (hooks › async). [D]

### The switch

- **The hook environment is the inherited environment** plus the settings
  `env` block, `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`,
  `CLAUDE_CODE_SESSION_ID` and others (bundle environment builder; hooks;
  plugins-reference). [S][D]
- **No built-in switch fits.** "There is no way to disable an individual hook
  while keeping it in the configuration" (hooks). `disableAllHooks` also stops
  the user's other hooks, the status line and mods (settings). Disabling the
  plugin removes every `linkuistics` skill. A plugin `userConfig` option is
  Claude-only. [D]
- **A flag file or an environment variable checked by each script works.** A
  settings `env` value takes effect on save, but "removing a variable from the
  file doesn't unset it in a running session" (env-vars), so toggle a value
  rather than delete the key. [D] Text already injected stays in a running or
  resumed session. [D]
- **Do not keep the reply log in `CLAUDE_PLUGIN_DATA`**: uninstall deletes it by
  default (plugins-reference). [D]

### For `claude-code-always-on-k11`

- **Handler schema** (bundle): `type: "command"`, `command`, `args` (exec form,
  2.1.139 and later, no shell, placeholders substituted per element),
  `shell`, `timeout` in **seconds**, `async`, `asyncRewake`, `onFailure`
  (2.1.295 and later; ignored on `Stop` and `SubagentStop`), `if`, `once`
  (frontmatter only). [S][D]
- **Defaults to override:** a command hook's timeout is 600 seconds;
  `UserPromptSubmit`'s is 30; `SessionEnd` shares a 1.5-second budget. [S][D]
- **Exit codes:** exit 0 with one JSON object is parsed; any other stdout is
  plain text, which reaches the model only on `SessionStart` and
  `UserPromptSubmit`. Exit 2 blocks, with an effect that depends on the event.
  **Any other non-zero exit is a non-blocking error shown to the user.** So a
  quiet hook exits 0 with empty stdout. [S][D]
- **`hookEventName` is required and must match the event**; a top-level
  `additionalContext` (or snake-case `additional_context`) is dropped with only
  a debug-log line: "Hook JSON output had unrecognized keys (ignored)". [S]
  anthropics/claude-code#96193 (open) shows the silent drop. [R]
- **Two claims in superpowers' hook script do not hold.**
  anthropics/claude-code#43127, which it cites for "a broken/empty PATH" in
  `SessionStart` hooks, is about the Bash tool's shell snapshot; the empty-PATH
  report is obra/superpowers#2310 alone. And 2.1.302 does not read a top-level
  `additional_context`, so the "reads both without deduplication" warning is
  out of date. [S][R] Using an absolute interpreter in exec form still costs
  nothing. [I]
- **Measured cost:** the user's bash hooks took a median of 9 to 14 ms per tool
  event; a native-binary `SessionStart` hook took about 1.4 s; the checker
  `ste-lint.py --json` takes about 30 ms on a short input. [L]
- **A layout assembled from the schema, not run:**

  ```json
  {
    "description": "technical-communication layers (fail open, quiet)",
    "hooks": {
      "SessionStart":     [{ "hooks": [{ "type": "command", "command": "/bin/sh",
        "args": ["${CLAUDE_PLUGIN_ROOT}/skills/technical-communication/<adapter>", "session-start"], "timeout": 10 }] }],
      "SubagentStart":    [{ "hooks": [{ "type": "command", "command": "/bin/sh",
        "args": ["${CLAUDE_PLUGIN_ROOT}/skills/technical-communication/<adapter>", "subagent-start"], "timeout": 10 }] }],
      "UserPromptSubmit": [{ "hooks": [{ "type": "command", "command": "/bin/sh",
        "args": ["${CLAUDE_PLUGIN_ROOT}/skills/technical-communication/<adapter>", "prompt"], "timeout": 5 }] }],
      "PostToolUse":      [{ "matcher": "Write|Edit|NotebookEdit", "hooks": [{ "type": "command", "command": "/bin/sh",
        "args": ["${CLAUDE_PLUGIN_ROOT}/skills/technical-communication/<adapter>", "doc-gate"], "timeout": 15 }] }],
      "Stop":             [{ "hooks": [{ "type": "command", "command": "/bin/sh",
        "args": ["${CLAUDE_PLUGIN_ROOT}/skills/technical-communication/<adapter>", "reply-log"], "timeout": 10 }] }],
      "SubagentStop":     [{ "hooks": [{ "type": "command", "command": "/bin/sh",
        "args": ["${CLAUDE_PLUGIN_ROOT}/skills/technical-communication/<adapter>", "reply-log"], "timeout": 10 }] }]
    }
  }
  ```

  It lives at `plugins/linkuistics/hooks/hooks.json`; the paths under the skill
  are the design's to fix.

## 2. Codex 0.163.0

### Delivery and trust

- **Hooks are on by default.** `hooks` is `Stage::Stable, default_enabled:
  true` (`features/src/lib.rs:1345-1350`), so the user's `[features] hooks =
  true` is redundant. [S][L] Twelve events exist: `PreToolUse`,
  `PermissionRequest`, `PostToolUse`, `PreCompact`, `PostCompact`,
  `SessionStart`, `SessionEnd`, `UserPromptSubmit`, `SubagentStart`,
  `SubagentStop`, `Stop`, `Interrupt` (`hooks/src/lib.rs:23-36`). [S]
- **Hooks come from every active layer**: `$CODEX_HOME/hooks.json` and
  `[hooks]` in `config.toml`, a trusted project's `.codex/`, managed policy,
  `-c` session flags, and plugins. All matching hooks load
  (`hooks/src/engine/discovery.rs:129-295`). [S][D] **Skills cannot declare
  hooks** (no hook handling under `skills/src` or `ext/skills/src`). [S]
- **A non-managed hook runs only if it is enabled and trusted**
  (`hooks/src/engine/discovery.rs:715-736`), since 0.129.0. [S][R]
  - **What is trusted** is a SHA-256 over a normalised identity of one
    handler (event, matcher, `type`, `command`, `timeout`, `async`,
    `statusMessage`, and `additionalContextLimit` when not 2,500): "Hash a
    normalized, config-derived identity instead of source text"
    (`discovery.rs:768-794`). The command is hashed **before**
    `${PLUGIN_ROOT}` substitution. [S] The survey reproduced all twelve
    `trusted_hash` values in the user's `config.toml`, and those of scratch
    user, session-flag and plugin hooks. [L]
  - **So editing the script needs no re-trust, but changing the entry does.**
    A changed command, matcher, timeout or status message resets trust: in
    openai/codex#21615 (open, comment on 0.161.0), changing only a timeout made
    every host's trust `Modified`. [S][R]
  - **The key is positional**, `<source>:<event>:<group>:<handler>`, with the
    source being the file's absolute path or, for a plugin,
    `<plugin>@<marketplace>:<hooks file>` (`hooks/src/lib.rs:112-123`). Adding
    a hook before an existing one un-trusts the hooks after it
    (openai/codex#49399, closed as not planned). [S][R] The user's
    `~/.codex/hooks.json` already holds twelve trusted handlers written by
    three tools; any addition there belongs at the end of each event's list.
    [I]
  - **How trust is granted:** in the TUI, a startup modal "Hooks need review"
    offers to review, to trust all, or to continue without trusting; `/hooks`
    trusts and toggles one by one. "Trust all" trusts every pending hook across
    plugins (openai/codex#47285, open) (`tui/src/startup_hooks_review.rs`). The
    docs say startup only prints a warning pointing at `/hooks`, which
    disagrees with the 0.163.0 source. [S][R][D]
  - **In `codex exec` an untrusted hook is skipped silently** (the trust skip
    pushes no warning, `discovery.rs:697-736`; openai/codex#46210, open). [S][R]
  - **There is no supported way for an installer to grant trust.**
    openai/codex#21615 (open since 0.129.0) names writing
    `[hooks.state."<key>"] trusted_hash` into the user's config as "the
    practical alternative". [R] `--dangerously-bypass-hook-trust` runs every
    enabled hook, unreviewed project hooks included. [S]
  - **A per-launch route needs no file edit.** A launch can pass the hooks and
    their trust as session flags:
    `-c 'hooks.UserPromptSubmit=[{hooks=[{type="command",command="…"}]}]'`
    with `-c 'hooks.state={"/<session-flags>/config.toml:user_prompt_submit:0:0"={trusted_hash="sha256:…"}}'`
    was reported `trusted` by the app-server's `hooks/list`. [L] That a real
    turn then runs the hook is not yet shown. The dotted form
    `-c 'hooks.state."<key>".trusted_hash=…'` fails, because the override path
    is split on every `.` (`config/src/overrides.rs:22`). [S][L]
- **Codex plugins can ship hooks, but not instructions.**
  - The manifest is looked for as a root `plugin.json` in the Agent Plugins
    format, then `.codex-plugin/plugin.json`, `.claude-plugin/plugin.json` and
    `.cursor-plugin/plugin.json`; marketplaces include
    `.claude-plugin/marketplace.json`
    (`utils/plugins/src/plugin_namespace.rs:43-80`,
    `core-plugins/src/marketplace.rs:20-24`). [S] So Codex can load
    Claude-format plugin directories as they are. [I]
  - **Agent Plugins format loads no hooks** in 0.163.0
    (`core-plugins/src/loader.rs:966-970`; openai/codex#47925, open), although
    the docs present it as the portable path. Use `.codex-plugin/plugin.json`.
    [S][R][D]
  - A manifest's `hooks` takes a path, paths or inline objects; without it,
    `hooks/hooks.json` loads; `"hooks": {}` means none
    (`core-plugins/src/manifest.rs:150-158`, `loader.rs:72`). [S][L] There is
    no manifest field for instructions (`manifest.rs:50-73`). [S]
  - Installing a plugin does not trust its hooks: a scratch plugin's hooks
    were listed `enabled: true, trustStatus: "untrusted"` after
    `codex plugin add`; the docs agree. [L][D]
  - Superpowers dropped its Codex `SessionStart` hook because "the bootstrap
    hook made the UX worse rather than better" and to avoid "its install-time
    trust prompt" (superpowers `RELEASE-NOTES.md`, v6.1.0 and v6.1.1). [R]
- **Hooks run outside the sandbox.** The runner spawns the shell directly and
  `hooks/src` never mentions a sandbox (`hooks/src/engine/command_runner.rs:384-437`);
  the TUI says "Hooks can run outside the sandbox after you trust them". [S]
  So a hook can write a log file under `~/.local/state` even when the model's
  own commands cannot. [I] The trust prompt is therefore Codex's security
  boundary for hooks, and a design that grants trust by machine bypasses it.
  [I]

### Instructions

- **Global** text comes from `$CODEX_HOME/AGENTS.override.md` if non-empty,
  else `$CODEX_HOME/AGENTS.md` (`codex-home/src/instructions/mod.rs`); the
  project walk adds `AGENTS.override.md` or `AGENTS.md` per directory from the
  repository root to the working directory, within `project_doc_max_bytes`
  (32 KiB), and only for trusted projects (`core/src/agents_md.rs`). [S] It is
  delivered as one user message. [L]
- **`~/.codex/instructions.md` is not read**: a probe's markers in
  `AGENTS.md` files appeared in `codex debug prompt-input`, and the marker in
  `instructions.md` did not. [L] The user's copy holds only a heading.
- **Without editing the user's file**, always-on text can come from a trusted
  `SessionStart` hook (a developer message), from `SubagentStart` for
  children, or from `-c developer_instructions=…` for launches the caller
  controls (it appeared as the first developer item in a probe). [S][L]

### Layer by layer

| Layer | Event | Input that matters | What to return |
|---|---|---|---|
| (a) instruction | `SessionStart` (runs at the first turn; `source` `startup\|resume\|clear\|compact\|fork`) | `source` | `hookSpecificOutput.additionalContext` |
| (a) in subagents | `SubagentStart` (replaces `SessionStart` in spawned children) | `agent_id`, `agent_type` | `hookSpecificOutput.additionalContext` |
| (b) reminder | `UserPromptSubmit` | `prompt`, `turn_id` | `hookSpecificOutput.additionalContext` |
| (c) gate | `PostToolUse`, matcher `apply_patch` (aliases `Write`, `Edit`) | `tool_input.command`: the whole patch text | `hookSpecificOutput.additionalContext` |
| (d) reply log | `Stop`; `SubagentStop` in children | `last_assistant_message`, `session_id`, `turn_id`, `agent_id` | **nothing**, exit 0 |

Sources: `hooks/src/schema.rs:278-638`, `hooks/src/events/common.rs:112-128`,
`core/src/tools/hook_names.rs:28-56`. [S]

- **Context is a developer message with no markers**
  (`core/src/context/hook_additional_context.rs:15-34`), capped by default at
  about 2,500 tokens; larger text is spilled to a temporary file with a preview
  (`hooks/src/engine/output_spill.rs:12`). [S] It does not show in the 0.163.0
  TUI ("Model-facing hook context never appears in the TUI",
  `tui/src/history_cell/hook_cell.rs:5`). [S]
- **(b) The reminder lands after the user's prompt**
  (`core/src/hook_runtime.rs:716-773`), so it is the last thing the model
  reads. In openai/codex#40680 (open, 0.149.1) a trailing style directive was
  taken as the task: the model answered "Understood. Ready for the task." [S][R]
  Phrase the reminder as a qualifier of the user's message. [I] It does not run
  for multi-agent v2 messages (`hook_runtime.rs:705-713`). [S]
- **(c) There is no changed-paths field.** The hook gets the patch text and
  must parse its `*** Add File:`, `*** Update File:` and `*** Move to:` headers
  (`core/src/tools/handlers/apply_patch.rs:417-432`,
  `apply-patch/src/parser.rs:39-42`). [S] `PostToolUse` fires only for a
  successful call (`core/src/tools/registry.rs:811-818`), and **not at all when
  the model runs `apply_patch` through the shell tool**
  (`unified_exec/exec_command.rs:365-392`, `tools/context.rs:432-436`). [S]
  How often current models do that is unknown. [I]
- **(c) A `PostToolUse` block replaces the tool's result with the feedback**
  ("A PostToolUse block rejects the result, not the already-completed tool
  execution", `registry.rs:846-871`); `additionalContext` leaves the result
  alone. [S]
- **(d) `Stop` has no loop cap.** A block records the reason as a user-role
  `<hook_prompt>` message, sets `stop_hook_active = true` and loops
  (`core/src/session/turn.rs:735-756`); nothing limits the count. [S]
  openai/codex#37937 (open) reports "an infinite no-escape loop", and
  openai/codex#34477 (open) a hook-injected loop that ran for about nine hours.
  [R] `Stop` also fires at every `/goal` segment (openai/codex#52950), skips
  `/review` (openai/codex#53574) and does not fire after an error or rate-limit
  stop (openai/codex#21160), all open. [R]
- **(d) `Stop` and `SubagentStop` reject plain stdout and unknown fields**,
  including a Claude-style `hookSpecificOutput`, as a visible failure
  (`hooks/src/events/stop.rs:326-341`; openai/codex#18887, open). [S][R]
- **(d) Legacy `notify` is an ungated alternative.** `notify = [argv…]` runs
  after the Stop hooks with `last-assistant-message` in its JSON argument, with
  no trust gate and regardless of `features.hooks`
  (`hooks/src/legacy_notify.rs:13-71`). It is a single slot, the user already
  uses it for the Computer Use client, and the code marks it for removal. [S][L]

### Subagents, `codex exec` and the daemon

- **Spawned subagents get `SubagentStart` instead of `SessionStart`, and
  `SubagentStop` instead of `Stop`**; internal subagents skip start hooks
  (`core/src/hook_runtime.rs:128-181`, `:392-446`). Tool hooks fire in children
  with `agent_id` and `agent_type`, and a test asserts that `SubagentStart`
  context reaches the child's model request
  (`core/tests/suite/subagent_notifications.rs:624-760`). [S] Subagents share
  the parent's `session_id` (release 0.131.0). [R]
- **Memory-consolidation workers run `SessionStart` hooks**
  (openai/codex#52442, open), so an always-on instruction also lands there. [R]
- **`codex exec` uses the same engine**, with no trust prompt; async hooks
  still running at session end are killed (`command_runner.rs:188-196`,
  `:331-357`). [S]
- **The interactive TUI usually runs sessions in a shared background daemon**
  (`daemon_auto_start` default on), so hooks inherit **the daemon's**
  environment, captured when it started, not the terminal's
  (`hooks/src/registry.rs:79`; openai/codex#44902, open). [S][R] Launches with
  `--profile`, `--no-daemon`, `--strict-config`,
  `--dangerously-bypass-hook-trust`, or any `-c` override other than a few TUI
  keys and boolean feature flags run embedded, in the launching process
  (`tui/src/daemon_startup.rs:65-125`). [S] Grove's Codex launches pass
  `-c default_permissions=grove-jj` (`~/.codex/config.toml` comment), so they
  run embedded and their hooks see the launching environment. [S][I]
- **`codex mcp-server` no longer exists**: "The deprecated `codex mcp-server`
  entry point is no longer available" (release 0.154.0, #42993). [R][L]

### The switch

- `[features] hooks = false` (or `--disable hooks`, `-c features.hooks=false`)
  turns off every non-managed hook, the user's other tools' hooks included;
  legacy `notify` keeps running (`hooks/src/engine/mod.rs:242-260`,
  `registry.rs:118-129`). [S][L]
- Per plugin: `[plugins."<plugin>@<marketplace>"] enabled = false`, or per run
  `-c 'plugins={"<plugin>@<marketplace>"={enabled=false}}'`; the dotted form
  does nothing, silently. Per hook: `[hooks.state."<key>"] enabled = false`.
  [L]
- **No environment variable disables hooks** (no such lookup in the source).
  [S] An environment-variable switch checked by the script reaches `codex exec`
  and embedded launches, but not daemon-hosted TUI sessions. **A flag file is
  the form that reaches every Codex session.** [I]

### For `codex-always-on-k12`

- **Schema** (`config/src/hook_config.rs:10-201`): `{"description"?, "hooks":
  {<Event>: [{"matcher"?, "hooks": [{"type": "command", "command",
  "timeout"? (seconds, default 600), "async"?, "statusMessage"?,
  "additionalContextLimit"?}]}]}}`. Any other top-level key fails the whole
  file. A matcher of only letters, digits, `_` and `|` is an exact list;
  anything else is an unanchored regex. [S]
- **Output:** exit 0 with empty stdout does nothing; exit 0 with stdout is
  parsed as JSON, and `hookSpecificOutput.hookEventName` is required. Exit 2
  blocks with stderr as the reason. Any other exit, a spawn error or a timeout
  is a red row in the TUI and `hook: <Event> Failed` in `exec`, never sent to
  the model. `suppressOutput: true` on `PostToolUse` fails the run and drops its
  context (`output_parser.rs:393-399`). [S]
- **Plugin hook environment:** `PLUGIN_ROOT`, `CLAUDE_PLUGIN_ROOT`,
  `PLUGIN_DATA`, `CLAUDE_PLUGIN_DATA`, also substituted as `${…}` in the
  command (`discovery.rs:262-270`). The environment otherwise is the hosting
  process's snapshot minus a short list of auth tokens;
  `shell_environment_policy` does not apply (`protocol/src/shell_environment.rs:14-28`).
  [S] The hook shell is the session's shell without a login profile. [S]
- **Pitfalls not listed above:** a git-marketplace auto-upgrade can delete the
  cached plugin a live session points at, and its hooks then fail for the rest
  of the session (openai/codex#31383, open); edits to `hooks.json` are not
  reloaded (openai/codex#17636, open); a background child must close stdout and
  stderr or the hook waits for it. [R][I]

## 3. Pi 0.84.2

### Delivery

- **Every `.ts` or `.js` file, and every directory with an `index.ts` or a
  `pi.extensions` manifest, in `~/.pi/agent/extensions/` loads on every
  start, with no trust check** (`dist/core/package-manager.js:2000`,
  `:391-441`; docs/extensions.md). Symlinks are followed; a dangling link is
  skipped silently (`package-manager.js:413-421`). [S][D] Project
  `.pi/extensions` load only for a trusted project, and in `-p`, JSON and RPC
  modes untrusted project resources are ignored (`package-manager.js:1980-1983`;
  docs/security.md). [S][D]
- **A skill directory cannot carry an extension.** The skill loader reads only
  Markdown (`dist/core/skills.js:4-7`, `:137`). [S] So the link `install.sh`
  already makes in `~/.pi/agent/skills` loads no code; a second link into
  `~/.pi/agent/extensions/` would. [I]
- **Packages** (`pi install npm:…|git:…|<path>`) carry `"pi": {"extensions",
  "skills", …}` in `package.json`, and `pi install` writes
  `~/.pi/agent/settings.json` (docs/packages.md). [D] A directory in
  `extensions/` with a `pi` manifest loads only its extensions, not its skills
  (`dist/core/extensions/loader.js:476-489`). [S]
- **TypeScript is compiled on load with no type check** (jiti;
  `loader.js:358-368`). [S]

### Instructions

- One context file per directory, the first of `AGENTS.override.md`,
  `AGENTS.md`, `AGENTS.MD`, `CLAUDE.md`, `CLAUDE.MD`, from `~/.pi/agent/` and
  then from every ancestor of the working directory, outermost first
  (`dist/core/resource-loader.js:31-106`). [S] `SYSTEM.md` replaces the base
  prompt and `APPEND_SYSTEM.md` appends to it, project copy (if trusted) *or*
  global copy, never both; `--append-system-prompt` suppresses
  `APPEND_SYSTEM.md` altogether (`resource-loader.js:380-389`, `:808-828`). [S]
- **`~/.pi/rules/` is not loaded in 0.84.2**, re-verified: no code reads it.
  The user's `~/.pi/rules/lean-ctx.md` stays dormant. [S]
- **An extension can add always-on text without touching the user's files**,
  through `before_agent_start` (below). There is no stable "append once" API;
  the feature request for one, earendil-works/pi#9432 (open), notes that
  writing `AGENTS.md` "mutates user/project-owned instructions, survives
  extension removal, and hides the source of the rule". [S][R]

### Layer by layer

Types are in `dist/core/extensions/types.d.ts`; handlers run in load order,
are awaited, and have **no timeout** (`dist/core/extensions/runner.js:579-609`). [S]

| Layer | Event | Payload that matters | What to return or do |
|---|---|---|---|
| (a) instruction | `before_agent_start` | `systemPrompt` (always the base prompt for the first handler) | `{ systemPrompt: event.systemPrompt + text }`, a full replacement for this run, never saved |
| (b) reminder | `before_agent_start` | `prompt` | `{ message: { customType, content, display: false } }`, saved as a custom message |
| (c) gate | `tool_result` for `write` and `edit` | `toolName`, `input.path`, `content`, `isError` | `{ content: [...event.content, findings] }` |
| (d) reply log | `agent_end` | `messages` of this run | nothing; append the record |

- **(a) The text must be returned on every prompt**, because the override is
  cleared after each run (`dist/core/agent-session.js:744-756`, `:901-909`).
  **Runs started by `pi.sendMessage(…, {triggerTurn: true})` while idle skip
  `before_agent_start`**, so their requests go out without the text
  (`agent-session.js:1089-1090`); earendil-works/pi#8712 and #10267 report it,
  the latter with a prompt-cache re-billing of 571,836 tokens. [S][R] It is
  fixed upstream in 1.2.0, where the event becomes a union on `messageType`
  (upstream CHANGELOG). [D]
- **(b) The reminder message is persisted**, one per prompt, and sent as a user
  message; `display: false` hides it only in the TUI
  (`dist/core/messages.js:89-95`). [S] The alternative is a `context` handler,
  which runs before **every** LLM request on a structured clone and whose
  result is used for that request only (`node_modules/@earendil-works/pi-agent-core/dist/agent-loop.js:179-190`). [S]
- **(c) The model sees the patched result before its next call**, in the same
  turn, so the gate needs no extra turn and cannot loop
  (`agent-session.js:244-272`). [S] Keep `isError` false. **Do not gate in
  `tool_call`**: a throwing `tool_call` handler blocks the tool ("`tool_call`
  errors block the tool (fail-safe)", docs/extensions.md). [S][D] Writes through
  `bash`, or through other extensions' tools, are invisible to the gate. [I]
- **(d) `agent_end` fires once per low-level run**, so retries and follow-ups
  give more than one per prompt (docs/extensions.md). [D] To make the model fix
  findings before it stops, queue a follow-up from `agent_end`, which keeps the
  run's system prompt (`agent-session.js:779-781`), and cap it: an uncapped
  follow-up loops (earendil-works/pi#4577, closed), and one queued during a run
  is stranded if the run aborts (earendil-works/pi#10017, open). [S][R] Do not
  log from `message_end`: a hung handler there "blocks the agent loop forever
  and loses the assistant message" (earendil-works/pi#10979). [R]

### Failure modes peculiar to Pi

- **A broken extension stops Pi from starting, in every mode.** A syntax
  error, a missing import, a throwing factory or a non-function export becomes
  "Failed to load extension", and then `if (runtime.diagnostics.some(… "error"))
  { … process.exit(1); }` (`dist/main.js:716-724`), with the hint to start with
  `pi -ne`. Upstream main does the same. [S] A tool or flag name that collides
  with another extension's is also a load error (`resource-loader.js:838-872`;
  earendil-works/pi#9649). [S][R] Through a symlink into the main checkout, one
  bad commit would stop every Pi session on the machine, unattended Grove runs
  included. [I] **`pi --help` exits 0 before that check**, so it cannot serve as
  a load test (`main.js:688-699`). [S]
- **Handler errors are visible, not quiet**: red text and a stack trace in the
  TUI, `Extension error (…)` on stderr in print mode, an `extension_error` event
  in RPC (`dist/modes/interactive/interactive-mode.js`, `print-mode.js:78-80`,
  `rpc/rpc-mode.js:259-261`). [S] Quiet means catching everything inside every
  handler.
- **Unknown event names are accepted silently** (`loader.js:209-214`); the
  user's `orca-agent-status.ts` subscribes to five events that 0.84.2 does not
  have. A renamed event becomes a dead hook with no error. [S]
- **The extension API moves fast.** Upstream is at 1.3.0; since 0.84.2 it has
  changed `before_agent_start` (1.2.0), added a prompt-sections API (0.86.0),
  and made `turn_end` actionable (0.87.0) (upstream CHANGELOG). [D] Feature-detect
  rather than assume one shape. [I]

### Subagents and headless runs

- **Pi has no built-in subagent tool** (docs/extensions.md; the user's
  `AGENTS.md`). The bundled example extension spawns child `pi -p --mode json`
  processes without `--no-extensions` and with the inherited environment, so
  its children load the same user extensions
  (`examples/extensions/subagent/index.ts:300-350`). [S]
- **`pi-subagents`** (not installed for this user) runs foreground children
  inside the parent process without its ambient extensions, and drops
  `~/.pi/agent/AGENTS.md` by default (its `agents.md`, "Tool and extension
  selection"). [D, third party] Its children would have only the fallback. [I]
- **Headless modes load extensions**: `-p`, JSON and RPC build the same session
  (`main.js:782-788`), and Pi chooses print mode by itself when stdin or stdout
  is not a terminal (`main.js:83-94`). [S]

### The switch

- **Extensions run in the Pi process and see the live `process.env`**; children
  started by `pi.exec` or `child_process.spawn` inherit it (`loader.js:351-377`,
  `dist/core/exec.js:12-16`). [S] A flag file checked in every handler is live
  without a restart. [I]
- **Built-in off switches:** `pi -ne` (all discovered and package extensions;
  `resource-loader.js:315-317`), or an `extensions` entry in settings such as
  `!pattern` (`package-manager.js:506-522`). [S]
- **On Pi the switch reaches everything only if no technical-communication text
  sits in `AGENTS.md`**: that file still loads with `-ne` and on the runs that
  skip `before_agent_start`. [I]

### For `pi-always-on-k13`

- Keep the factory to registrations only; register no tools and no flags; make
  no value import from `@earendil-works/*` (0.84.2 still aliases the old
  `@mariozechner` scope, `loader.js:32-56`). [S][I]
- `pi.exec(cmd, args, {timeout})` cannot feed stdin and reports `code: 0` for a
  process killed by its timeout (`dist/core/exec.js:56-63`); check `killed`, or
  use `child_process.spawn` with an explicit timer. Never `spawnSync`, which
  blocks rendering (earendil-works/pi#9811). [S][R]
- Never put a function in a message's `details`: the history is structured-cloned
  before every request, and one non-cloneable value "silently breaks every later
  model call" (earendil-works/pi#10714). [S][R]
- Superpowers' approach does not hold: it injects its bootstrap through
  `context` and clears its flag at `agent_end`, so the bootstrap is gone from the
  second prompt on (obra/superpowers#2083, open). `before_agent_start` →
  `systemPrompt` is simpler and survives compaction. [S][R]
- A load check that spends no model quota is still to be found:
  `pi --mode rpc --no-session -ne -e ./index.ts < /dev/null` is a candidate.
  [I]
- **Side finding outside this survey.** The user's
  `~/.pi/agent/extensions/cbmem.ts`, generated by `codebase-memory-mcp`,
  registers tools with no `parameters` schema. On providers using
  `anthropic-messages`, which includes the Kimi package, such a tool "makes
  every request … fail" (earendil-works/pi#9300), and upstream 0.86.0 and later
  reject it at registration, which would make Pi exit 1 after an upgrade
  (upstream `loader.ts:289-295`). [R][S]

## 4. Gemini CLI 0.64.0

### Status here

- **Not installed**: `gemini` is not on `PATH`, `~/.gemini` does not exist, and
  neither global npm prefix nor Homebrew has it; `plugins/install.sh` skips it
  (`plugins/install.sh`, "harness not installed"). [L]
- **Its README, on `main` today:** "**Unpaid tier and Google One users:**
  Gemini CLI was replaced by Antigravity CLI on June 18th, 2026", and "Gemini
  CLI is not optimized for the latest Gemini models. For the best coding
  experience with Gemini models use Antigravity CLI." [D] It still releases:
  0.64.0 on 2026-10-13, and nightly builds to 2026-10-18 (npm). [L] Superpowers
  removed its Gemini support on this news in v6.1.0 (2026-06-30) and restored
  it in v6.2.0 (2026-07-23), calling the removal "premature"
  (obra/superpowers#1959). [R] Antigravity CLI is not installed here and is not
  surveyed.

### Delivery and trust

- **Hooks have been on by default since 0.26.0**: `hooksConfig.enabled`
  defaults to `true`, "Canonical toggle for the hooks system"
  (`packages/cli/src/config/settingsSchema.ts:2548-2567`). The old
  `hooks.enabled` and `hooks.disabled` keys of 0.24 and 0.25 are now skipped
  silently, and the release's own docs still show `hooks.disabled`
  (`bundle/docs/hooks/best-practices.md`). [S]
- **An extension ships hooks in `hooks/hooks.json`**, a fixed name ("hooks are
  not defined in the `gemini-extension.json` manifest",
  `bundle/docs/extensions/reference.md`;
  `packages/cli/src/config/extension-manager.ts:1063-1098`). [S][D] Install
  consent warns "This extension contains Hooks which can automatically execute
  commands" and is asked again on an update that adds hooks
  (`packages/cli/src/config/extensions/consent.ts:310-323`); `--consent` skips
  it. [S][D]
- **Settings-file hooks, the user's included, are dropped in an untrusted
  folder; extension hooks are not.** `processHooksFromConfig` registers the
  merged settings hooks only `if (this.config.isTrustedFolder())`, logging
  "Project hooks disabled because the folder is not trusted"; extension hooks
  are registered after it with no trust check
  (`packages/core/src/hooks/hookRegistry.ts:175-198`). [S] Folder trust is on by
  default since 0.28.0, and a folder never seen counts as untrusted
  (`packages/cli/src/config/trustedFolders.ts:32-35`;
  `packages/core/src/config/config.ts:3181-3200`). [S] The release's docs
  contradict themselves on whether trust is on by default. [D] This may explain
  google-gemini/gemini-cli#27712 (open: "hooks.AfterAgent is never executed in
  settings.json configuration"). [R][I] **So on Gemini the hooks belong in an
  extension, not in `~/.gemini/settings.json`.** [I]
- **Headless runs in an untrusted folder exit fatally** unless `--skip-trust`
  or `GEMINI_CLI_TRUST_WORKSPACE=true` is given. [S]
- **A Claude Code plugin and a Gemini extension cannot share one `hooks/`
  directory**: Gemini warns, visibly, on each Claude event name it does not
  know (`hookRegistry.ts:213-219`; google-gemini/gemini-cli#25630, #19969). [S][R]

### Instructions and skills

- **Context files:** `~/.gemini/GEMINI.md` goes into the system instruction;
  project files and extension context go into the first user message
  (`packages/core/src/config/config.ts:2598-2635`). Global and extension memory
  load whatever the folder's trust; project memory only when trusted
  (`packages/core/src/context/memoryContextManager.ts:49-66`). [S] The default
  file name is `GEMINI.md` only; `AGENTS.md` loads only when `context.fileName`
  names it (`packages/core/src/tools/memoryTool.ts:11`;
  google-gemini/gemini-cli#28227, open). [S][R]
- **An extension's `contextFileName`** (or a `GEMINI.md` at its root) is
  always-on context while the extension is active
  (`bundle/docs/extensions/reference.md`). [D] Superpowers' Gemini extension is
  exactly this: a two-line `GEMINI.md` of `@./…` imports. [L] **No environment
  variable or flag file can switch that layer off**; only
  `gemini extensions disable` or `-e none` can. [I]
- **An `@import` must resolve, by real path, inside its project root**, or it is
  replaced by `<!-- Import failed: … - Path traversal attempt -->`
  (`packages/core/src/utils/memoryImportProcessor.ts:390-424`). [S] An import
  from `~/.gemini/GEMINI.md` through `install.sh`'s symlink into the grove
  checkout would therefore fail. [I]
- **Skills are on demand, and activating a non-built-in skill asks the user to
  confirm** (`packages/core/src/tools/activate-skill.ts:72-108`;
  `bundle/docs/cli/skills.md`). [S][D] So on Gemini the fallback needs both the
  model's choice and a click. [I] Skills are stable since 0.29.0 and
  extensions can ship them. [R][D]

### Layer by layer

| Layer | Event | Input that matters | What to return |
|---|---|---|---|
| (a) instruction | `SessionStart` (**no matcher**) | `source` | `hookSpecificOutput.additionalContext` |
| (b) reminder | `BeforeAgent` (matcher ignored) | `prompt` | `hookSpecificOutput.additionalContext` |
| (c) gate | `AfterTool`, matcher `^(write_file\|replace)$` | `tool_response.returnDisplay.filePath` (absolute) | `hookSpecificOutput.additionalContext` |
| (d) reply log | `AfterAgent` (matcher ignored) | `prompt_response`, `session_id` | `{}`, never a `decision` |

Sources: `packages/core/src/hooks/types.ts:43-722`,
`packages/core/src/core/client.ts`, `packages/core/src/core/coreToolHookTriggers.ts`,
`packages/core/src/tools/write-file.ts:549-558`, `edit.ts:1051-1060`. [S]

- **Lifecycle matchers are exact strings**, tool matchers unanchored regexes:
  `matchesTrigger` is `matcher === trigger`, while `matchesToolName` uses
  `new RegExp(matcher).test(toolName)`
  (`packages/core/src/hooks/hookPlanner.ts:100-121`). [S] So
  `"startup|resume|clear"` never fires (google-gemini/gemini-cli#28160, 0 of 94
  sessions), and superpowers' Claude-format `SessionStart` hook is inert on
  Gemini. [S][R][I]
- **(a) In interactive mode the context becomes a user-role history entry; in
  headless mode it is prepended to the prompt** (`packages/cli/src/gemini.tsx:937-958`). [S]
- **(b) `BeforeAgent` context joins this turn's request and stays in history**,
  firing once per prompt (`client.ts:165-193`, `:977-985`). [S][I]
- **(c) `additionalContext` is appended to the tool result** as
  `<hook_context>…</hook_context>`; a `deny` instead replaces the result with
  "Tool result blocked" and marks it an error (`coreToolHookTriggers.ts:207-237`).
  [S] `<` and `>` in context are HTML-escaped (`types.ts:263-277`). [S]
- **(d) `prompt_response` is cumulative**: all model text for the prompt, joined
  with newlines, or `'[no response text]'` (`client.ts:238-241`, `:891-903`).
  [S] A `deny` from `AfterAgent` makes its reason a new prompt, up to
  `MAX_TURNS = 100` (`client.ts:79`, `:1032-1064`); `stop_hook_active` was
  broken until 0.34.0, which gave an endless loop
  (google-gemini/gemini-cli#20426). [S][R]

### Failure modes peculiar to Gemini

- **A broken hook blocks.** Plain-text output with exit 0 is allowed and shown;
  with exit 1 it is a warning; with **any other exit code, including 2 and
  127, it becomes `decision: 'deny'`**: "All other non-zero exit codes
  (including 2) are blocking" (`packages/core/src/hooks/hookRunner.ts:537-560`).
  stderr is parsed when stdout is empty (`hookRunner.ts:451-489`). The release's
  docs say other codes give "a warning" and that stderr is never parsed as JSON
  (`bundle/docs/hooks/reference.md`, `index.md`). [S][D] So a missing
  interpreter would discard the user's turn on `BeforeAgent`, report a good
  write as failed on `AfterTool`, and start a retry loop on `AfterAgent`. [I]
- **Hooks are never invisible**: a failure shows "Hook(s) [x] failed … Press
  F12", and an "Executing Hook" indicator shows for every hook unless
  `hooksConfig.notifications` is false (`hookEventHandler.ts:397-432`). [S]
- **Timeouts are milliseconds** (default 60,000, `hookRunner.ts:36`), and
  `gemini hooks migrate --from-claude` copies Claude's seconds unchanged
  (google-gemini/gemini-cli#29122, open). [S][R]
- **Matching hooks run in parallel** unless one definition sets
  `sequential: true` (`hookPlanner.ts:53-56`). [S]

### Subagents and headless runs

- **Subagents are on by default** (`experimental.enableAgents`, since 0.35.0).
  Their tool calls go through the same scheduler, so `AfterTool` fires inside
  them; `BeforeAgent` and `AfterAgent` do not (`packages/core/src/agents/local-executor.ts:1251-1258`;
  google-gemini/gemini-cli#18278 and #15269, both open). [S][R] Subagents get
  global and extension memory, but not `SessionStart` history. [S][I]
- **Headless `gemini -p` runs extensions, `SessionStart`, `BeforeAgent` and
  `AfterAgent`** (`packages/cli/src/nonInteractiveCli.ts:326`). [S]

### The switch

- **The hook environment is inherited in full by default**, plus
  `GEMINI_PROJECT_DIR` (the working directory, despite the docs' "project
  root"), `GEMINI_CWD`, `GEMINI_SESSION_ID` and `CLAUDE_PROJECT_DIR`
  (`hookRunner.ts:348-356`); redaction is off by default
  (`packages/core/src/services/environmentSanitization.ts:13-22`). [S] The docs
  say hooks get "a sanitized environment" (`bundle/docs/hooks/index.md`). [D]
- `${extensionPath}` is substituted into `hooks/hooks.json` once, at load, and
  is not shell-escaped: quote it (`extension-manager.ts:871-877`). [S]
- **Off switches:** `gemini extensions disable <name>`, `-e none`,
  `hooksConfig.enabled: false`, `hooksConfig.disabled: ["<name>"]`, `/hooks
  disable-all` (`packages/cli/src/config/extensions/extensionEnablement.ts`,
  `hookRegistry.ts:256-281`). [S] A flag file checked by every hook script
  reaches all four hook layers, but not a context file. [I]

### For `gemini-always-on-k14`

The layer table above, plus, from the same source:

```json
{ "hooks": {
  "SessionStart": [{ "hooks": [{ "name": "tc-session", "type": "command",
    "command": "\"${extensionPath}/<adapter>\" session-start 2>/dev/null || true", "timeout": 5000 }] }],
  "BeforeAgent":  [{ "hooks": [{ "name": "tc-reminder", "type": "command",
    "command": "\"${extensionPath}/<adapter>\" before-agent 2>/dev/null || true", "timeout": 3000 }] }],
  "AfterTool":    [{ "matcher": "^(write_file|replace)$", "hooks": [{ "name": "tc-docgate", "type": "command",
    "command": "\"${extensionPath}/<adapter>\" after-tool 2>/dev/null || true", "timeout": 20000 }] }],
  "AfterAgent":   [{ "hooks": [{ "name": "tc-replylog", "type": "command",
    "command": "\"${extensionPath}/<adapter>\" after-agent 2>/dev/null || true", "timeout": 10000 }] }]
} }
```

This is assembled from the schema, not run. It needs its own extension
directory beside a `gemini-extension.json`, and is installed with
`gemini extensions link <dir>` (a live link) or `install` (a copy). [S][D][I]

## 5. Across the harnesses

### The layer map

| | Claude Code 2.1.302 | Codex 0.163.0 | Pi 0.84.2 | Gemini CLI 0.64.0 |
|---|---|---|---|---|
| (a) instruction | Plugin `SessionStart` context; `SubagentStart` for subagents | Trusted `SessionStart` context; `SubagentStart`; `-c developer_instructions` for Grove's launches | Extension `before_agent_start` → `systemPrompt` | Extension `SessionStart` context, or extension `contextFileName` |
| (b) reminder | `UserPromptSubmit` context | `UserPromptSubmit` context, after the prompt | `before_agent_start` message, or a `context` insertion | `BeforeAgent` context |
| (c) document gate | `PostToolUse` on `Write\|Edit\|NotebookEdit`, context | `PostToolUse` on `apply_patch`, parse the patch, context | `tool_result` on `write`/`edit`, append to content | `AfterTool` on `^(write_file\|replace)$`, context |
| (d) reply log | `Stop`/`SubagentStop`, `last_assistant_message`, print nothing | `Stop`/`SubagentStop`, `last_assistant_message`, print nothing | `agent_end`, last assistant message of the run | `AfterAgent`, cumulative `prompt_response`, print `{}` |
| (e) switch | Flag file or env var read by each script | Flag file (env var misses daemon-hosted TUI sessions) | Flag file or `process.env` in each handler | Flag file or env var; context file only by disabling the extension |
| On from installation? | Yes: plugin enabled, auto-update, no prompt | **No**: each handler needs a person's trust | Yes: link in `~/.pi/agent/extensions/`, no prompt | After `gemini extensions install`/`link`, with a consent prompt |
| Default when a hook breaks | Non-zero other than 2 is a visible non-blocking error; 2 blocks | Visible failure row; 2 blocks | Load error: Pi exits 1. Handler error: visible | Exit ≥ 2 with text: **deny** |
| Subagents | Tool hooks, `SubagentStart`, `SubagentStop`; no prompt hook | Tool and prompt hooks, `SubagentStart`, `SubagentStop` | No built-in subagents; depends on the extension that spawns them | Tool and model hooks only |
| Cap on injected text | 10,000 characters per string | About 2,500 tokens per message (configurable) | None found | None found |

### Shared conclusions for the design

- **The fallback is weakest where it is most needed.** It depends on the skill
  loading: on every harness only the description is always in context; on Pi
  the model must `read` the `SKILL.md`, and "models don't always do this"
  (Pi docs/skills.md); on Gemini activation also needs the user's click. Hooks
  beat it on Claude Code, on Pi, on Gemini where Gemini is used, and on Codex
  only once a person has trusted them. [D][S][I]
- **One adapter per harness, one shared core, is what the payloads allow.** The
  four harnesses agree on stdin JSON with `session_id` and `cwd`, on
  `hookSpecificOutput.additionalContext` (Claude Code, Codex, Gemini), and on
  short-lived processes, but differ in event names, tool names and payloads
  (absolute path, patch text, `returnDisplay.filePath`), timeout units
  (seconds, seconds, none, milliseconds), and failure semantics. Pi alone runs
  code in process. [S][I]
- **Codex reads Claude-format plugin directories and substitutes
  `${CLAUDE_PLUGIN_ROOT}`, and its `apply_patch` answers to the matchers
  `Write` and `Edit`.** One `hooks.json` could in principle serve both, with
  the script telling the harnesses apart, but Codex rejects Claude-style output
  on `Stop`, and trust is still required. Untested. [S][I]
- **Write the log outside every harness's plugin data directory.** Claude Code
  deletes `CLAUDE_PLUGIN_DATA` on uninstall; Codex's hooks may write anywhere
  because they run outside the sandbox; Pi's extension runs as the user. A
  harness-neutral state directory survives all four. [D][S][I]
- **Identifiers for the log**, as each harness provides them: Claude Code
  `session_id`, `agent_id`, `agent_type`; Codex `session_id` (the parent's, in
  subagents), `turn_id`, `agent_id`; Pi `ctx.sessionManager.getSessionId()` and
  the session file; Gemini `session_id`. None marks a turn as human, scheduled
  or exempted, and Claude Code's `UserPromptSubmit` cannot tell those turns
  apart. **No harness offers a signal that the human exempted an output**; that
  mark has to come from the model or the human. [S][I]
- **Checker interface facts that bear on every adapter**
  (`plugins/linkuistics/skills/technical-communication/scripts/ste-lint.py`,
  see `vendored-checker-k3`): exit 0 for hard findings at or under the
  baseline, 1 over it, 2 on a usage error, and 1 with a traceback on a file that
  is not UTF-8. It needs `python3`, so an adapter must survive a `PATH` without
  it. [S]

## 6. Walk-away checks

For each mechanism: with it removed or its harness's support uninstalled, what
still works, and what stays legible.

| Mechanism | What still works | What stays legible |
|---|---|---|
| `SKILL.md` and the checker | The checker runs from any shell; the skill is plain Markdown | Everything |
| Claude Code plugin `hooks/hooks.json` | Nothing runs; the skill goes too while the plugin is disabled | The JSON reads as a list of which event runs which script |
| Claude Code injected context | A resumed session replays it from the transcript | Plain text in `hook_additional_context` attachments |
| Claude Code `CLAUDE_PLUGIN_DATA` log | Deleted on uninstall by default | Nothing, if it was kept there |
| Codex plugin or `hooks.json` entries | The skill and the fallback remain | Trust entries stay in `config.toml` as inert leftovers; removing a non-final group un-trusts other tools' hooks |
| Codex `-c` session flags | Nothing persisted | Nothing to clean |
| Pi extension link | Pi starts normally; a dangling link is skipped silently | The link, by `ls` only |
| Pi extension present but broken | **Nothing: Pi will not start** until fixed or run with `-ne` | The red startup error |
| Pi reminder messages | Stay in past and resumed sessions | Custom messages in the session JSONL |
| `AGENTS.md` priming text (Codex, Pi) | Stays in every session, subagents included, with every extension off | The text, in a user-owned file that "survives extension removal" (earendil-works/pi#9432) |
| Gemini extension | Context, hooks and skills stop at the next start | Its directory; an entry in `extension-enablement.json` |
| Gemini hooks without the `\|\| true` wrapper | A missing interpreter **denies** turns and writes | The F12 debug drawer |
| Reply log in a neutral state directory | Survives every uninstall | Plain JSONL |
| Flag-file switch | Nothing reads it once the scripts are gone | An empty file whose name says what it did |

The mechanisms that are not walk-away safe are those that change the system
prompt and leave nothing behind (a forced Claude Code output style or plugin
`agent`), those that fail closed (a broken Pi extension, an unwrapped Gemini
hook), and text written into a user's own instruction file.

## 7. Search silence and limits

- **No live run of any hook in any harness.** Every behaviour above is from
  source, docs or reports, plus probes that made no model call. The open
  questions below list what only a run can settle.
- **Claude Code:** no Anthropic-side report that `SessionStart` hooks get an
  empty `PATH` (issue searches for "SessionStart PATH", "hook PATH",
  "dirname: command not found"; the changelog; the hooks page). No documentation
  of the runtime `{"async":true}` first-line protocol found in the bundle. The
  docs speak of workspace trust only for "hooks from settings files"; that it
  covers plugin hooks rests on the bundle's global gate.
- **Codex:** no environment variable that disables hooks; no supported command
  to trust a hook; no manifest field for instructions; no cap on `Stop`
  continuations; no changed-paths field for `apply_patch`. The docs do not
  mention `trusted_hash`, the key format, `codex exec`'s trust behaviour or the
  hooks' relation to the sandbox. No issue found about async `Stop` hooks killed
  at `exec` exit, or about `PostToolUse` missing for shell-routed patches.
- **Pi:** no handler timeout or watchdog in 0.84.2; no stable session-scoped
  prompt-append API; no per-extension enable flag; no documentation of why the
  TUI exits on end of input under `script` (the `tail -f /dev/null` recipe in
  the project memory), whose mechanism is inferred, not shown.
- **Gemini:** no issue about hook latency (only the best-practices line "slow
  hooks delay the agent loop"); no issue naming user hooks dropped in untrusted
  folders directly; no live evidence that `AfterTool` context makes the model
  revise a file; no cap on injected text found in source.
- **Not surveyed:** Antigravity CLI; how Ravel launches each harness (it lives
  outside this repository); the Agent SDK's loading of installed marketplace
  plugins.

## 8. Open questions for the live checks

**Claude Code (`claude-code-always-on-k11`):** does each plugin hook fire and
reach the model (`claude --debug-file` shows `(plugin linkuistics) finished
with status 0`)? Inside a subagent, does `PostToolUse` context go to the
subagent? Does `SubagentStart` also fire for forks, doubling the instruction?
Is `last_assistant_message` absent when a turn ends on a tool call? On which
non-human turns does `UserPromptSubmit` fire in practice? Does the model act on
`additionalContext` findings as reliably as on a block? Does a synchronous
30 ms `Stop` hook show at turn end?

**Codex (`codex-always-on-k12`):** does the session-flag trust pattern make a
hook actually run in `codex exec` and in the TUI? Does a newly installed
plugin's hook raise the startup modal, and does "Continue without trusting"
persist anything? Where does the `PostToolUse` developer message sit relative
to the patch result? Does the current model ever route `apply_patch` through
the shell? Does an async `Stop` hook complete on the last turn of `exec`?

**Pi (`pi-always-on-k13`):** does a `before_agent_start` system-prompt addition
reach every request of a multi-tool run and of follow-ups queued from
`agent_end`? Is the `agent_end` record always written before `pi -p` exits? Is
`pi --mode rpc --no-session -ne -e ./index.ts < /dev/null` a usable load check?
Does a child spawned by the bundled subagent example run the gate?

**Gemini (`gemini-always-on-k14`), if it goes ahead:** in a folder never
trusted, does a user-settings hook run (source says no) and an extension hook
(source says yes)? Does the `|| true` wrapper neutralise a missing script? Does
`prompt_response` include text from before tool calls? Do symlinked skills in
`~/.gemini/skills` load, and with what conflict warnings?

## Where the findings landed

The design adopted these findings, and two of its decisions are ADRs:

- Findings 6 and 7 are in
  [documents are gated, and replies are measured](../adr/documents-are-gated-replies-are-measured.md):
  the document gate uses the quiet channel, and no hook holds back a reply.
- Finding 9 is in
  [the off file is the only switch](../adr/the-off-file-is-the-only-switch.md).

The other findings landed in the
[spec](../specs/technical-communication.md), under *Harness mapping*. Finding 2
is there as the decision that this repository never grants Codex trust, and
finding 4 is there as the decision that Gemini CLI gets the skill only.

## Handoff

This survey decides nothing; these are the decisions it hands on, with the
evidence each rests on.

- **For `skill-design-k5`, and for the human:**
  - *Codex trust.* The choice is between asking the human to trust stable,
    thin hook entries once (the hash ignores script contents, so trust survives
    script changes but not entry changes), injecting hooks and trust per launch
    through `-c` session flags for Grove's own launches, or leaving Codex with
    the skill and the fallback. Writing trust into the user's `config.toml`
    would use internal state and bypass the boundary trust exists to be. This
    touches the brief's question of whether this repository starts managing
    the human's files.
  - *Gemini scope.* Whether "Gemini" in this grove still means Gemini CLI,
    given its README's transition notice and that it is not installed here. If
    not, `gemini-always-on-k14`'s own done-when already allows "nothing beyond
    the skill".
  - *What `plugins/install.sh` or Grove's provisioning delivers beyond skill
    links*: a Pi extension link, a Codex plugin, a Gemini extension.
  - *Where the always-on text lives on Codex and Pi*, given that text in
    `AGENTS.md` is outside every switch.
- **For the always-on leaves:** the per-harness sections above give the
  events, payloads, schemas, defaults to override and pitfalls; the open
  questions in section 8 are the live checks.
- **Not in this tree:** a durable, version-pinned guide to making a skill
  always on across harnesses, distilled from this survey and the always-on
  leaves' live evidence, is Linkuistics/grove#21.

## Where these findings landed

- [Documents are gated, and replies are measured](../adr/documents-are-gated-replies-are-measured.md)
  rests on findings 6 and 7. The gate uses the quiet channel, and the reply log
  prints nothing.
- [The off file is the only switch](../adr/the-off-file-is-the-only-switch.md)
  rests on finding 9.
- The spec [`technical-communication`](../specs/technical-communication.md)
  carries the layer map, the fail-open recipe for each entry, the Codex trust
  constraint, the Pi extension and Gemini's scope. It also records the human's
  answers to the handoff's questions. This repository never writes the human's
  global instruction files. Codex gets a hooks plugin that the human trusts
  once. Gemini CLI gets the skill only. `plugins/install.sh` also links the Pi
  extension.
