# technical-communication

## Problem

The human wants every piece of natural language that a model writes to be in
controlled English modelled on ASD-STE100. That covers replies to the human,
documents for people, and text for other models, on Claude Code, Codex and Pi.
A skill alone loads only when a task matches its description, so it cannot
cover every reply. The rules must therefore reach each session from outside the
skill. The result must also be measurable, because the human runs it as an
experiment, and one switch must turn all of it off.

## Solution

The skill has five layers. Two are text and three are code.

| Layer | What it is | When it acts |
|---|---|---|
| Skill | `SKILL.md` and its references: every rule, the procedures, the switch | When the model loads it |
| Always-on text | A short instruction and a shorter reminder, independent of any harness | Session start, and each prompt |
| Checker | The vendored `ste-lint.py`, extended: lint, markers, scoring, log | Each write of a prose document, each reply, and on request |
| Hook core | One script that reads the off file, then serves the harness's events | On every hook event |
| Harness entries | One thin, stable file for each harness that maps its events to the core | Loaded by the harness |

The always-on layer makes four things happen. The instruction reaches each new
session and subagent. The reminder qualifies each prompt. The document gate
checks each prose file that the model writes. The reply log scores each reply.
[Documents are gated, and replies are measured](../adr/documents-are-gated-replies-are-measured.md)
records why replies are never gated. [The off file is the only
switch](../adr/the-off-file-is-the-only-switch.md) records the switch. The
skill carries no text from the standard
([technical-communication carries no text from the standard](../adr/technical-communication-carries-no-standard-text.md)).

The precedence order is the human's. From the highest: (1) the human's
explicit request for one output. (2) Meaning: no rule may cost information, a
hedge or a caveat, or add a fact. (3) This skill's rules for words and
sentences. (4) House style, for everything the rules leave alone. Spelling is British, with
the -ise forms. Lazy conversion applies: an edit to an existing prose document
converts the whole document first, as its own change.

## Decisions

### Layers and layout

Everything lives in one skill directory, in the `craft/` root of the writing
plugin. That directory is also the root of the two hooks plugins. The
marketplace files that list the hooks plugins are in Grove's Homebrew keg.

```text
plugins/writing/
└── craft/technical-communication/
    ├── SKILL.md                     the skill
    ├── references/                  one file for each section of the standard's Part 1
    ├── always-on/
    │   ├── instruction.md           sent at session and subagent start
    │   └── reminder.md              sent with each prompt
    ├── scripts/
    │   ├── ste-lint.py              the checker (vendored, then extended)
    │   ├── tc-hook                  POSIX sh guard: finds python3, always exits 0
    │   └── tc_hook.py               the hook core
    ├── .claude-plugin/plugin.json   Claude Code hooks plugin manifest, with "skills": []
    ├── hooks/hooks.json             Claude Code entry (the plugin root is required)
    ├── .codex-plugin/plugin.json    Codex manifest; the skill directory is its root
    ├── harnesses/
    │   ├── codex/hooks.json         Codex entries
    │   └── pi/technical-communication.ts   Pi extension
    └── LICENSES/                    the vendored checker's MIT notice
```

- **Claude Code's entry is at the root of the hooks plugin**
  `technical-communication-hooks`, because Claude Code loads always-on plugin
  hooks only from `hooks/hooks.json` there or from `plugin.json`. A skill's
  frontmatter hook lasts only while that skill is active. The root of the hooks
  plugin is the skill directory. Its manifest lists `"skills": []`, so the
  plugin adds no second copy of the skill. The writing plugin declares the hooks
  plugin as a dependency and declares no hook. No route installs the writing
  plugin, and `task check:hooks` checks that each hook still runs once in each
  combination of the two plugins.
- **The Codex plugin's root is the skill directory itself.** Its entries then
  reach the scripts through `${PLUGIN_ROOT}/scripts/`, and they work from
  Codex's cache, which holds a copy of the whole directory. The directory has no
  `skills/` subdirectory, so the plugin adds no second copy of the skill.
- **Each harness adds a plugin only from a marketplace.** Grove's keg holds
  both marketplace files at its root, with the name `writing-hooks`. Each file
  lists one plugin, with the skill directory in the keg's `writing` source as
  its source. Codex resolves a `local` source only inside its marketplace root,
  so this repository holds no marketplace file. Codex stores the path of the
  versioned keg, and its cache copy is a snapshot. After an upgrade of Grove,
  the human adds the marketplace and the plugin again and repeats the hook
  review. Until then the cached scripts run, and a cached script that lacks a
  newer harness name does nothing.
