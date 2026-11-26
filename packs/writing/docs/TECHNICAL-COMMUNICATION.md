# Technical communication

`technical-communication` is a skill for the model. It makes the model write
in controlled English. The skill is modelled on ASD-STE100. It covers the replies
you read, the documents the model writes for people, and the text the model
writes for other models.

The skill is an experiment. It is on from installation, and one file turns it
off. A log of scores shows how well the model follows it. You judge the result
by using it.

The decisions behind the skill are in
[the spec](specs/technical-communication.md). The skill itself is in
[`plugins/writing/craft/technical-communication/`](../plugins/writing/craft/technical-communication/SKILL.md).

## What it does

The skill has two parts. The first part is the rules. The second part is the
always-on layers, which put the rules in front of the model in every session.

The rules cover sentence length, plain words, short noun groups, active verbs,
procedures, warnings, punctuation and writing habits. They also set British
spelling. Your own request always comes first. If you ask for a different form,
the model uses it.

The skill leaves code, commands, tool output, quotations, file names and
identifiers alone. Licences and released history stay as they are.

Four layers run on a harness that allows hooks.

| Layer | What it does |
|---|---|
| Instruction | Tells each new session and each subagent to write by the skill |
| Reminder | Marks each prompt as one that the skill covers |
| Document gate | Checks each prose file the model writes, and reports findings to the model |
| Reply log | Scores each reply and appends the counts to a log |

The gate checks documents only. No layer holds a reply back. You see a reply
before any check can run on it, and a gate would change what the log measures.
The log scores replies instead.

## Where each harness stands

| Harness | What you get |
|---|---|
| Claude Code | The skill and all four layers |
| Codex | The skill. All four layers once you trust the hooks plugin |
| Pi | The skill and all four layers, through an extension |
| Gemini CLI | The skill only, when its user points Gemini CLI to the skill directory |

Where no layer runs, the model checks a document itself. The skill tells it to
run the checker before it sends a document.

## Install

**The skill.** Install Grove with Homebrew. Grove's formula carries this
pack, and Ravel delivers the skill to each session that it starts.

```sh
brew tap linkuistics/taps
brew trust linkuistics/taps
brew install grove
```

Homebrew 6.0.0 and later do not trust a tap that `brew tap` adds. Grove's
formula loads its dependency `ravel` from the same tap, so trust the tap
before the first install. Ravel 2 delivers the skill, and Ravel 1.x delivers
none.

