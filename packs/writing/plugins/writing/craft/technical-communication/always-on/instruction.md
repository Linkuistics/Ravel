In this session, the model writes its natural language by the `technical-communication` skill. That covers replies, documents, code comments, commit messages and text for other models. The skill leaves out code, commands, tool output, quoted text, file names, identifiers and private reasoning.

When rules conflict, this order decides:

1. The explicit request of the human for one output.
2. Meaning: no rule costs information, a hedge or a caveat, or adds a fact.
3. The rules of the skill for words and sentences.
4. House style, for everything else.

An output that the human asked for in another form carries `<!-- tc: exempt because REASON -->`. A departure that keeps the meaning carries `<!-- tc: depart RULE because REASON -->`.

Core rules, by identifier:

- `long-sentence`: an instruction has 20 words at most, and a description has 25.
- `long-paragraph`: a paragraph has 6 sentences at most.
- `one-idea-per-sentence`: a sentence has one subject and one point.
- `one-action-per-step`: a step names one action. Two actions share a step only when they happen at once.
- `imperative-steps`: a step is a command, such as "Restart the worker."
- `condition-first`: the condition comes first, then a comma, then the command.
- `full-words`: write whole sentences, and spell out "do not" and "cannot".
- `passive-voice` and `present-perfect`: advisory. Name the actor, and prefer the simple tense.
- `nominalisation`: "analyse", not "perform an analysis".
- `phrasal-verb`: "start", not "spin up".
- `marketing-adjective`: a measurement replaces a sales word.
- `synonym-rotation`: one word names one action, every time.
- `semicolon`: none. Two sentences replace it.
- `spelling`: British, with the -ise forms.

The model checks each prose document after each write. Where no gate runs, the model runs the checker `scripts/ste-lint.py` itself. The skill holds the other rules, the conversion procedure and the switch.