- **The Pi extension finds the skill directory through its own real path.**
  The documented command links it into `~/.pi/agent/extensions/` from
  Homebrew's shared skill directory, so its loaded path is the link.
- **The rules are laid out for progressive disclosure.** `SKILL.md` holds what
  the model needs for most text. That is the scope and the exclusions, the
  precedence order, spelling, and the text types with their limits. It also
  holds the core rules, a one-line index of every rule with its identifier, the
  conversion procedure, the markers, the checker and the switch.
  Each reference file restates one section of Part 1 in full, with invented
  examples. The model reads a reference when the task needs it: `procedures.md`
  for steps, `safety.md` for a warning, `words.md` for a choice of word.
  The nine files are `words.md`, `noun-clusters.md`, `verbs.md`,
  `sentences.md`, `procedures.md`, `descriptions.md`, `safety.md`,
  `punctuation.md` and `practices.md`, in the order of the standard's sections.
- **Every rule has one identifier and one owner.** The identifier is a short
  kebab-case name. An enforced rule uses the checker's identifier for that
  rule. `SKILL.md` owns every rule. The instruction repeats only the core rules,
  each with its identifier, and states no rule that `SKILL.md` lacks.
  Identifiers are stable once the log exists, because the log keys its counts by
  them.
- **The description loads the skill only on a signal.** It names the capability.
  It says to use the skill when the always-on instruction is in context, or
  when the human asks for controlled English. So with the layers off, ordinary
  writing does not load it.

**Size budgets.** The always-on text costs context in every session, and the
reminder costs it again with each prompt.

| Text | Budget | Reason |
|---|---|---|
| `instruction.md` | 2,500 characters | Once for each start and each subagent. Well under Claude Code's cap of 10,000 characters and Codex's cap of 2,500 tokens |
| `reminder.md` | 200 characters | Kept in the transcript with every prompt on each harness that persists it |
| A gate message | 1,500 characters | Once for each write that has findings |
| `SKILL.md` | 300 lines, and never more than 500 | The house rule for progressive disclosure |
| Each reference file | 300 lines, or a table of contents above that | The same rule |
| The description | 400 characters | In every session's context on every harness |

**What the instruction carries**, in this order:

1. The session writes its natural language by this skill.
2. What is outside the skill.
3. The precedence order, with its markers.
4. The core rules, each with its identifier.
5. British spelling.
6. Documents are checked after each write. Where no gate runs, the model runs
   the checker itself.
7. The skill holds the rest.

The instruction states facts about the session, not commands. Claude Code's
defences against prompt injection can show hook text that reads as a command to
the human, instead of using it. The reminder is one sentence that qualifies the
human's message. Codex places it after the prompt, and a model can take a bare
directive there as the task.

### The switch

The switch is the off file, `technical-communication/off` in
`$XDG_STATE_HOME`, by default `~/.local/state`. The human creates it to turn the
layers off and deletes it to turn them on, or asks the model to do either. Its
content does not matter. [The off file is the only
switch](../adr/the-off-file-is-the-only-switch.md) records the choice. The
variable `TECHNICAL_COMMUNICATION_STATE_DIR` moves the whole state directory for
a test run. It is a location, not a second switch. The state directory is
`technical-communication` in `$XDG_STATE_HOME`: it holds `off` and `sessions/`.
The variable names that directory itself, so with the variable set the off file
is `$TECHNICAL_COMMUNICATION_STATE_DIR/off`. A value of `$XDG_STATE_HOME` that is
not an absolute path counts as unset.

| Layer | Effect of the off file |
|---|---|
| Instruction | Not sent to a session or subagent that starts afterwards |
| Reminder, gate, reply log | Stop at the next event: the core exits 0, prints nothing and writes nothing |
| Text already in a running session | Stays until that session ends. Nothing retracts it. When the human turns the switch off in a session, `SKILL.md` tells the model to stop applying the rules there too, because the human's request is precedence 1 |
| The skill | Stays installed. Its description loads it only on the instruction or an explicit request |
| A line the human pastes into a global `AGENTS.md` | Obeys the switch only because the offered line names the off file |

### The reply log

The log lives in the state directory as one JSON Lines file for each session:
`sessions/<harness>--<session>.jsonl`. Characters outside `[A-Za-z0-9._-]` in
the session identifier become `_`, and a missing identifier is `unknown`. A
subagent writes into its parent's file, because the harness gives it the
parent's session identifier, and its records carry its own `agent`. To read the
whole log: `cat sessions/*.jsonl | jq …`.