`brew upgrade` keeps the skill current. The [README](../README.md#install) of
this repository states the details. The layers are in hook adapters, which
you register in each harness yourself, as follows.

**Claude Code.** Run these commands once:

```sh
claude plugin marketplace add "$(brew --prefix)/opt/grove"
claude plugin install technical-communication-hooks@writing-hooks
```

The hooks plugin holds no skill. Start a new session. The layers are on. The
plugin runs from Grove's keg through the stable `opt` path, so an upgrade of
Grove needs no new command.

**Codex.** Nothing in this repository writes trust into your configuration.
Run these commands after each installation or upgrade of Grove:

```sh
# 1. Add the marketplace of Grove's keg.
codex plugin marketplace add "$(brew --prefix)/opt/grove"

# 2. Install the plugin. This copies the skill directory into Codex's cache.
codex plugin add technical-communication@writing-hooks
```

Then start `codex`. Codex shows a "Hooks need review" prompt. Choose "Trust
all and continue". You can also run `/hooks` and trust each
`technical-communication` entry. Do not choose "Continue without trusting".

Before you trust the plugin, Codex has the skill and no layer runs.
`codex exec` skips an untrusted entry and says nothing.

Codex stores the path of the versioned keg, and its trust covers the bytes of
the plugin. After an upgrade of Grove, run both commands and the review again.

**Pi.** Run this command once:

```sh
ln -s "$(brew --prefix)/share/ravel/skills/writing/craft/technical-communication/harnesses/pi/technical-communication.ts" ~/.pi/agent/extensions/
```

The link resolves through Homebrew's shared directory each time Pi loads the
extension. Restart Pi once after the link first exists. Pi asks for no trust.
To start Pi without any extension for one run, use `pi -ne`.

**Gemini CLI.** No layer runs there. Ravel starts no Gemini CLI session, so
Gemini CLI finds no skill by itself. Its user can point it to the skill
directory that `ravel skills resolve` lists.

## Turn it off and on

One file is the switch. It is `technical-communication/off` in
`$XDG_STATE_HOME`, which is `~/.local/state` when the variable is not set.

```sh
# Off
mkdir -p ~/.local/state/technical-communication
touch ~/.local/state/technical-communication/off

# On
rm ~/.local/state/technical-communication/off
```

The content of the file does not matter. You can also ask the model to create
or delete it.

Each layer reads the file at its next event. Then it does nothing and writes
nothing. A running session keeps the instruction it already holds until the
session ends. The skill tells the model to stop applying the rules in that
session when you turn the switch off.

The skill stays installed while the switch is off. It loads only when the
instruction is in context, or when you ask for controlled English.

## An optional line for `AGENTS.md`

The layers cannot reach a global instruction file. If you want the rules in a
harness with no layer, you can paste this line into your own global `AGENTS.md`.
This repository never writes that file.

```text
Write natural language by the technical-communication skill, unless the file technical-communication/off exists in $XDG_STATE_HOME (default ~/.local/state).
```

The line names the off file, so the switch still works there.

## Read the reply log

The log is in the state directory, with one file for each session:

```text
~/.local/state/technical-communication/sessions/<harness>--<session>.jsonl
```

A subagent writes into its parent's file. Each line is one JSON record. A
*start* record marks each time a harness sends the instruction. A *reply* record holds
the counts for one reply.

The log keeps scores and counts only. It stores no reply text and no matched
fragment. A reply with no prose words writes no record.

| Field | Meaning |
|---|---|
| `harness`, `model`, `session`, `agent` | Where the reply came from. `agent` is null for the main thread |
| `seq` | The position of the reply in the file |
| `since_start` | The position of the reply since the last start record. A compaction resets it |
| `words` | The prose words scored |
| `hard`, `advisory` | The count of findings at each level |
| `per_100` | Hard findings for each 100 words |
| `rules` | A count for each rule, at both levels |
| `exempt` | `true` when you asked for another form |
| `checker` | A short hash of the checker, so a changed rule set is visible |

Advisory findings are passive voice and compound tenses. They match word
shapes, not grammar, so `per_100` leaves them out.

These commands answer the questions of the experiment. They need `jq`.

```sh
LOG=~/.local/state/technical-communication/sessions

# Hard findings for each 100 words, for each harness and model
cat $LOG/*.jsonl | jq -s '[.[] | select(.kind=="reply")]
  | group_by([.harness, .model])
  | map({harness: .[0].harness, model: .[0].model, replies: length,
         hard_per_100: ((map(.hard) | add) * 100 / (map(.words) | add))})'

# Does compliance fade as a session goes on?
cat $LOG/*.jsonl | jq -s '[.[] | select(.kind=="reply")]
  | group_by(.since_start)
  | map({since_start: .[0].since_start, replies: length,
         hard_per_100: ((map(.hard) | add) * 100 / (map(.words) | add))})'

# Which rules fail most?
cat $LOG/*.jsonl | jq -s '[.[] | select(.kind=="reply") | .rules | to_entries[]]
  | group_by(.key)
  | map({rule: .[0].key, count: (map(.value) | add)})
  | sort_by(-.count)'
```

To judge a false alarm, run the checker on the reply again. The log cannot
show it to you.

## Check a text yourself

The checker needs Python 3.9 or later and nothing else. It reads files, or
standard input.

```sh
python3 plugins/writing/craft/technical-communication/scripts/ste-lint.py README.md
echo "Run the tests before you merge." | python3 plugins/writing/craft/technical-communication/scripts/ste-lint.py
```

The checker uses patterns, not a parser. It can raise a false alarm. When a
finding is wrong, record it as a failure (below).

## Report a failure

Log each failure that you see in use as an issue on
[`Linkuistics/WritingSkills`](https://github.com/Linkuistics/WritingSkills/issues). Say which
harness and model you used, what the model wrote, and what you expected. A later
grove reads these issues and acts on them.