Each record is one line. Every record has `v` (1), `kind`, `ts` (UTC, ISO 8601,
to the second), `harness` (`claude-code`, `codex` or `pi`), `session` and
`agent` (null for the main thread). The record keeps the session identifier as
the harness gave it. Only the file name is cleaned. A missing `harness` or
`session` is `unknown`, in the record and in the file name.

- **A start record** (`kind: "start"`) is written each time the instruction is
  sent. It adds `source`: the harness's own value (`startup`, `resume`,
  `clear`, `compact` or `fork`), or `subagent`. Pi sends the instruction with
  every prompt, so it writes a start record only when a session starts, with the
  source `startup`, `resume` or `fork`. Pi's compaction keeps the system prompt,
  so on Pi it does not reset `since_start`.
- **A reply record** (`kind: "reply"`) adds `model` (null when the harness does
  not give it), `seq`, `since_start`, `words`, `hard`, `advisory`, `per_100`,
  `rules`, `exempt` and `checker`.

| Field | Meaning |
|---|---|
| `seq` | The position of this reply among this agent's reply records in the file, from 1 |
| `since_start` | The position since this agent's last start record, from 1. It equals `seq` when there is no start record. A start record of another agent does not reset it |
| `words` | Prose words scored. An exempt reply is scored too |
| `hard`, `advisory` | Counts of findings at each level |
| `per_100` | Hard findings per 100 words, to one decimal place |
| `rules` | A count for each rule identifier, at both levels |
| `exempt` | `true` when the reply carries the exemption marker |
| `checker` | The first 12 hex digits of the SHA-256 of `ste-lint.py`, so scores from a changed rule set are visible as such |

These fields answer the experiment's questions. `seq` and `since_start` show
whether compliance fades within a session, and a compaction resets
`since_start` because it sends the instruction again. `harness` and `model`
show whether harnesses differ. `ts` and `checker` show whether replies improve
over time and across versions of the rules. Advisory findings (passive voice
and compound tenses) are counted in `rules` and `advisory` but not in
`per_100`, because they match word shapes, not grammar. A faulty `exempt` marker
in a reply is a hard `marker` finding, like any other hard finding.

**Where `model` comes from.** Codex's hook payloads carry the model. Claude
Code's hook payloads do not, so its adapter reads the model of the latest
assistant entry at the end of the transcript. A subagent has its own transcript.
When the read fails, the record holds null. The transcript can lag the reply, so
the first reply after a model change may carry the old name.

**The log stores no reply text and no matched fragment**, and not the reason in
an exemption marker. To judge a false alarm, the human runs the checker on the
reply again. A reply with no prose words writes no record, and a reply that
only holds code and an `exempt` marker has no prose words.

**The writer** is the checker. It takes an exclusive `flock` on the session's
file, counts the earlier records, appends one line and releases the lock. It
waits two seconds at most for the lock. When it cannot write, for any reason, it
writes nothing and exits 0. A lost record shifts `seq` for later replies, and
the design accepts that. The same holds for a bad argument: the two log modes
parse their own arguments and exit 0 on a usage error too. They skip a line of
the file that is not JSON when they count, and they start a new line first when
a crashed writer left the last one without its newline. They read the reply
as UTF-8 and replace bytes that are not, so a bad byte does not lose the score.
The file mode is 0600.

### The checker's contract

The checker is `scripts/ste-lint.py`: Python 3.9 or later, standard library
only. Every change to the vendored file is marked as ours. It checks only rules
that need no dictionary.

**Text types and limits.** A sentence is an instruction or a description.

- The limit is 20 words for an instruction and 25 words for a description. Both
  report as `long-sentence`.
- A paragraph has six sentences at most (`long-paragraph`). This applies to a
  paragraph, not to a list item, a table cell or a heading.
- The default mode is `auto`. In it, a sentence is an instruction in two
  cases: it is in an ordered-list item, or its first word is an imperative. An
  imperative is a base-form verb from the checker's own list, or "do not",
  "never" or "always". A leading clause that starts with "if", "when",
  "before", "after" or "once" and ends with a comma is skipped first.
- `--text-type instruction` or `description` sets one type for all the text.
  `--text-type commit` reads a commit message (below).
- A word is a token that has at least one letter or digit. Markup tokens such as
  `#`, `-`, `|` and `---` are not words.

**What counts as prose.**

- *In Markdown* (`.md`, `.markdown`), prose is the text of paragraphs,
  headings, list items, table cells and block quotes. The text of links and
  emphasis is prose too. These are not prose: fenced code, inline code, link and
  image targets, reference definitions, bare URLs and autolinks. HTML comments
  (which hold the markers) and HTML tags are not prose. Front matter (in `.md`
  and `.markdown` files only) is not prose, except the value of a `description`
  key. A fence closes only on a fence
  of its own kind and at least its own length. A span in double quotation marks
  on one line is a quotation, and it is masked as code is. Text inside an `off`
  or `pending` region is not checked.
- *In a commit message* (`--text-type commit`), the subject and the body are
  prose, checked as descriptions. A leading `<token>:` on the subject is an
  identifier, and the final block of `Key: value` trailers is not prose.
- *In a code comment*, the words are prose for the model, which writes them by
  the rules. The checker does not scan source files, and the gate does not check
  them. Comment syntax differs by language, and a scan would give false alarms on
  commented-out code, licence headers and directives. The model can check a long
  comment by giving it to the checker on standard input.
- *In a plain-text file* (`.txt`), every line is prose, read by the Markdown
  rules.

**British spelling without a dictionary.** The rule `spelling` (hard) finds
American forms with suffix patterns and a short word list that the checker
owns. The patterns and the list cover:

- -ize and -ization, with exceptions such as "size", "prize" and "seize"
- -yze
- the -or words that are -our in British English
- the -er words that are -re in British English
- the -ense nouns
- the single-l forms that British English doubles, such as "labeled" and
  "modeling"
- a few single words

The rule flags only a form that British usage does not also accept. So it
leaves "program", "disk", "dialog", "meter", "license", "practice" and
"judgment" alone. Code, identifiers (a token with an underscore, a dot, a slash
or a leading hyphen), words with internal capitals and words in capitals only
are not checked. A word that starts with a capital inside a sentence is a name
and is not checked. Hedges ("may", "might", "could") are never flagged.

**Markers.** A marker is an HTML comment, so rendered Markdown hides it. Its
grammar is `<!-- tc: VERB [RULES] because REASON -->`.

| Verb | Use | Scope |
|---|---|---|
| `exempt` | The human explicitly asked for this output in another form (precedence 1) | A document: not checked. A reply: scored, and logged with `exempt: true` |
| `depart RULE[,RULE]` | No wording keeps both the rule and the meaning (precedence 2) | The block that holds the marker. A marker alone on its line applies to the next block. Block means a paragraph, list item, table row or heading |
| `off` | Text that is not ours to change: a quotation, vendored text, released history | To the next `on`, or to the end of the file |
| `pending` | Text that a large document's conversion has not reached | To the next `on`, or to the end of the file |
| `on` | Ends the open `off` or `pending` region | Takes no rules and no reason |

A departure accepts only the findings of the named rules in its scope. A marker
that lacks `because` and a reason, names an unknown verb or rule, or closes
nothing is a hard `marker` finding, and it does not apply. Two more cases are
faults of the same kind: a `depart` marker alone on its line with no block after
it, and an `off` or `pending` marker while another region is open. So a
justified departure is accepted and a bare one is not. A valid `exempt` marker
anywhere in a document makes the whole document unchecked. In a reply, only
`exempt` has an effect: the checker ignores every other verb there, faulty or
not.

**Rule identifiers.** The vendored rules keep their identifiers, except that
`nominalization` becomes `nominalisation` for the British spelling. New rules
are `long-paragraph`, `spelling` and `marker`. The levels are `hard` and
`advisory`. `passive-voice` and `present-perfect` are advisory and never fail a
check. Messages name this skill's rule, and they never cite the standard's
numbering or name.

**Modes, output and exit codes.**

| Invocation | Output | Exit |
|---|---|---|
| `ste-lint.py [--json] [--text-type T] [--baseline N] [--disable R,…] [FILE…]` | Findings, then a summary. Reads standard input when no file is named | 0, 1 or 2 |
| `ste-lint.py --markers FILE…` | Each marker with its line, verb, rules and reason, and each faulty marker | 0, 1 or 2 |
| `ste-lint.py --log-start --harness H --session S [--agent A] --source X` | Nothing. Appends a start record | Always 0 |
| `ste-lint.py --log-reply --harness H --session S [--agent A] [--model M]` | Nothing. Scores the reply on standard input and appends a reply record | Always 0 |
| `ste-lint.py --selftest` | `selftest OK` | 0, or 1 on a failed assertion |

- Exit 0: no unaccepted hard findings above the baseline. Exit 1: more
  unaccepted hard findings than the baseline allows. Exit 2: a usage error, or a
  file that cannot be read or is not UTF-8. Text that is not UTF-8 is an exit 2,
  not a traceback, so exit 1 always means findings.
- Text output prints `FILE:LINE:COL RULE: MESSAGE` for each unaccepted
  finding. One summary line follows. It gives the findings at each level, the
  accepted departures, the pending regions, the words, and the hard findings
  per 100 words. The message of a rule that matches a word or phrase names it,
  so a gate message needs no other field.
- `--json` prints one object: `findings` (each with `file`, `line`, `col`,
  `rule`, `level`, `match` and `message`), `hard`, `advisory`, `accepted` (a
  count for each rule), `pending` (the number of pending regions), `words`,
  `per_100`, `exempt`, `text_type` and `baseline`. Only this command-line
  output carries `match`. The log never does.
- `--markers` prints one line for each marker, `FILE:LINE VERB [RULES] [because
  REASON]`, and one line `FILE:LINE:COL marker: MESSAGE` for each faulty
  marker. With `--json` it prints `{"markers": [...]}`, each marker with `file`,
  `line`, `col`, `verb`, `rules`, `reason` and `error` (null when valid). It
  exits 1 when any marker is faulty.
- The hook core does not read exit codes. It loads the checker by path and calls
  the same functions in its own process. The function is
  `analyse(text, filename, text_type, reply)`. It returns a dict with
  `findings` (the unaccepted ones), `accepted`, `pending`, `words`, `exempt`,
  `markers`, `first_prose_line` (the first line that holds prose, so a gate can
  tell whether a first marker comes before any prose), `unclosed` (the verb of
  an `off` or `pending` region that the end of the text leaves open, or null)
  and `text_type`. `lint(text, filename, text_type)` returns the findings and
  the word count. Neither function writes anything. With `reply` true, a valid
  `exempt` marker sets `exempt` and the reply is still scored. With `reply` false
  it makes the document unchecked.

### The document gate and conversion

**Which files the gate checks.** A prose document is a file whose name ends in
`.md`, `.markdown` or `.txt`, in any letter case. The gate skips:

- A path inside a temporary directory (`$TMPDIR`, `/tmp`, `/private/tmp`,
  `/var/folders`), or inside `.git` or `.jj`.
- Licences and legal notices: any path inside a `LICENSES` directory, and file
  names that start with `LICENSE`, `LICENCE`, `COPYING` or `NOTICE`.
- Vendored text: any path inside `vendor`, `third_party` or `node_modules`.
- Historical records: grove task files with a `DONE` or `ABANDONED` infix.
- Generated files: `@generated` or `DO NOT EDIT` in the first five lines.
- A file whose first marker, before any prose, is `off` or `exempt` with no
  closing `on`.

Quotations and released history inside a checked file take `off` regions. The
same classes never convert.

**What the gate does.** On each successful write through the harness's tools
for files, the hook core finds the written paths. It checks each prose document
as a whole, in `auto` mode. When a file has unaccepted hard findings,
the core returns one added-context message of 1,500 characters at most. The
message names each file with its count of findings, lists up to ten findings
(`LINE:COL RULE: MESSAGE`), and says how many more there are. One line tells the
model to fix the findings, or to mark a justified departure, before it
finishes. The same line says to convert the document first if it was not
converted. A file with no unaccepted
hard findings gives no message. Advisory findings never reach the gate's
message. No harness's gate sees a file that is written through the shell, so the
skill tells the model to check such a file itself.

**Lazy conversion.** Before the model edits an existing prose document that is
not in a never-convert class, it runs the checker on the whole document. When
the document is clean, or all its findings are accepted, the model makes the
edit. Otherwise:

1. **Convert.** The model converts the whole document to the rules. Meaning wins
   over a rule: when no wording keeps both, the model marks a departure.
2. **Large documents convert in part.** A document is too large for one
   session when it has more than 3,000 prose words or more than 60 hard
   findings. Then the model converts the sections that the edit touches and puts
   each other section in a `pending` region. A later edit converts more of it.
   These thresholds are starting values for field use to tune.
3. **Land the conversion first, as its own change.** Under jj,
   `jj split -m "<handle or summary>: convert <document> to controlled English" <document>`
   puts the converted document into its own parent change. Other in-progress
   work stays in the working copy on top, and this also works when the
   conversion is the only change. Under git,
   `git commit -m "…" -- <document>` commits that file alone. This was tested
   in a scratch jj repository with jj 0.46.0.
4. **Edit.** The model makes the requested edit in the next change. The gate
   checks it like any other write.

The model converts before it touches the document, so the conversion and the
edit never share a file's change. Some sessions may not create commits on their
own. In such a session, the model converts the document and stops. It asks the
human to land the conversion, or to allow it, before it makes the edit. In a grove session the conversion is a
preparatory commit before the task's own commit.

### Precedence in grove's own text

`grove-communication-reword-k15` edits grove's "Human-facing communication"
section (`plugins/grove/skills/grove/SKILL.md`) to these requirements.

- **It keeps what house style owns.** Start with the outcome or the question.
  Explain what was tried, why, what was seen and what it means. Build each idea
  on the one before. Introduce terms in plain language. Give the reader enough
  context to form an opinion, and the practical consequence of each limitation
  or trade-off. Keep the main explanation readable alone, with logs and evidence
  in a linked section. Before asking for a decision, explain what accepting the
  recommendation means. Let what needs explaining set the length.
- **It drops what pulls against the rules.** "Conversational" invites idiom and
  long sentences. It becomes "guided and explanatory". "Rather than terseness"
  can read as permission for long sentences. The section says instead that
  what needs explaining sets the length of an explanation, and that no word
  budget does.
- **It names the order once.** One sentence states the order. When the
  `technical-communication` skill is on, its rules govern words and sentences,
  and this section governs what to explain and in what order. The human's
  explicit request still comes first. Without the skill, the section
  binds as it stands. It restates no rule of the skill.
- **The plugin hub lists the dependency.** `references/grove.md` lists
  `technical-communication` by its bare name with the other craft skills that
  Grove cites, and it says that the section itself is what binds without it.
- **Grove's commit procedure allows the conversion commit.**
  `references/commit.md` gains one sentence. It says that a change another
  skill needs to land first, such as a document's conversion, is its own commit
  before the task's commit. The task's commit stays one focused commit.

The explanatory output style and its insight blocks are house style. The rules
govern the words inside them, and nothing in that plugin changes.

### Harness mapping

| Layer | Claude Code | Codex | Pi | Gemini CLI |
|---|---|---|---|---|
| Instruction | `SessionStart`, and `SubagentStart` for subagents | `SessionStart` and `SubagentStart`, once trusted | `before_agent_start`, added to the system prompt on each run | None: the skill only |
| Reminder | `UserPromptSubmit` | `UserPromptSubmit`, once trusted | `before_agent_start`, as a hidden message that Pi saves | None |
| Document gate | `PostToolUse` on `Write`, `Edit` and `NotebookEdit` | `PostToolUse` on `apply_patch`, with the paths read from the patch's headers | `tool_result` on `write` and `edit` | The model runs the checker |
| Reply log | `Stop` and `SubagentStop`, from `last_assistant_message` | `Stop` and `SubagentStop`, from `last_assistant_message`, once trusted | `agent_end`, from the run's last assistant message | None |
| Delivered by | The hooks plugin `technical-communication-hooks` from Grove's keg, which the human registers once | The hooks plugin from Grove's keg, which the human adds and trusts after each upgrade of Grove | A link that the human makes once through Homebrew's shared skill directory | Nothing. The human can point Gemini CLI to the skill directory |

- **Every entry fails open and quietly.** The core prints at most one complete
  JSON object, written once, and only when it adds context. For a reply it
  prints nothing, because any output from a stop hook continues the turn. It
  exits 0 on every path: when `python3`, the checker or the state directory is
  missing, and on any error. On Pi, every handler catches every error, and the
  extension's factory only registers handlers. It imports only Node's built-in
  modules, because a broken extension stops Pi from starting in every mode.
- **No entry writes inside the source.** The hooks run in place from an
  installed source, such as Grove's Homebrew keg, and Ravel digests every file
  of it. A file written there changes the source's `resourceDigest`. The core
  writes only in the state directory. `tc-hook` runs Python with `-I -B`, and
  the core sets `sys.dont_write_bytecode` before it loads the checker, so the
  import writes no bytecode into `scripts/__pycache__`. `scripts/hook-writes.py`
  runs each declared entry against a read-only and a writable copy of the
  source, and the writable copy's tree must not change.
- **Entries never change shape.** Codex trusts a hash of each entry's command,
  matcher and timeout, so a changed entry loses its trust, and `codex exec` then
  skips it without a word. Behaviour changes go into the scripts.
- **A Codex entry is a shell command string**, because Codex has no `args`
  field: `/bin/sh "${PLUGIN_ROOT}/scripts/tc-hook" codex EVENT || true`. Codex
  hashes the command before it fills in `${PLUGIN_ROOT}`, so the trust holds
  wherever the cache is. `|| true` covers a missing guard file, which would
  otherwise give a failed hook row on every event. The timeouts are 10 seconds
  for `SessionStart`, `SubagentStart`, `Stop` and `SubagentStop`, 5 for
  `UserPromptSubmit` and 15 for `PostToolUse`, whose matcher is `apply_patch`.
- **The Codex adapter needs no transcript.** Codex puts `model` in every
  payload. For the gate it reads the paths from the patch text in
  `tool_input.command`: the files of its `*** Add File:`, `*** Update File:`
  and `*** Move to:` headers, taken relative to `cwd`. A deleted file is not
  checked, and a content line of an added file cannot pass for a header.
- **The core takes the harness's own payload** as
  `tc-hook <harness> <event>`. The events are `session-start`,
  `subagent-start`, `prompt`, `write` and `reply`. Pi also uses `instruction`,
  which returns the instruction and writes no start record. The thin adapter for each
  harness lives in `tc_hook.py`: it reads that harness's payload and writes that
  harness's output. The Pi extension calls the same core or the checker as a
  child process, and it repeats no rule logic in TypeScript.
- **Codex runs no layer until the human trusts the plugin.** Until then, Codex
  has the skill, and the model runs the checker itself. `codex exec` skips an
  untrusted entry without a word. Codex's hooks run outside its sandbox, so the
  log write is not blocked there, even when the state directory is outside the
  session's writable folders.
- **The Pi extension sends one event for each Pi event, and holds no rule.** It
  runs `/bin/sh scripts/tc-hook pi EVENT` as a child process and writes a small
  JSON payload to its standard input. It kills the child at a timeout: 5 seconds
  for `instruction` and `prompt`, 10 for `session-start` and `reply`, and 15 for
  `write`. It uses `spawn` and never `spawnSync`, which would block Pi's
  rendering. The core answers with `{"additionalContext": TEXT}` or nothing.
  - `session_start` sends `session-start` and drops the answer. It writes the
    start record. A reload writes none, because the session stays the same.
  - `before_agent_start` sends `instruction` and `prompt` at the same time. It
    returns the system prompt with the instruction added, and the reminder as a
    hidden message. Pi clears the system prompt after each run, so every prompt
    sends the instruction again. Pi saves the reminder after the prompt, as the
    other harnesses do. The history then stays the same between requests, and
    the provider's prompt cache keeps working. A reminder added to the end of
    each request would move the cache breakpoint every time.
  - `tool_result` for `write` and `edit` sends `write` with the path of the call
    and adds the gate message to the result. `isError` stays as it is.
  - `agent_end` sends `reply` with the text and the model of the last assistant
    message. A run that ends in an error or an abort writes no record.
  - A file that the model writes through `bash` is not gated, and the model runs
    the checker on it. A run that Pi starts with `sendMessage` while it is idle
    skips `before_agent_start` in Pi 0.84.2, so it has no instruction.
- **Gemini CLI gets the skill only.** It is not installed here, and for
  unpaid-tier users Antigravity CLI replaced it. The extension layout in the
  [harness survey](../research/harness-hooks-a.md) remains for a later grove.
- **This repository never writes the human's global instruction files**, and it
  never grants Codex trust. The priming sections in `~/.codex/AGENTS.md` and
  `~/.pi/agent/AGENTS.md` are maintained outside this repository. Text there is
  outside every hook, and a trust grant written by a machine would bypass the
  boundary that trust exists to be. The user document offers one optional line
  that the human can paste. That line names the skill and says that it applies
  unless the off file exists.
- **No installer writes a hook registration.** Homebrew cannot write a
  harness's configuration. The human runs the documented commands, which the
  `grove` formula's caveats print: the Claude Code and Codex registrations and
  the Pi link. Grove's startup writes no skill or hook links. Snapshots that Grove 22.8 and
  earlier provisioned carry the hook scripts as supporting files, and nothing
  reads them there.

### Restatement discipline

These rules bind every leaf that writes the skill's text or its examples.

- **Read the standard only as page images**, with the `Read` tool's `pages`
  option. Do not extract its text by any tool. Keep working notes in the
  session's scratchpad, never in the repository or `.grove/`.
- **Write each rule from its idea, not from its sentence.** Close the pages
  before writing a rule's text. Give each rule this skill's own identifier and
  title. Do not reuse the standard's rule titles. Merge or split rules where that
  makes the skill clearer.
- **Invent every example, about software and model work**: code, commands,
  repositories, reviews, sessions. Never adapt an example from the standard,
  which is about other domains. The domain distance is itself a safeguard.
- **Copy no word list.** Word choice is stated as principles. A short pair of
  words in an example comes from general plain-English practice or from the
  vendored MIT checker, never from the standard's dictionary.
- **Cite only numbers.** A reference file may say which section of Part 1 it
  restates. For the coverage check, it may also list the rule numbers that it
  covers. It cites no title and no text.

**The sweep before publication.** `publication-k16` follows these steps. Their
results go into its running log, with no text from the standard.

1. **Enumerate the surface.** List every file that differs between `main` and
   the working copy (`jj diff --from main --to @ --summary`). Add every file in
   `.grove/`, and each file under `plugins/`, `docs/` or `.grove/` that the VCS
   ignores. The sweep covers that list, not a list of expected files.
2. **Read in pairs.** For each of the nine sections, open its pages as images
   beside the matching reference file and its lines in `SKILL.md`. For each rule
   and example, decide if it reproduces a sentence, a rule title, an example's
   scenario or a list in the same order. A phrase of five or more words that is
   not ordinary technical English counts too. Record a verdict for each rule.
3. **Check the examples' domain.** An example about aircraft, maintenance or
   hardware is checked against the pages and rewritten.
4. **Probe with phrases, with controls.** From the pages, type in the
   scratchpad three to five distinctive phrases of each section, eight words at
   most each. Search the enumerated surface for each phrase, case-insensitively,
   allowing any whitespace between words. First put one phrase into a scratch
   copy of a repository file and show that the search finds it. Then change one
   word of the phrase and show that the search does not. Only then credit a clean
   result. Delete the phrase list at the end of the session.
5. **Sweep for the trademark.** Search the same surface for the standard's full
   title, for its short code with the digits after it, and for the initials of
   its owner and of its title as whole words, matching case. Type each pattern
   in the scratchpad, so that this spec holds none of them. Classify each hit.
   Three classes are allowed: the phrase "modelled on ASD-STE100", the upstream
   repository's name `danyuchn/asd-ste100-skill` in provenance and attribution,
   and the licence file. Fix each other hit. A hit in the log of a finished leaf
   is a finding for the human, because that log records the human's own
   decisions and the VCS history keeps it anyway.
6. **Decide on a fresh read.** If a leaf that wrote the text asked for a
   fresh-context read in its commit message, run it here, within the session's
   reviewer allowance.

## Verification

No tests are built beyond the checker's own self-test, which runs in
`task check:tc-checker` (the human's decision in `plan-k1`). Each leaf shows its
behaviour by running it. The checker leaves run it on real documents and
replies. Each always-on leaf runs a real session and shows each layer and the
switch working.

`scripts/pi-extension-load.test.sh` is a Pi load check that uses no model
quota, and `task check:tc-pi` runs it. It starts Pi in RPC mode and reads the
start record that the extension writes. So it fails when the extension does not
load, or when it registers no working handler. It also fails when the off file
does not stop the record, or when Pi stops reporting a broken extension. It
checks that the extension loads, not how well the skill works.

## Out of scope

- **A reply gate.** See [Documents are gated, and replies are
  measured](../adr/documents-are-gated-replies-are-measured.md).
- **Gemini CLI hooks and an Antigravity CLI survey.** Gemini gets the skill
  only. A later grove can use the survey's extension layout.
- **Checking code comments and commit messages automatically.** No hook sees a
  commit message, and comment syntax differs by language. The skill tells the
  model to check a commit message, a pull request or an issue text with the
  checker before it sends it.
- **Retracting the instruction from running sessions.** The human chose the
  simpler switch, which acts at each layer's next event.
- **Reply text in the log.** The human chose scores and counts only.
- **A dictionary, or vocabulary control beyond principles.** See
  [technical-communication carries no text from the
  standard](../adr/technical-communication-carries-no-standard-text.md).
- **Any write to the human's global instruction files, or any trust grant.**
  See *Harness mapping*.
