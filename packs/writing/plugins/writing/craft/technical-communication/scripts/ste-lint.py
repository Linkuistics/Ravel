#!/usr/bin/env python3
# Vendored from danyuchn/asd-ste100-skill at commit
# 3944283c183eea13fe5bd902b572da3b4de8bf44 (scripts/ste-lint.py), copyright
# (c) 2026 Dustin Yuchen Teng, MIT licensed; see
# ../LICENSES/danyuchn-asd-ste100-skill.LICENSE.
#
# The vendored commit entered this repository unchanged (apart from these
# comment lines) in vendored-checker-k3. Every later change is ours, and every
# changed or added hunk carries a comment that starts with "ours:". A single
# line with that comment is one changed line. A "# ours: begin ..." comment
# opens a hunk and "# ours: end" closes it. The module docstring is ours too.
# Code with no such comment is as upstream wrote it. To see the difference, compare this file with the version
# in the history of vendored-checker-k3.
"""Deterministic checker for the structural rules of the technical-communication skill.

Checks only rules that need no dictionary. Deliberately never flags hedges or
modality (may/might/could): the skill treats confidence as content, and a
checker that pressures hedges out would rewrite claims.

Usage:
    ste-lint.py FILE [FILE ...]
    echo "text" | ste-lint.py [--json]
    ste-lint.py --text-type instruction FILE
    ste-lint.py --text-type commit MESSAGE_FILE
    ste-lint.py --baseline 5 FILE      # pass unless hard findings exceed 5
    ste-lint.py --disable passive-voice,present-perfect FILE
    ste-lint.py --markers FILE [FILE ...]
    ste-lint.py --log-start --harness H --session S [--agent A] --source X
    echo "reply" | ste-lint.py --log-reply --harness H --session S [--agent A] [--model M]
    ste-lint.py --selftest

The two log modes write one record to the session's file in the state directory
and print nothing. They exit 0 on every path, and write nothing when they fail.

Exit 0 when the hard findings that no departure accepts are at or under the
baseline (default 0). Advisory findings never fail a check. Exit 1 when they
are over it. Exit 2 on a usage error: an unknown flag, a bad value, an unknown
rule, or a file that cannot be read or is not UTF-8.
"""
import argparse
import bisect
import contextlib
import datetime  # ours: the reply log
import hashlib  # ours: the reply log
import io
import json
import os
import re
import sys
import tempfile
import threading  # ours: the self-test of the reply log
import time  # ours: the reply log

# ponytail: regex heuristics, not a parser. No noun-cluster rule — needs POS
# tagging to avoid constant false positives; add spaCy-backed rule if ever needed.
# No ellipsis rule by owner's choice: technical writing sometimes earns one.
# Irregular past participles used by the present-perfect heuristic.
IRREGULAR_PARTICIPLES = "given|taken|made|done|found|seen|known|shown|written|built|sent|set|run|read|kept|held|left|put|cut|hit|let|shut|split|spread|begun|become|come|gone|got|gotten|lost|met|paid|said|sold|told|thought|brought|bought|caught|taught|won|worn|torn|born|drawn|grown|thrown|flown|driven|risen|chosen|broken|spoken|frozen|hidden|ridden|forgotten|fallen|eaten|beaten|understood|stood|struck|stuck|swung|hung|led|fed|bled|fled|sped|bound|wound|dug|spun|slid|bit|lit|quit"

# Preserve the original passive heuristic; perfects also include intransitive verbs.
PASSIVE_PARTICIPLES = "given|taken|made|done|found|seen|known|shown|written|built|sent|set|run|read|kept|held|left|put"
MODAL_PERFECT_PREFIX = re.compile(
    r"\b(?:may|might|could|should|would|must|can|will|shall)"
    r"(?:\s+not|n['’]t)?\s+$", re.I,
)

# ours: begin levels, identifiers and messages
# The levels are "hard" and "advisory". The upstream word for hard was
# "advisory-free". Each message names this skill's rule, and {match} stands for
# the text that matched. No message cites the standard or its numbering.
HARD = "hard"
ADVISORY = "advisory"

RULES = [
    ("semicolon", HARD,
     re.compile(r";"),
     "Semicolon. Split the sentence in two."),
    ("phrasal-verb", HARD,
     re.compile(r"\b(spin(?:ning|s)? up|spun up|reach(?:ing|es|ed)? out|div(?:e|es|ing|ed) into|dove into|kick(?:ing|s|ed)? off|circl(?:e|es|ing|ed) back|touch(?:ing|es|ed)? base)\b", re.I),
     "Soft phrasal verb '{match}'. Use one plain verb (start, contact, read, begin)."),
    ("marketing-adjective", HARD,
     re.compile(r"\b(seamless(?:ly)?|robust(?:ly)?|cutting-edge|effortless(?:ly)?|blazing[- ]fast|world-class|state-of-the-art|game-chang(?:ing|er))\b", re.I),
     "Marketing word '{match}'. Delete it, or give the measurement that supports the claim."),
    ("nominalisation", HARD,
     re.compile(r"\b(perform|performs|performed|conduct|conducts|conducted|carry out|carries out|carried out)\s+(?:a|an|the)\s+\w+(?:tion|sion|ment|ance|ence|ysis)\b", re.I),
     "Action turned into a noun ('{match}'). Use the verb: write 'analyse', not 'perform an analysis of'."),
    ("passive-voice", ADVISORY,
     re.compile(r"\b(is|are|was|were|been|being)\s+(\w+ed|" + PASSIVE_PARTICIPLES + r")\b(?!\s+(?:to|for|by)\s+\w+ing)", re.I),
     "Possible passive voice ('{match}'). Name the actor and use an active verb, unless the actor is unknown or does not matter."),
    ("present-perfect", ADVISORY,
     # modal + perfect infinitive ("may have failed") is a protected hedge, not present perfect
     re.compile(r"(?<!\bmay )(?<!\bmight )(?<!\bcould )(?<!\bshould )(?<!\bwould )(?<!\bmust )\b(has|have|had)\s+(?:been\s+)?(?:\w+(?:ed|en)|" + IRREGULAR_PARTICIPLES + r")\b", re.I),
     "Compound tense ('{match}'). Use the simple past or present, unless the link to now is the point."),
]
# ours: end

# One word, one meaning: groups of verbs commonly rotated for the same action.
# Only pairs where the members are genuinely interchangeable — error/fault/failure
# are distinct concepts and stay out.
SYNONYM_GROUPS = [
    ("check", "verify", "confirm", "validate"),
    ("delete", "remove", "erase"),
    ("start", "launch", "begin", "initiate"),
    ("stop", "halt", "terminate"),
    ("show", "display"),
    ("use", "utilize", "employ"),
    ("fix", "repair", "correct"),
    ("send", "transmit"),
    ("get", "retrieve", "fetch", "obtain"),
    ("change", "modify", "alter"),
]

# ours: begin limits and text types
# One sentence is an instruction or a description. Their limits differ, and a
# paragraph has a limit of its own.
INSTRUCTION_WORDS = 20
DESCRIPTION_WORDS = 25
MAX_SENTENCES = 6
TEXT_TYPES = ("auto", "instruction", "description", "commit")
# ours: end

CODE_FENCE = re.compile(r"^(```|~~~)")
INLINE_CODE = re.compile(r"`[^`]*`")
LIST_ITEM_START = re.compile(
    r"^(?P<indent> {0,3})(?P<marker>[-*+]|[0-9]+[.)])(?P<gap> +)(?P<body>.*)$"
)
CONJUNCTION_END = re.compile(r"\b(?:and|or)\s*$", re.I)
TABLE_SEPARATOR_CELL = re.compile(r"^:?-{3,}:?$")
# Headings, thematic breaks, front-matter delimiters, and HTML lines stand alone.
STANDALONE_LINE = re.compile(r"^ {0,3}(?:#|<|(?:[-*_=] *){3,}$)")
BLOCKQUOTE_PREFIX = re.compile(r"^ {0,3}(?:> ?)+")
# Any list marker at any depth starts a new prose block.
ANY_LIST_ITEM = re.compile(r"^\s*(?:[-*+]|[0-9]+[.)])(?:\s|$)")
# A token that ends in terminal punctuation, optionally followed by closing
# quotes, brackets, or Markdown emphasis markers ("**Lead.**").
SENTENCE_END = re.compile(r"[.!?][\"'”’)\]*_]*$")
OPENERS = "\"'“‘([*_"
# Abbreviations whose period never ends a sentence.
ABBREVIATIONS = {"vs.", "cf.", "approx.", "mr.", "mrs.", "ms.", "dr.", "fig.", "no."}
# Dotted initialisms such as "e.g." and "U.S." end a sentence only when the
# next word starts with a capital letter.
DOTTED_ABBREVIATION = re.compile(r"(?:[a-z]\.){2,}", re.I)


def _word_re(base):
    return re.compile(r"\b" + base + r"(?:s|es|ed|d|ing)?\b", re.I)


# ours: begin word count
# A word is a token with at least one letter or digit. Markup tokens such as
# "#", "-", "|" and "---" are not words. Upstream's _mask_inline_code is gone:
# _prepare masks code spans before any rule runs.
_WORD_CHAR = re.compile(r"[^\W_]")


def _count_words(text):
    return sum(1 for token in text.split() if _WORD_CHAR.search(token))
# ours: end


def _sentence_starts(text):
    """Return the offset of each sentence start in text.

    A sentence ends at a word that ends in terminal punctuation, after any
    closing quotes or brackets. Known abbreviations do not end a sentence.
    One pass over the words keeps the cost linear in the text length.
    """
    words = list(re.finditer(r"\S+", text))
    if not words:
        return [0]
    starts = [words[0].start()]
    for word, next_word in zip(words, words[1:]):
        end = SENTENCE_END.search(word.group())
        if not end:
            continue
        core = word.group()[:end.start() + 1].lstrip(OPENERS)
        if core.lower() in ABBREVIATIONS:
            continue
        if (DOTTED_ABBREVIATION.fullmatch(core)
                and not next_word.group().lstrip(OPENERS)[:1].isupper()):
            continue
        starts.append(next_word.start())
    return starts


# ours: begin sentence type and block check
# Upstream's _long_sentence_findings applied one limit of 25 words to every
# sentence. This replacement picks the limit from the sentence's type and adds
# the paragraph check. A block is one prose block: its pieces are
# (lineno, source_column, text) tuples, and its kind is "paragraph", "item",
# "heading" or "cell". "ordered" is true for an ordered-list item.
IMPERATIVE_VERBS = frozenset("""
add adjust allow answer apply ask assign avoid be begin build call cancel change
check choose clean clear click close collect commit compare complete configure
confirm connect consider convert copy create cut define delete describe disable
discard download drag edit enable ensure enter erase execute exit expand export
extract fetch fill find finish fix follow format generate get give go grant
hold identify ignore import include increase initialise initialize insert
inspect install keep kill launch leave let list load locate lock log look make
mark measure merge move name navigate note observe open overwrite paste pause
perform pick place plan press prefer prevent print provide pull push put read
record refer reload remember remove rename repeat replace report request reset
resolve restart restore retry return review run save say scroll search see
select send set show shut sign skip sort specify split start stop store submit
supply switch take tell test tighten trace try turn type uninstall unplug unset
update upload use verify view wait watch write
""".split())
LEADING_CLAUSE = re.compile(r"(?:if|when|before|after|once)\b[^,]*,\s*", re.I)
FIRST_WORDS = re.compile(r"[A-Za-z]+(?:['’][A-Za-z]+)?")


def _is_instruction(sentence):
    """True when the sentence starts with an imperative.

    A leading clause that starts with if, when, before, after or once and ends
    with a comma is skipped first.
    """
    text = sentence.lstrip(OPENERS + " ")
    clause = LEADING_CLAUSE.match(text)
    if clause:
        text = text[clause.end():].lstrip(OPENERS + " ")
    words = FIRST_WORDS.findall(text[:60])
    if not words:
        return False
    first = words[0].lower()
    if first in ("never", "always", "don't", "don’t"):
        return True
    if first == "do" and len(words) > 1 and words[1].lower() == "not":
        return True
    return first in IMPERATIVE_VERBS


def _sentence_type(sentence, ordered, text_type):
    if text_type == "instruction":
        return "instruction"
    if text_type in ("description", "commit"):
        return "description"
    if ordered or _is_instruction(sentence):
        return "instruction"
    return "description"


def _block_findings(block, filename, text_type):
    """Check the sentences of one prose block, and its length if a paragraph.

    The texts of the pieces are joined with spaces, so a sentence wrapped across
    source lines is counted once, and each finding points at the line and
    column where it starts.
    """
    pieces = block["pieces"]
    if not pieces:
        return []
    offsets, position = [], 0
    for _, _, text in pieces:
        offsets.append(position)
        position += len(text) + 1
    joined = " ".join(text for _, _, text in pieces)

    def place(start):
        index = bisect.bisect_right(offsets, start) - 1
        lineno, source_column, _ = pieces[index]
        return lineno, source_column + start - offsets[index] + 1

    findings = []
    sentences = 0
    starts = _sentence_starts(joined)
    for start, end in zip(starts, starts[1:] + [len(joined)]):
        sentence = joined[start:end]
        n = _count_words(sentence)
        if n == 0:
            continue
        sentences += 1
        kind = _sentence_type(sentence, block["ordered"], text_type)
        cap = INSTRUCTION_WORDS if kind == "instruction" else DESCRIPTION_WORDS
        if n <= cap:
            continue
        lineno, col = place(start)
        label = "Instruction" if kind == "instruction" else "Description"
        findings.append({"file": filename, "line": lineno, "col": col,
                         "rule": "long-sentence", "level": HARD,
                         "match": f"{n} words",
                         "message": f"{label} has {n} words (limit {cap}). Split it."})
    if block["kind"] == "paragraph" and sentences > MAX_SENTENCES:
        lineno, col = place(starts[0])
        findings.append({"file": filename, "line": lineno, "col": col,
                         "rule": "long-paragraph", "level": HARD,
                         "match": f"{sentences} sentences",
                         "message": f"Paragraph has {sentences} sentences (limit {MAX_SENTENCES}). Split it or cut it."})
    return findings
# ours: end


def _leading_spaces(line):
    return len(line) - len(line.lstrip(" "))


def _is_list_continuation(line, content_indent):
    if not line.strip():
        return True
    if LIST_ITEM_START.match(line):
        return False
    return _leading_spaces(line) >= content_indent


def _split_table_row(line):
    """Return trimmed table cells and their zero-based source columns.

    A pipe must separate at least two cells. Escaped pipes stay in their cell.
    This deliberately implements only the ordinary Markdown table shape; it is
    enough to distinguish a table from prose that happens to contain a pipe.
    """
    left = len(line) - len(line.lstrip())
    right = len(line.rstrip())
    content = line[left:right]
    if "|" not in content:
        return None
    if content.startswith("|"):
        content = content[1:]
        left += 1
    if content.endswith("|"):
        content = content[:-1]
    raw_cells = re.split(r"(?<!\\)\|", content)
    if len(raw_cells) < 2:
        return None

    cells = []
    column = left
    for raw_cell in raw_cells:
        leading = len(raw_cell) - len(raw_cell.lstrip())
        cells.append((raw_cell.strip(), column + leading))
        column += len(raw_cell) + 1
    return cells


def _markdown_table_cells(lines):
    """Map ordinary Markdown table rows to their prose cells.

    The separator row anchors detection, so pipe-containing prose is not
    treated as a table. Both leading-pipe and no-leading-pipe table styles are
    accepted when their header and body use the same number of cells.
    """
    table_cells = {}
    index = 1
    while index < len(lines):
        separator = _split_table_row(lines[index])
        header = _split_table_row(lines[index - 1])
        if (not separator or not header or len(separator) != len(header)
                or not all(TABLE_SEPARATOR_CELL.fullmatch(cell)
                           for cell, _ in separator)):
            index += 1
            continue

        table_cells[index - 1] = header
        table_cells[index] = []
        index += 1
        while index < len(lines):
            row = _split_table_row(lines[index])
            if not row or len(row) != len(separator):
                break
            table_cells[index] = row
            index += 1
    return table_cells


def _dangling_conjunction_findings(text, filename):
    lines = text.splitlines()
    findings = []
    in_fence = False
    index = 0
    while index < len(lines):
        line = lines[index]
        stripped = line.strip()
        if CODE_FENCE.match(stripped):
            in_fence = not in_fence
            index += 1
            continue
        if in_fence:
            index += 1
            continue
        start = LIST_ITEM_START.match(line)
        if not start:
            index += 1
            continue

        content_indent = (len(start.group("indent"))
                          + len(start.group("marker"))
                          + len(start.group("gap")))
        item_lines = [(index, start.group("body"))]
        next_index = index + 1
        item_fence = False
        while next_index < len(lines):
            candidate = lines[next_index]
            candidate_stripped = candidate.strip()
            if CODE_FENCE.match(candidate_stripped):
                # Fence delimiters are state markers, not meaningful item lines.
                item_fence = not item_fence
                next_index += 1
                continue
            if item_fence:
                next_index += 1
                continue
            if not _is_list_continuation(candidate, content_indent):
                break
            item_lines.append((next_index, candidate))
            next_index += 1

        meaningful = []
        for line_index, item_line in item_lines:
            # Preserve code spans as neutral operands while ignoring their contents.
            cleaned = INLINE_CODE.sub(" CODE ", item_line).strip()
            if cleaned:
                meaningful.append((line_index, cleaned))
        if meaningful:
            end_line_index, end_line = meaningful[-1]
            conjunction = CONJUNCTION_END.search(end_line)
        else:
            end_line_index, end_line, conjunction = None, None, None
        if conjunction:
            if end_line_index == index:
                finding_line = index + 1
                finding_col = start.start("marker") + 1
            else:
                raw_end_line = next(
                    raw for line_index, raw in item_lines
                    if line_index == end_line_index
                )
                masked_end_line = INLINE_CODE.sub(
                    lambda match: " " * len(match.group(0)), raw_end_line
                )
                raw_conjunction = CONJUNCTION_END.search(masked_end_line)
                finding_line = end_line_index + 1
                finding_col = raw_conjunction.start() + 1 if raw_conjunction else 1
            findings.append({
                "file": filename,
                "line": finding_line,
                "col": finding_col,
                "rule": "dangling-conjunction",
                "level": HARD,  # ours: was "advisory-free"
                "match": end_line,
                # ours: message names the rule in this skill's words
                "message": "List item ends with a conjunction. Complete the item or join it to the next one.",
            })
        index = next_index
    return findings


# ours: begin British spelling without a dictionary
# The rule "spelling" finds American forms with suffix patterns and a short
# word list that this file owns. It flags a form only when British usage does
# not also accept it. Code, identifiers, words with internal capitals and words
# in capitals only are not checked.
_PREFIXES = ("un", "dis", "mis", "non", "re", "multi", "water", "tri", "bi",
             "over", "under", "sub")


def _build_spelling():
    """Map each American form to its British form, one entry for each word."""
    table = {}
    full = ("", "s", "ed", "ing", "er", "ers", "ful", "less", "able", "ably",
            "ite", "ites", "ist", "ists")
    verb = ("", "s", "ed", "ing", "er", "ers")
    plain = ("", "s")
    # -or words that are -our in British English
    for us, uk, suffixes in (
            ("color", "colour", full + ("y", "way")),
            ("honor", "honour", full),
            ("favor", "favour", full + ("itism",)),
            ("flavor", "flavour", full + ("ings",)),
            ("behavior", "behaviour", ("", "s", "al", "ally", "ism", "ist", "ists")),
            ("neighbor", "neighbour", verb + ("ly", "hood", "hoods", "liness")),
            ("labor", "labour", verb),
            ("humor", "humour", verb),
            ("harbor", "harbour", verb),
            ("armor", "armour", verb + ("y",)),
            ("clamor", "clamour", verb),
            ("endeavor", "endeavour", verb),
            ("rumor", "rumour", verb),
            ("vapor", "vapour", plain + ("s",)),
            ("savior", "saviour", plain),
            ("odor", "odour", plain + ("less",)),
            ("parlor", "parlour", plain),
            ("tumor", "tumour", plain),
            ("valor", "valour", plain),
            ("vigor", "vigour", plain),
            ("rigor", "rigour", plain),
            ("candor", "candour", plain),
            ("fervor", "fervour", plain),
            ("splendor", "splendour", plain),
            ("ardor", "ardour", plain),
            ("demeanor", "demeanour", plain),
            ("rancor", "rancour", plain),
            ("succor", "succour", plain)):
        for suffix in suffixes:
            table[us + suffix] = uk + suffix
    # -er words that are -re in British English
    re_ending = {"": "e", "s": "es", "ed": "ed", "ing": "ing", "ly": "ely",
                 "piece": "epiece", "pieces": "epieces", "fold": "efold",
                 "glass": "eglass", "optic": "eoptic", "optics": "eoptics",
                 "goer": "egoer", "goers": "egoers"}
    for us, uk_stem, suffixes in (
            ("center", "centr", ("", "s", "ed", "ing", "piece", "pieces", "fold")),
            ("fiber", "fibr", ("", "s", "glass", "optic", "optics")),
            ("liter", "litr", plain),
            ("theater", "theatr", ("", "s", "goer", "goers")),
            ("caliber", "calibr", plain),
            ("saber", "sabr", plain),
            ("somber", "sombr", ("", "ly")),
            ("luster", "lustr", plain),
            ("meager", "meagr", ("", "ly")),
            ("specter", "spectr", plain),
            ("scepter", "sceptr", plain),
            ("sepulcher", "sepulchr", plain),
            ("kilometer", "kilometr", plain),
            ("centimeter", "centimetr", plain),
            ("millimeter", "millimetr", plain),
            ("nanometer", "nanometr", plain),
            ("decimeter", "decimetr", plain)):
        for suffix in suffixes:
            table[us + suffix] = uk_stem + re_ending[suffix]
    # single-l forms that British English doubles
    ending = ("ed", "ing", "er", "ers")
    for stem, suffixes in (
            ("label", ending), ("model", ending), ("cancel", ("ed", "ing")),
            ("travel", ending), ("signal", ending), ("level", ending),
            ("fuel", ("ed", "ing")), ("total", ("ed", "ing")),
            ("counsel", ending + ("or", "ors")), ("channel", ("ed", "ing")),
            ("tunnel", ("ed", "ing")), ("funnel", ("ed", "ing")),
            ("dial", ending), ("panel", ("ed", "ing", "ist", "ists")),
            ("rival", ("ed", "ing")), ("swivel", ("ed", "ing")),
            ("shovel", ("ed", "ing")), ("quarrel", ("ed", "ing")),
            ("jewel", ("er", "ers")), ("unravel", ("ed", "ing")),
            ("grovel", ("ed", "ing")), ("libel", ("ed", "ing")),
            ("pencil", ("ed", "ing")), ("equal", ("ed", "ing")),
            ("marvel", ("ous", "ously")), ("imperil", ("ed", "ing"))):
        for suffix in suffixes:
            table[stem + suffix] = stem + "l" + suffix
    for suffix in ("ed", "ing", "er", "ers"):
        table["worship" + suffix] = "worshipp" + suffix
    # a few single words
    for us, uk in (
            ("cancelation", "cancellation"),
            ("gray", "grey"), ("grays", "greys"), ("grayed", "greyed"),
            ("graying", "greying"), ("grayish", "greyish"), ("grayer", "greyer"),
            ("grayest", "greyest"),
            ("aluminum", "aluminium"),
            ("plow", "plough"), ("plows", "ploughs"), ("plowed", "ploughed"),
            ("plowing", "ploughing"), ("plowshare", "ploughshare"),
            ("pajama", "pyjama"), ("pajamas", "pyjamas"),
            ("skeptic", "sceptic"), ("skeptics", "sceptics"),
            ("skeptical", "sceptical"), ("skeptically", "sceptically"),
            ("skepticism", "scepticism"),
            ("jewelry", "jewellery"), ("woolen", "woollen"),
            ("cozy", "cosy"), ("cozier", "cosier"), ("cozily", "cosily"),
            ("sulfur", "sulphur"), ("sulfate", "sulphate"), ("sulfates", "sulphates"),
            ("sulfide", "sulphide"),
            ("mustache", "moustache"), ("mustaches", "moustaches"),
            ("airplane", "aeroplane"), ("airplanes", "aeroplanes"),
            ("esthetic", "aesthetic"), ("esthetics", "aesthetics"),
            ("esthetically", "aesthetically"),
            ("fulfill", "fulfil"), ("fulfills", "fulfils"),
            ("fulfillment", "fulfilment"), ("fulfillments", "fulfilments"),
            ("enroll", "enrol"), ("enrolls", "enrols"),
            ("enrollment", "enrolment"), ("enrollments", "enrolments"),
            ("skillful", "skilful"), ("skillfully", "skilfully"),
            ("willful", "wilful"), ("willfully", "wilfully"),
            ("installment", "instalment"), ("installments", "instalments"),
            ("distill", "distil"), ("distills", "distils"),
            ("instill", "instil"), ("instills", "instils"),
            ("defense", "defence"), ("defenses", "defences"),
            ("defenseless", "defenceless"),
            ("offense", "offence"), ("offenses", "offences"),
            ("pretense", "pretence"), ("pretenses", "pretences"),
            ("maneuver", "manoeuvre"), ("maneuvers", "manoeuvres"),
            ("maneuvered", "manoeuvred"), ("maneuvering", "manoeuvring"),
            ("maneuverable", "manoeuvrable")):
        table[us] = uk
    return table


_SPELLING = _build_spelling()
# -ize and -yze words. The exceptions are words whose -ize is not the verb
# ending, or that British English spells with z as well.
_IZE = re.compile(
    r"(?P<stem>[a-z]{2,}?)iz(?P<suffix>e|es|ed|ing|er|ers|ation|ations|able|ability)$")
_YZE = re.compile(r"(?P<stem>[a-z]+)yz(?P<suffix>e|es|ed|ing|er|ers)$")
_IZE_EXCEPTION = re.compile(
    r"(?:(?:re|over|under|down|up|out|mid|super|bite|king|pint|full|half|life|"
    r"pocket|large|small|medium|double)?(?:size[sd]?|sizing|sizeable|sizable)"
    r"|prize[sd]?|prizing|seize[sd]?|seizing|capsize[sd]?|capsizing|maize"
    r"|baize|assizes?)$")
_WORD_LIKE = re.compile(r"[A-Za-z]+(?:['’-][A-Za-z]+)*")
_TOKEN_LEAD = "([{\"'“‘*_<>~"
_TOKEN_EDGE = _TOKEN_LEAD + ".,:;!?)]}”’"


def _british(word):
    """Return the British form of an American word, or None."""
    lower = word.lower()
    uk = _SPELLING.get(lower)
    if uk is None:
        for prefix in _PREFIXES:
            if lower.startswith(prefix) and lower[len(prefix):] in _SPELLING:
                uk = prefix + _SPELLING[lower[len(prefix):]]
                break
    if uk is None:
        match = _YZE.fullmatch(lower)
        if match:
            uk = match.group("stem") + "ys" + match.group("suffix")
    if uk is None:
        match = _IZE.fullmatch(lower)
        if match and not _IZE_EXCEPTION.fullmatch(lower):
            uk = match.group("stem") + "is" + match.group("suffix")
    if uk is not None and word[:1].isupper():
        uk = uk[:1].upper() + uk[1:]
    return uk


def _spelling_findings(line, lineno, column, filename):
    """Flag the American forms in one masked prose line.

    column is the zero-based source column of the line's first character.
    """
    findings = []
    previous = ""
    for token in re.finditer(r"\S+", line):
        text = token.group()
        core = text.strip(_TOKEN_EDGE)
        before = previous
        previous = text
        if not core or not _WORD_LIKE.fullmatch(core):
            continue
        offset = token.start() + (len(text) - len(text.lstrip(_TOKEN_EDGE)))
        if core.isupper() or re.search(r"[a-z][A-Z]", core):
            continue
        if core[:1].isupper() and before and not re.search(r"[.!?:]['\"”’)\]*_]*$", before):
            continue  # a capital inside a sentence is a name
        position = 0
        for part in core.split("-"):
            word = re.split(r"['’]", part)[0]
            if word:
                uk = _british(word)
                if uk is not None:
                    findings.append({
                        "file": filename, "line": lineno,
                        "col": column + offset + position + 1,
                        "rule": "spelling", "level": HARD, "match": word,
                        "message": f"American spelling '{word}'. Write '{uk}'."})
            position += len(part) + 1
    return findings
# ours: end


# ours: begin the text that counts as prose
# upstream tracked fences, and the rest of the line work, inside lint(). This
# pass masks everything that is not prose with spaces (so columns hold) and
# records the markers, before any rule runs.
FENCE_OPEN = re.compile(r"^(`{3,}|~{3,})(.*)$")
# A code span, or the start of an HTML comment (the markers are HTML comments).
CODE_OR_COMMENT = re.compile(
    r"(?P<code>(?<!`)(`+)(?!`).+?(?<!`)\2(?!`))|(?P<comment><!--)")
LINK_TARGET = re.compile(
    r"(?<=\])\((?:<[^>\n]*>|(?:[^()\s]|\([^()\s]*\))*)"
    r"(?:\s+(?:\"[^\"]*\"|'[^']*'))?\s*\)")
LINK_LABEL = re.compile(r"(?<=\])\[[^\]\n]*\]")
REFERENCE_DEFINITION = re.compile(r"^ {0,3}\[(?!\^)[^\]\n]+\]:\s*\S.*$")
HTML_TAG = re.compile(r"</?[A-Za-z][^<>\n]*>")
BARE_URL = re.compile(r"(?:\b(?:https?|ftp)://|\bwww\.|\bmailto:)[^\s)>\]]+")
DOUBLE_QUOTED = re.compile(r"\"[^\"\n]*\"")
CURLY_QUOTED = re.compile(r"“[^”\n]*”")
SENTENCE_FOLLOWS = re.compile(r"\s*$|\s+[\"“‘(A-Z0-9]")
LIST_MARKER_PREFIX = re.compile(
    r"^(\s*)(?:(?:#{1,6}|[-*+]|[0-9]+[.)])(?:\s+|$))(?:\[[ xX]\]\s+)?")
HEADING_LINE = re.compile(r"^ {0,3}#{1,6}(?:\s|$)")
ORDERED_ITEM = re.compile(r"^\s*[0-9]+[.)](?:\s|$)")
TRAILER_LINE = re.compile(r"^[A-Za-z][A-Za-z0-9-]*:\s+\S")
COMMIT_IDENTIFIER = re.compile(
    r"^\s*[A-Za-z0-9][\w./-]*(?:\([^)\s]*\))?!?:(?=\s|$)")


def _blank(pattern, line):
    return pattern.sub(lambda match: " " * len(match.group(0)), line)


def _mask_quotes(line):
    """Mask quotations on one line, so they are checked as code is not.

    The quotation marks stay. A full stop, question mark or exclamation mark
    that ends the quotation stays too, when a new sentence follows, so the
    sentence that holds the quotation still ends where it ends.
    """
    def replace(pattern, line):
        out, last = [], 0
        for match in pattern.finditer(line):
            inner = match.group(0)[1:-1]
            keep = ""
            if inner[-1:] in (".", "!", "?") and SENTENCE_FOLLOWS.match(line, match.end()):
                keep = inner[-1]
            out.append(line[last:match.start()])
            out.append(match.group(0)[0] + " " * (len(inner) - len(keep))
                       + keep + match.group(0)[-1])
            last = match.end()
        out.append(line[last:])
        return "".join(out)
    return replace(CURLY_QUOTED, replace(DOUBLE_QUOTED, line))


def _mask_prose_syntax(line):
    """Mask link targets, URLs, HTML tags and quotations on one line."""
    if REFERENCE_DEFINITION.match(line):
        return " " * len(line)
    line = _blank(HTML_TAG, line)
    line = _blank(LINK_TARGET, line)
    line = _blank(LINK_LABEL, line)
    line = _blank(BARE_URL, line)
    return _mask_quotes(line)


def _mask_code_and_comments(line, open_comment, comments, index):
    """Mask code spans and HTML comments on one line.

    Returns the masked line and the comment that is still open at its end.
    A comment is a dict with its first line (start), last line (end), column
    (col) and text. A comment that spans lines is recorded when it closes.
    """
    chars = list(line)

    def blank(first, last):
        for k in range(first, last):
            chars[k] = " "

    position = 0
    if open_comment is not None:
        end = line.find("-->")
        if end < 0:
            open_comment["parts"].append(line)
            return " " * len(line), open_comment
        open_comment["parts"].append(line[:end])
        open_comment["end"] = index
        open_comment["text"] = " ".join(
            part.strip() for part in open_comment["parts"]).strip()
        comments.append(open_comment)
        open_comment = None
        blank(0, end + 3)
        position = end + 3
    while True:
        match = CODE_OR_COMMENT.search(line, position)
        if not match:
            break
        if match.group("code") is not None:
            # the outer backticks stay, so the dangling-conjunction check still
            # sees a code span where upstream saw one
            blank(match.start() + 1, match.end() - 1)
            position = match.end()
            continue
        end = line.find("-->", match.end())
        if end < 0:
            open_comment = {"start": index, "col": match.start() + 1,
                            "parts": [line[match.end():]]}
            blank(match.start(), len(line))
            break
        comments.append({"start": index, "end": index, "col": match.start() + 1,
                         "text": line[match.end():end].strip()})
        blank(match.start(), end + 3)
        position = end + 3
    return "".join(chars), open_comment


def _front_matter(lines):
    """Return (end, keep) for a YAML front matter block, or None.

    end is the index after the closing delimiter. keep maps a line index to the
    line with everything blanked except the value of a description key, which
    is the only prose in front matter.
    """
    if not lines or lines[0].strip() != "---":
        return None
    first = next((i for i in range(1, len(lines)) if lines[i].strip()), None)
    if first is None or not re.match(r"[A-Za-z_][\w-]*\s*:", lines[first]):
        return None
    end = next((i for i in range(first, len(lines))
                if lines[i].strip() in ("---", "...")), None)
    if end is None:
        return None
    keep = {}
    index = 1
    while index < end:
        match = re.match(r"description:[ \t]*(.*)$", lines[index])
        if not match:
            index += 1
            continue
        last = index
        while (last + 1 < end and lines[last + 1].strip()
               and lines[last + 1][:1] in (" ", "\t")):
            last += 1
        value = match.group(1)
        quote = value[:1] if value[:1] in ("\"", "'") else ""
        for k in range(index, last + 1):
            text = lines[k]
            if k == index:
                text = " " * match.start(1) + value
                if value[:1] in ("|", ">"):
                    text = " " * len(lines[k])
                elif quote:
                    text = text[:match.start(1)] + " " + text[match.start(1) + 1:]
            keep[k] = text
        if quote:
            tail = keep[last].rstrip()
            if tail.endswith(quote) and len(tail) > 0:
                keep[last] = tail[:-1] + " " + keep[last][len(tail):]
        index = last + 1
    return end + 1, keep


def _mask_commit(lines):
    """Mask what a commit message holds that is not prose.

    That is a leading "token:" on the subject, and the final block of
    "Key: value" trailers.
    """
    lines = list(lines)
    if not lines:
        return lines
    identifier = COMMIT_IDENTIFIER.match(lines[0])
    if identifier:
        lines[0] = " " * identifier.end() + lines[0][identifier.end():]
    end = len(lines)
    while end > 0 and not lines[end - 1].strip():
        end -= 1
    begin = end
    while begin > 0 and lines[begin - 1].strip():
        begin -= 1
    if begin > 0 and all(TRAILER_LINE.match(lines[i]) or lines[i][:1] in (" ", "\t")
                         for i in range(begin, end)):
        for i in range(begin, end):
            lines[i] = ""
    return lines


def _prepare(text, filename, text_type):
    """Return the masked lines, the HTML comments and the first prose line."""
    lines = text.splitlines()
    if text_type == "commit":
        lines = _mask_commit(lines)
    front = _front_matter(lines) if filename.lower().endswith((".md", ".markdown")) else None
    masked, comments = [], []
    fence = None
    open_comment = None
    first_prose = None
    for index, raw in enumerate(lines):
        if front and index < front[0]:
            kept = front[1].get(index)
            if kept is None:
                masked.append("")
                continue
            raw = kept
        elif fence is not None and open_comment is None:
            stripped = raw.strip()
            if len(stripped) >= fence[1] and set(stripped) == {fence[0]}:
                fence = None
            masked.append("")
            continue
        elif open_comment is None:
            opening = FENCE_OPEN.match(raw.strip())
            if opening and not (opening.group(1)[0] == "`" and "`" in opening.group(2)):
                fence = (opening.group(1)[0], len(opening.group(1)))
                masked.append("")
                continue
        line, open_comment = _mask_code_and_comments(raw, open_comment, comments, index)
        line = _mask_prose_syntax(line)
        if not line.strip():
            line = ""
        elif first_prose is None:
            first_prose = index + 1
        masked.append(line)
    return masked, comments, first_prose
# ours: end


# ours: begin markers
# A marker is an HTML comment: <!-- tc: VERB [RULES] because REASON -->
MARKER_VERBS = ("exempt", "depart", "off", "pending", "on")
MARKER_START = re.compile(r"tc:\s*(.*)$", re.S)
MARKER_FORM = "Write 'tc: VERB [RULES] because REASON'."


def _marker_error(verb, rules, message):
    return {"verb": verb, "rules": rules, "reason": "", "error": message}


def _parse_marker(body, departable):
    """Parse the text after 'tc:' into a marker dict. error is None when valid."""
    match = re.match(r"([A-Za-z][\w-]*)\s*(.*)$", body.strip(), re.S)
    if not match:
        return _marker_error("", [], "Marker has no verb. " + MARKER_FORM)
    verb, rest = match.group(1), match.group(2).strip()
    if verb not in MARKER_VERBS:
        return _marker_error(
            verb, [], f"Marker verb '{verb}' is unknown. Use exempt, depart, off, pending or on.")
    if verb == "on":
        if rest:
            return _marker_error(verb, [], "Marker 'on' takes no rules and no reason.")
        return {"verb": verb, "rules": [], "reason": "", "error": None}
    parts = re.match(r"(?:(\S+)\s+)?because\s+(\S.*)$", rest, re.S)
    if not parts:
        return _marker_error(
            verb, [], f"Marker '{verb}' needs 'because' and a reason. " + MARKER_FORM)
    token, reason = parts.group(1), parts.group(2).strip()
    rules = []
    if verb == "depart":
        if token is None:
            return _marker_error(verb, [], "Marker 'depart' needs the rules it departs from.")
        rules = token.split(",")
        for rule in rules:
            if rule not in departable:
                return _marker_error(
                    verb, rules, f"Marker names '{rule}', which is not a rule that a departure can accept.")
    elif token is not None:
        return _marker_error(verb, [], f"Marker '{verb}' takes no rules.")
    return {"verb": verb, "rules": rules, "reason": reason, "error": None}
# ours: end


def lint(text, filename="<stdin>", text_type="auto"):
    # ours: upstream returned every finding. This returns the findings that no
    # departure accepts. analyse() gives the full result.
    result = analyse(text, filename, text_type)
    return result["findings"], result["words"]


# ours: begin the whole-document analysis
def _lint_lines(lines, filename, text_type):
    """Run every rule over the masked lines.

    Returns the findings, the word count and a list that maps each line number
    to the id of the block that holds it (index 0 is unused).
    """
    findings = []
    words_total = 0
    table_cells = _markdown_table_cells(lines)
    # first occurrence of each synonym-group member: (group_idx, base) -> (line, col, match)
    seen_synonyms = {}
    # consecutive prose lines form one block, so wrapped sentences count once
    block = {"pieces": [], "kind": "paragraph", "ordered": False}
    scope = [0]
    line_scope = [0]
    in_quote = False

    def flush():
        findings.extend(_block_findings(block, filename, text_type))
        block["pieces"] = []
        block["kind"] = "paragraph"
        block["ordered"] = False
        scope[0] += 1

    for lineno, raw_line in enumerate(lines, 1):
        segments = table_cells.get(lineno - 1, [(raw_line, 0)])
        is_table = (lineno - 1) in table_cells
        quote = BLOCKQUOTE_PREFIX.match(raw_line)
        is_quote = bool(quote)
        # judge block boundaries on the text inside any blockquote prefix
        content = raw_line[quote.end():] if quote else raw_line
        standalone = STANDALONE_LINE.match(content)
        if (is_table or not content.strip() or standalone
                or ANY_LIST_ITEM.match(content) or is_quote != in_quote):
            flush()
        in_quote = is_quote
        line_scope.append(scope[0])
        for segment, source_column in segments:
            line = segment
            skip = 0
            if not is_table:
                # a heading mark or list marker is markup, not a word
                skip = quote.end() if quote else 0
                marker = LIST_MARKER_PREFIX.match(line[skip:])
                if marker:
                    gap = marker.end() - len(marker.group(1))
                    line = (line[:skip + len(marker.group(1))] + " " * gap
                            + line[skip + marker.end():])
            words_total += _count_words(line[skip:])
            for rule_id, level, pattern, msg in RULES:
                for m in pattern.finditer(line):
                    if rule_id == "present-perfect" and MODAL_PERFECT_PREFIX.search(
                        line[:m.start()]
                    ):
                        continue
                    findings.append({"file": filename, "line": lineno,
                                     "col": source_column + m.start() + 1,
                                     "rule": rule_id, "level": level,
                                     "match": m.group(0),
                                     "message": msg.format(match=m.group(0))})
            findings.extend(_spelling_findings(line[skip:], lineno,
                                               source_column + skip, filename))
            for gi, group in enumerate(SYNONYM_GROUPS):
                for base in group:
                    if (gi, base) in seen_synonyms:
                        continue
                    m = _word_re(base).search(line)
                    if m:
                        seen_synonyms[(gi, base)] = (
                            lineno, source_column + m.start() + 1, m.group(0)
                        )
            # the blockquote prefix is markup, not words in the sentence
            if line[skip:].strip():
                if not block["pieces"]:
                    if is_table:
                        block["kind"] = "cell"
                    elif HEADING_LINE.match(content):
                        block["kind"] = "heading"
                    elif ANY_LIST_ITEM.match(content):
                        block["kind"] = "item"
                    block["ordered"] = bool(ORDERED_ITEM.match(content))
                block["pieces"].append((lineno, source_column + skip, line[skip:]))
            if is_table:
                flush()  # each table cell is its own block
        if standalone:
            flush()
    flush()
    # synonym rotation: flag each member after the first, at its first occurrence
    for gi, group in enumerate(SYNONYM_GROUPS):
        present = [(seen_synonyms[(gi, b)], b) for b in group if (gi, b) in seen_synonyms]
        if len(present) > 1:
            present.sort()  # document order
            first_base = present[0][1]
            for (lineno, col, match), base in present[1:]:
                findings.append({"file": filename, "line": lineno, "col": col,
                                 "rule": "synonym-rotation", "level": HARD,
                                 "match": match,
                                 "message": f"'{base}' and '{first_base}' name the same action. Pick one and use it every time."})
    findings.extend(_dangling_conjunction_findings("\n".join(lines), filename))
    return findings, words_total, line_scope


def analyse(text, filename="<stdin>", text_type="auto", reply=False):
    """Check one text. Returns a dict, and writes nothing.

    findings   the findings that no departure accepts, sorted by position
    accepted   a count for each rule of the findings that a departure accepted
    pending    the number of pending regions
    words      the prose words that were checked
    exempt     true when a valid exempt marker is present; a document it makes
               unchecked, with no findings and no words
    markers    each marker: line, col, verb, rules, reason and error
    first_prose_line  the first line that holds prose, or None
    unclosed   the verb of an off or pending region that the end of the text
               leaves open, or None

    With reply=True only an exempt marker has an effect: it sets exempt, and
    the reply is still scored. The other markers are ignored, valid or not.
    """
    masked, comments, first_prose = _prepare(text, filename, text_type)
    departable = RULE_IDS - {"marker"}
    markers = []
    for comment in comments:
        match = MARKER_START.match(comment["text"])
        if not match:
            continue
        marker = _parse_marker(match.group(1), departable)
        if reply and marker["verb"] != "exempt":
            continue
        marker["line"] = comment["start"] + 1
        marker["end_line"] = comment["end"] + 1
        marker["col"] = comment["col"]
        marker["alone"] = all(
            not re.sub(r"[>\s]", "", masked[i])
            for i in range(comment["start"], comment["end"] + 1))
        marker["text"] = comment["text"]
        markers.append(marker)
    markers.sort(key=lambda m: (m["line"], m["col"]))

    # resolve the markers in order: regions open and close, exempt applies
    exempt = False
    pending = 0
    regions = []
    open_region = None
    for marker in markers:
        if marker["error"] is not None:
            continue
        verb = marker["verb"]
        if verb == "exempt":
            exempt = True
        elif verb in ("off", "pending"):
            if open_region is not None:
                marker["error"] = (f"Marker '{verb}' opens a region while the "
                                   f"'{open_region['verb']}' region from line "
                                   f"{open_region['line']} is open. Close that one with 'tc: on' first.")
            else:
                open_region = marker
                if verb == "pending":
                    pending += 1
        elif verb == "on":
            if open_region is None:
                marker["error"] = "Marker 'on' closes nothing. No 'off' or 'pending' region is open."
            else:
                regions.append((open_region["line"], marker["line"]))
                open_region = None
    unclosed = open_region["verb"] if open_region is not None else None
    if open_region is not None:
        regions.append((open_region["line"], len(masked) + 1))

    result = {"findings": [], "accepted": {}, "pending": pending, "words": 0,
              "exempt": exempt, "markers": markers,
              "first_prose_line": first_prose, "unclosed": unclosed,
              "text_type": text_type}
    if exempt and not reply:
        return result
    for first, last in regions:
        for lineno in range(first + 1, last):
            masked[lineno - 1] = ""

    findings, words, line_scope = _lint_lines(masked, filename, text_type)

    # apply the departures
    for marker in markers:
        if marker["error"] is not None or marker["verb"] != "depart":
            continue
        target = marker["line"]
        if marker["alone"]:
            target = next((n for n in range(marker["end_line"] + 1, len(masked) + 1)
                           if masked[n - 1].strip()), None)
        if target is None:
            marker["error"] = "Departure has no block to apply to. Put it before the block or in it."
            continue
        marker["scope"] = line_scope[target]
    accepted = {}
    kept = []
    for finding in findings:
        for marker in markers:
            if (marker["error"] is None and marker["verb"] == "depart"
                    and finding["rule"] in marker["rules"]
                    and marker["scope"] == line_scope[finding["line"]]):
                accepted[finding["rule"]] = accepted.get(finding["rule"], 0) + 1
                break
        else:
            kept.append(finding)
    for marker in markers:
        if marker["error"] is not None:
            kept.append({"file": filename, "line": marker["line"],
                         "col": marker["col"], "rule": "marker", "level": HARD,
                         "match": marker["text"][:60], "message": marker["error"]})
    kept.sort(key=lambda f: (f["line"], f["col"]))
    result.update(findings=kept, accepted=accepted, words=words)
    return result
# ours: end


# ours: begin reports
def _tally(results):
    """Add the results of several files together."""
    total = {"findings": [], "accepted": {}, "pending": 0, "words": 0,
             "exempt": False}
    for result in results:
        total["findings"].extend(result["findings"])
        for rule, count in result["accepted"].items():
            total["accepted"][rule] = total["accepted"].get(rule, 0) + count
        total["pending"] += result["pending"]
        total["words"] += result["words"]
        total["exempt"] = total["exempt"] or result["exempt"]
    total["hard"] = sum(1 for f in total["findings"] if f["level"] == HARD)
    total["advisory"] = len(total["findings"]) - total["hard"]
    total["per_100"] = (round(total["hard"] * 100 / total["words"], 1)
                        if total["words"] else 0.0)
    return total


def report(total, text_type, as_json, baseline):
    if as_json:
        print(json.dumps({"findings": total["findings"], "hard": total["hard"],
                          "advisory": total["advisory"],
                          "accepted": total["accepted"],
                          "pending": total["pending"], "words": total["words"],
                          "per_100": total["per_100"], "exempt": total["exempt"],
                          "text_type": text_type, "baseline": baseline}, indent=2))
        return
    for f in total["findings"]:
        print(f"{f['file']}:{f['line']}:{f['col']} {f['rule']}: {f['message']}")
    print(f"{total['hard']} hard, {total['advisory']} advisory, "
          f"{sum(total['accepted'].values())} accepted, {total['pending']} pending, "
          f"{total['words']} words, {total['per_100']} hard per 100 words "
          f"(baseline {baseline})")


def report_markers(entries, as_json):
    """Print each marker, and each faulty marker, for the --markers mode."""
    if as_json:
        print(json.dumps({"markers": entries}, indent=2))
        return
    for e in entries:
        if e["error"] is None:
            rules = ",".join(e["rules"])
            detail = f" {rules}" if rules else ""
            reason = f" because {e['reason']}" if e["reason"] else ""
            print(f"{e['file']}:{e['line']} {e['verb']}{detail}{reason}")
        else:
            print(f"{e['file']}:{e['line']}:{e['col']} marker: {e['error']}")
# ours: end

# ours: begin the reply log
STATE_DIR_VARIABLE = "TECHNICAL_COMMUNICATION_STATE_DIR"
LOCK_WAIT_SECONDS = 2.0
LOG_VERSION = 1


def state_dir():
    """The directory that holds the off file and the sessions: the variable's
    value, else technical-communication in $XDG_STATE_HOME (default ~/.local/state)."""
    override = os.environ.get(STATE_DIR_VARIABLE)
    if override:
        return override
    base = os.environ.get("XDG_STATE_HOME", "")
    if not os.path.isabs(base):
        base = os.path.join(os.path.expanduser("~"), ".local", "state")
    return os.path.join(base, "technical-communication")


def checker_digest():
    """The first 12 hex digits of the SHA-256 of this file."""
    with open(os.path.realpath(__file__), "rb") as fh:
        return hashlib.sha256(fh.read()).hexdigest()[:12]


def _session_file(harness, session):
    safe = lambda value: re.sub(r"[^A-Za-z0-9._-]", "_", value or "unknown")  # noqa: E731
    name = f"{safe(harness)}--{safe(session)}.jsonl"
    return os.path.join(state_dir(), "sessions", name)


def _utc_now():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def _append_record(path, build):
    """Append one record to a session file under an exclusive lock.

    build(earlier) receives the earlier records that parse and returns the new
    record. Returns True when the line was written. A failure of any kind writes
    nothing: the caller has no error to handle.
    """
    import fcntl
    os.makedirs(os.path.dirname(path), exist_ok=True)
    fd = os.open(path, os.O_RDWR | os.O_CREAT | os.O_APPEND, 0o600)
    try:
        deadline = time.monotonic() + LOCK_WAIT_SECONDS
        while True:
            try:
                fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
                break
            except OSError:
                if time.monotonic() >= deadline:
                    return False
                time.sleep(0.01)
        size = os.fstat(fd).st_size
        data = b""
        while len(data) < size:
            chunk = os.pread(fd, size - len(data), len(data))
            if not chunk:
                break
            data += chunk
        earlier = []
        for line in data.decode("utf-8", errors="replace").splitlines():
            try:
                record = json.loads(line)
            except ValueError:
                continue
            if isinstance(record, dict):
                earlier.append(record)
        record = build(earlier)
        if record is None:
            return False
        line = json.dumps(record, ensure_ascii=False, separators=(",", ":")) + "\n"
        # a crashed writer may have left a line without its newline
        if data and not data.endswith(b"\n"):
            line = "\n" + line
        payload = line.encode("utf-8")
        written = os.write(fd, payload)
        return written == len(payload)
    finally:
        os.close(fd)  # closing the descriptor releases the lock


def _base_record(kind, harness, session, agent):
    return {"v": LOG_VERSION, "kind": kind, "ts": _utc_now(),
            "harness": harness or "unknown", "session": session or "unknown",
            "agent": agent or None}


def log_start(harness, session, agent, source):
    """Append a start record. Returns True when it was written."""
    def build(_earlier):
        record = _base_record("start", harness, session, agent)
        record["source"] = source or None
        return record
    return _append_record(_session_file(harness, session), build)


def score_reply(text):
    """Score one reply. Returns the scoring fields of a reply record, or None
    when the reply has no prose words."""
    result = analyse(text, "<reply>", "auto", reply=True)
    if not result["words"]:
        return None
    rules = {}
    for finding in result["findings"]:
        rules[finding["rule"]] = rules.get(finding["rule"], 0) + 1
    hard = sum(1 for f in result["findings"] if f["level"] == HARD)
    return {"words": result["words"], "hard": hard,
            "advisory": len(result["findings"]) - hard,
            "per_100": round(hard * 100 / result["words"], 1),
            "rules": dict(sorted(rules.items())), "exempt": result["exempt"]}


def log_reply(text, harness, session, agent, model):
    """Score a reply and append a reply record. Returns True when written."""
    score = score_reply(text)
    if score is None:
        return False
    digest = checker_digest()
    agent = agent or None

    def build(earlier):
        mine = [r for r in earlier if r.get("agent") == agent]
        replies = sum(1 for r in mine if r.get("kind") == "reply")
        last_start = max((i for i, r in enumerate(mine) if r.get("kind") == "start"),
                         default=-1)
        since = sum(1 for r in mine[last_start + 1:] if r.get("kind") == "reply")
        record = _base_record("reply", harness, session, agent)
        record.update(model=model or None, seq=replies + 1, since_start=since + 1,
                      **score, checker=digest)
        return record
    return _append_record(_session_file(harness, session), build)


class _QuietParser(argparse.ArgumentParser):
    def error(self, message):
        raise ValueError(message)


def log_main(argv):
    """The --log-start and --log-reply modes. Exit 0 on every path: when
    anything fails, nothing is written."""
    try:
        parser = _QuietParser(prog="ste-lint.py", add_help=False, allow_abbrev=False)
        mode = parser.add_mutually_exclusive_group(required=True)
        mode.add_argument("--log-start", action="store_true")
        mode.add_argument("--log-reply", action="store_true")
        for name in ("--harness", "--session", "--agent", "--source", "--model"):
            parser.add_argument(name, default=None)
        args = parser.parse_args(argv)
        if args.log_start:
            log_start(args.harness, args.session, args.agent, args.source)
        else:
            text = sys.stdin.buffer.read().decode("utf-8", errors="replace")
            log_reply(text.lstrip("﻿"), args.harness, args.session,
                      args.agent, args.model)
    except BaseException:  # noqa: BLE001 - a log failure never reaches the model
        pass
    return 0
# ours: end


def selftest():
    # ours: the upstream assertions follow, with the renamed rule and level
    bad = ("The panel is removed; spin up the job. "
           "Perform an analysis of the seamless log. "
           "We have received the report.")
    findings, _ = lint(bad)
    rules = {f["rule"] for f in findings}
    for expected in ("semicolon", "phrasal-verb", "nominalisation",  # ours
                     "marketing-adjective", "passive-voice", "present-perfect"):
        assert expected in rules, expected
    # hedges must never be flagged, including modal + perfect infinitive
    findings, _ = lint("The request may have failed. It could be a timeout. "
                       "The disk might have filled.")
    assert findings == [], findings
    # irregular participles: "has run" is a compound tense as much as "has failed"
    findings, _ = lint("The task has run. The job has set the flag. We have begun.")
    assert sum(1 for f in findings if f["rule"] == "present-perfect") == 3, findings
    findings, _ = lint("The job may have run.")
    assert not any(f["rule"] == "present-perfect" for f in findings), findings
    # Modal perfects remain protected across negation and variable whitespace.
    for modal in ("may", "might", "could", "should", "would", "must"):
        for gap in (" ", "  ", "\t", " not "):
            findings, _ = lint(f"The task {modal}{gap}have run.")
            assert not any(f["rule"] == "present-perfect" for f in findings), findings
    findings, _ = lint("The task couldn't have run. The task MAY NOT HAVE RUN.")
    assert not any(f["rule"] == "present-perfect" for f in findings), findings
    findings, _ = lint("The task has run. We have begun. The flag is set.")
    assert sum(f["rule"] == "present-perfect" for f in findings) == 2, findings
    assert any(f["rule"] == "passive-voice" for f in findings), findings
    findings, _ = lint("The task is gone.")
    assert not any(f["rule"] == "passive-voice" for f in findings), findings
    # code blocks skipped
    findings, _ = lint("```\nx = a; y = b\n```")
    assert findings == []
    # all supported list markers, case variants, and trailing whitespace
    findings, _ = lint(
        "- Confirm the target and\n"
        "* Record the result OR  \n"
        "+ Close the panel\n"
        "1. Start the task and\n"
        "2) Stop the task OR"
    )
    dangling = [f for f in findings if f["rule"] == "dangling-conjunction"]
    assert len(dangling) == 4, dangling
    assert [f["line"] for f in dangling] == [1, 2, 4, 5], dangling
    assert [f["col"] for f in dangling] == [1, 1, 1, 1], dangling
    assert all(f["level"] == "hard" for f in dangling), dangling  # ours

    # valid continuation lines and standalone four-space code are ignored
    findings, _ = lint("  - Confirm the target and\n    record the result.")
    assert not any(f["rule"] == "dangling-conjunction" for f in findings)
    findings, _ = lint("- Confirm the target\n  and")
    dangling = [f for f in findings if f["rule"] == "dangling-conjunction"]
    assert len(dangling) == 1 and dangling[0]["line"] == 2, dangling
    assert dangling[0]["col"] == 3, dangling
    findings, _ = lint("    - code and")
    assert not any(f["rule"] == "dangling-conjunction" for f in findings)
    findings, _ = lint("> - Confirm the target and\n> - Record the result or")
    assert not any(f["rule"] == "dangling-conjunction" for f in findings)
    findings, _ = lint("- Do this and\n~~~\ncode and\n~~~")
    dangling = [f for f in findings if f["rule"] == "dangling-conjunction"]
    assert len(dangling) == 1 and dangling[0]["line"] == 1, dangling
    findings, _ = lint("```text\n- code and\n```")
    assert not any(f["rule"] == "dangling-conjunction" for f in findings)

    # one- and three-space markers and ordered continuation width
    findings, _ = lint(" - Start the task and\n   record the result.")
    assert not any(f["rule"] == "dangling-conjunction" for f in findings)
    findings, _ = lint("-  Start the task and\n   record the result.")
    assert not any(f["rule"] == "dangling-conjunction" for f in findings)
    findings, _ = lint("-\tStart the task and")
    assert not any(f["rule"] == "dangling-conjunction" for f in findings)
    findings, _ = lint("   - Start the task and", filename="fixture.md")
    dangling = [f for f in findings if f["rule"] == "dangling-conjunction"]
    assert len(dangling) == 1 and dangling[0]["col"] == 4, dangling
    assert dangling[0]["file"] == "fixture.md"
    assert dangling[0]["match"].endswith("and")
    assert "Complete the item" in dangling[0]["message"]
    findings, _ = lint("100. Start the task and\n  unrelated text")
    dangling = [f for f in findings if f["rule"] == "dangling-conjunction"]
    assert len(dangling) == 1, dangling
    findings, _ = lint("- Start the task and.\n- Stop the task or,")
    assert not any(f["rule"] == "dangling-conjunction" for f in findings)
    findings, _ = lint("- Start the task and\n\n  record the result.")
    assert not any(f["rule"] == "dangling-conjunction" for f in findings)
    findings, _ = lint("- Parent item and\n  - Nested item or")
    dangling = [f for f in findings if f["rule"] == "dangling-conjunction"]
    assert [f["line"] for f in dangling] == [1, 2], dangling

    # ordinary prose, inline code, and fenced code are ignored
    findings, _ = lint("The process may include steps and")
    assert not any(f["rule"] == "dangling-conjunction" for f in findings)
    findings, _ = lint("- Use `and` as a label")
    assert not any(f["rule"] == "dangling-conjunction" for f in findings)
    findings, _ = lint("- Combine `left` and `right`")
    assert not any(f["rule"] == "dangling-conjunction" for f in findings), findings
    findings, _ = lint("~~~\n- code and\n~~~")
    assert not any(f["rule"] == "dangling-conjunction" for f in findings)
    findings, _ = lint(("word " * 30).strip() + ".")
    assert any(f["rule"] == "long-sentence" for f in findings)
    # Markdown table syntax is layout, not prose. Each cell stays lintable.
    short_cell = " ".join(f"term{number}" for number in range(1, 25)) + "."
    for table in (
            "| Label | Detail |\n"
            "| --- | --- |\n"
            f"| Clear | {short_cell} |",
            "Label | Detail\n"
            "--- | ---\n"
            f"Clear | {short_cell}"):
        findings, words_total = lint(table)
        assert not any(f["rule"] == "long-sentence" for f in findings), findings
        assert words_total == 27, words_total
    long_cell = " ".join(f"term{number}" for number in range(1, 27)) + "."
    findings, _ = lint(
        "| Label | Detail |\n"
        "| --- | --- |\n"
        f"| Clear | {long_cell} |"
    )
    long_sentences = [f for f in findings if f["rule"] == "long-sentence"]
    assert len(long_sentences) == 1, long_sentences
    assert long_sentences[0]["match"] == "26 words", long_sentences
    # synonym rotation: second member flagged, first named as the keeper
    findings, _ = lint("Check the config file. Then verify the output. Verify twice.")
    rot = [f for f in findings if f["rule"] == "synonym-rotation"]
    assert len(rot) == 1 and "'verify' and 'check'" in rot[0]["message"], rot
    # single consistent term: no flag
    findings, _ = lint("Check the config. Check the output.")
    assert not any(f["rule"] == "synonym-rotation" for f in findings)
    # per-file labels
    findings, _ = lint("a; b", filename="x.md")
    assert findings[0]["file"] == "x.md"
    # inline code is masked, not deleted, so columns match the source
    findings, _ = lint("Run `make all` then spin up the node.")
    phrasal = [f for f in findings if f["rule"] == "phrasal-verb"]
    assert len(phrasal) == 1 and phrasal[0]["col"] == 21, phrasal
    # a sentence ends after closing quotes and brackets
    twenty = " ".join(["word"] * 20)
    findings, _ = lint(f'He asked "{twenty}?" Then {twenty} ended. (It {twenty}.) Next.')
    assert not any(f["rule"] == "long-sentence" for f in findings), findings
    # abbreviations do not end a sentence
    findings, _ = lint(f"Use a tool, e.g. the {twenty} linter here now.")
    assert any(f["rule"] == "long-sentence" for f in findings), findings
    findings, _ = lint(f"Compare A vs. B in the {twenty} test.")
    assert any(f["rule"] == "long-sentence" for f in findings), findings
    # a sentence wrapped across lines is counted once, at its start
    findings, _ = lint(f"Intro text here.\nThe {twenty}\nwrapped tail words go here.")
    long_sentences = [f for f in findings if f["rule"] == "long-sentence"]
    assert len(long_sentences) == 1, long_sentences
    assert (long_sentences[0]["line"], long_sentences[0]["col"]) == (2, 1), long_sentences
    findings, _ = lint(f"- Item {twenty}\n  continued in the same item.")
    assert any(f["rule"] == "long-sentence" for f in findings), findings
    # blank lines, headings, rules, list items, and quotes end a prose block
    for sep in ("\n\n", "\n# Heading\n", "\n---\n", "\n- ", "\n> "):
        findings, _ = lint(f"First {twenty} line{sep}Second {twenty} line.")
        assert not any(f["rule"] == "long-sentence" for f in findings), (sep, findings)
    # a bold or italic lead sentence ends at its closing emphasis marker
    findings, _ = lint(f"**Lead.** The {twenty} more words here now.")
    assert not any(f["rule"] == "long-sentence" for f in findings), findings
    findings, _ = lint(f"_Note._ The {twenty} more words here now.")
    assert not any(f["rule"] == "long-sentence" for f in findings), findings
    findings, _ = lint(f"See **e.g.** the {twenty} linter here now.")
    assert any(f["rule"] == "long-sentence" for f in findings), findings
    # a dotted abbreviation ends a sentence before a capitalized word
    findings, _ = lint(f"The team shipped it in the U.S. Then {twenty} today.")
    assert not any(f["rule"] == "long-sentence" for f in findings), findings
    # nested list items, quoted list items, and HTML lines end a prose block
    for text in (f"- First item\n    - First {twenty} line\n      - Second {twenty} line",
                 f"> - First {twenty} line\n> - Second {twenty} line",
                 f"<!-- First {twenty} line -->\nSecond {twenty} line.",
                 f"> First {twenty} line.\n>\n> Second {twenty} line."):
        findings, _ = lint(text)
        assert not any(f["rule"] == "long-sentence" for f in findings), (text, findings)
    # a blockquote prefix is not a word in a wrapped quoted sentence
    findings, _ = lint("> " + "\n> ".join(["word word word word word"] * 5) + ".")
    assert not any(f["rule"] == "long-sentence" for f in findings), findings
    # a long block stays linear: one short sentence per line, no blank lines
    findings, _ = lint("The agent read the log.\n" * 20000)
    assert not any(f["rule"] == "long-sentence" for f in findings)
    # flags may appear between file paths
    with tempfile.TemporaryDirectory() as tmp:
        paths = [os.path.join(tmp, name) for name in ("a.md", "b.md")]
        for path in paths:
            with open(path, "w", encoding="utf-8") as fh:
                fh.write("Short line here.\n")
        out = io.StringIO()
        with contextlib.redirect_stdout(out):
            assert main([paths[0], "--json", paths[1]]) == 0
        assert json.loads(out.getvalue())["words"] == 6, out.getvalue()
    # command-line parsing rejects typos and bad values
    for bad in (["--basline", "3"], ["--baseline"], ["--baseline", "x"],
                ["--baseline", "-1"], ["--disable", "passive"], ["--base", "3"],
                ["--text-type", "poem"]):  # ours
        try:
            with contextlib.redirect_stderr(io.StringIO()):
                main(bad)
        except SystemExit as e:
            assert e.code == 2, (bad, e.code)
        else:
            raise AssertionError(bad)
    assert _rule_list("passive-voice,present-perfect") == {"passive-voice", "present-perfect"}
    # ours: begin assertions for the whole-document checks
    _selftest_documents()
    # ours: end
    # ours: begin assertions for the reply log
    _selftest_log()
    # ours: end
    print("selftest OK")


# ours: begin
def _rules_of(findings):
    return [f["rule"] for f in findings]


def _selftest_documents():
    twenty = " ".join(["word"] * 20)
    # levels and names: hard and advisory, nominalisation, no standard in a message
    findings, _ = lint("Perform an analysis of the log. The log is read.")
    assert {f["level"] for f in findings} == {"hard", "advisory"}, findings
    # ours: the standard's initials are built in two parts, so that a sweep for
    # the initials does not match this test
    assert all(("ST" + "E") not in f["message"] and "Rule " not in f["message"]
               for f in findings), findings

    # text types: 20 words for an instruction, 25 for a description
    twenty_one = "Check " + " ".join(["word"] * 20) + "."      # 21 words, imperative
    assert "long-sentence" in _rules_of(lint(twenty_one)[0])
    assert "long-sentence" not in _rules_of(lint("The " + " ".join(["word"] * 20) + ".")[0])
    twenty_four = "The " + " ".join(["word"] * 23) + "."       # 24 words, description
    assert "long-sentence" not in _rules_of(lint(twenty_four)[0])
    assert "long-sentence" in _rules_of(lint(twenty_four, text_type="instruction")[0])
    assert "long-sentence" not in _rules_of(lint(twenty_one, text_type="description")[0])
    assert "long-sentence" in _rules_of(lint("1. The " + " ".join(["word"] * 20) + ".")[0])
    # a leading if/when clause is skipped before the imperative test
    assert "long-sentence" in _rules_of(lint("If it fails, check " + " ".join(["word"] * 18) + ".")[0])
    assert "long-sentence" not in _rules_of(lint("If it fails, the " + " ".join(["word"] * 19) + ".")[0])
    for opener in ("Do not", "Never", "Always"):
        assert "long-sentence" in _rules_of(lint(opener + " " + " ".join(["word"] * 20) + ".")[0]), opener
    # markup tokens are not words
    assert lint("# Title\n\n- item one\n\n1. item two\n\n| a | b |\n| --- | --- |\n")[1] == 7
    assert lint("---\n")[1] == 0

    # paragraph: six sentences at most, but not for list items or headings
    six = " ".join(["It runs."] * 6)
    assert "long-paragraph" not in _rules_of(lint(six)[0])
    assert "long-paragraph" in _rules_of(lint(six + " It stops.")[0])
    assert "long-paragraph" not in _rules_of(lint("- " + six + " It stops.")[0])
    assert "long-paragraph" not in _rules_of(lint("# " + six + " It stops.")[0])

    # what is not prose: fences by kind and length, link targets, URLs, comments
    assert lint("````\n```\na; b\n````\nA clean line.")[0] == []
    assert lint("~~~\n```\na; b\n~~~\nA clean line.")[0] == []
    assert lint("See [the guide](https://e.com/a;b) now.")[0] == []
    assert lint("See https://e.com/a;b now.")[0] == []
    assert lint("See <https://e.com/a;b> now.")[0] == []
    assert lint("[ref]: https://e.com/a;b\n")[0] == []
    assert "semicolon" in _rules_of(lint("See [a; b](https://e.com) now.")[0])
    assert lint("A clean line. <!-- note; with a semicolon --> More text.")[0] == []
    assert lint("A clean line. <!-- a\nsplit; comment --> More text.")[0] == []
    assert lint("Run ``a ` b; c`` now.")[0] == []
    assert lint('He said "a; b" once.')[0] == []
    assert lint("He said “a; b” once.")[0] == []
    assert "semicolon" in _rules_of(lint("**bold; text**")[0])
    # front matter is not prose, except the value of a description
    assert lint("---\ntitle: Use utilize; verify\n---\nA clean line.", "a.md")[0] == []
    assert "semicolon" in _rules_of(lint("---\ndescription: One; two\n---\nBody.", "a.md")[0])
    assert "semicolon" in _rules_of(lint('---\ndescription: "One; two"\n---\nBody.', "a.md")[0])
    assert "semicolon" in _rules_of(lint("---\ndescription: >\n  One; two\n---\nBody.", "a.md")[0])
    assert lint("---\ndescription: One two.\n---\nBody.", "a.md")[1] == 3
    assert lint("---\ntitle: a; b\n---\n", "a.txt")[0] != []
    assert "semicolon" in _rules_of(lint("---\nA rule; above.\n---\n", "a.md")[0])
    # a stray fence of the other kind does not flip the state
    assert lint("```\n~~~\na; b\n```\nA clean line.")[0] == []
    # commit messages: the subject token and the trailers are not prose
    commit = "skill-design-k5: design the layers\n\nExplain the change.\n\nCo-Authored-By: A B <a@b.c>\n"
    assert lint(commit, text_type="commit")[0] == []
    assert lint("fix(scope)!: x; y", text_type="commit")[0] != []

    # British spelling
    for american, british in (("organization", "organisation"), ("analyze", "analyse"),
                              ("analyzing", "analysing"), ("color", "colour"),
                              ("behavior", "behaviour"), ("center", "centre"),
                              ("defense", "defence"), ("labeled", "labelled"),
                              ("modeling", "modelling"), ("canceled", "cancelled"),
                              ("gray", "grey"), ("fulfill", "fulfil")):
        found = [f for f in lint(f"The {american} is here.")[0] if f["rule"] == "spelling"]
        assert len(found) == 1 and british in found[0]["message"], (american, found)
        assert found[0]["col"] == 5, found
    for fine in ("size", "sized", "resize", "prize", "seize", "capsize", "maize",
                 "organisation", "analyse", "colour", "program", "disk", "dialog",
                 "license", "practice", "meter", "judgment", "learned", "humorous",
                 "rigorous", "vigorous", "honorary", "laboratory", "collaborate",
                 "centered", "central", "color_scheme", "--color", "iPhone", "ISO",
                 "analyzer_name", "organization.py", "labels", "modelled"):
        text = f"The {fine} is here."
        if fine in ("centered", "color_scheme", "--color", "organization.py"):
            found = [f for f in lint(text)[0] if f["rule"] == "spelling"]
            assert (fine == "centered") == bool(found), (fine, found)
            continue
        assert "spelling" not in _rules_of(lint(text)[0]), fine
    assert "spelling" not in _rules_of(lint("The `color` and `organization` keys.")[0])
    assert "spelling" not in _rules_of(lint("The Color Picker opens.")[0])
    assert "spelling" in _rules_of(lint("Color is here.")[0])
    assert "spelling" in _rules_of(lint("It ends. Color is here.")[0])
    assert "spelling" in _rules_of(lint("A color-coded table.")[0])
    assert "spelling" not in _rules_of(lint('The word "color" is here.')[0])

    # markers: justified departure, bare departure, scope
    long_line = "The " + " ".join(["word"] * 29) + "."
    marker = "<!-- tc: depart long-sentence because a name cannot be split -->"
    result = analyse(f"{marker}\n{long_line}\n\n{long_line}\n")
    assert [f["line"] for f in result["findings"]] == [4], result
    assert result["accepted"] == {"long-sentence": 1}, result
    result = analyse(f"{long_line} {marker}\n\n{long_line}\n")
    assert [f["line"] for f in result["findings"]] == [3], result
    result = analyse(f"- {long_line} {marker}\n- {long_line}\n")
    assert [f["line"] for f in result["findings"]] == [2], result
    result = analyse(f"{marker}\n| a | b |\n| --- | --- |\n| {long_line} | x |\n| y | z |\n")
    assert [f["line"] for f in result["findings"]] == [4], result       # the marker applies to the header row
    result = analyse("<!-- tc: depart long-sentence because x -->\n# Heading\n" + long_line + "\n")
    assert [f["line"] for f in result["findings"]] == [3], result       # a heading is a block of its own
    result = analyse("<!-- tc: depart spelling because x -->\n" + long_line + "\n")
    assert _rules_of(result["findings"]) == ["long-sentence"], result   # only the named rule is accepted
    result = analyse("<!-- tc: depart long-sentence,spelling because x -->\nThe color. " + long_line + "\n")
    assert result["findings"] == [] and result["accepted"] == {"long-sentence": 1, "spelling": 1}, result
    for bad, expected in (("<!-- tc: depart long-sentence -->", "because"),
                          ("<!-- tc: depart long-sentence because -->", "because"),
                          ("<!-- tc: depart because x -->", "rules"),
                          ("<!-- tc: depart nonsense because x -->", "nonsense"),
                          ("<!-- tc: depart marker because x -->", "marker"),
                          ("<!-- tc: depart long-sentence, spelling because x -->", "because"),
                          ("<!-- tc: wave because x -->", "wave"),
                          ("<!-- tc: -->", "no verb"),
                          ("<!-- tc: off -->", "because"),
                          ("<!-- tc: off long-sentence because x -->", "no rules"),
                          ("<!-- tc: on because x -->", "no rules"),
                          ("<!-- tc: on -->", "closes nothing"),
                          ("<!-- tc: exempt -->", "because")):
        result = analyse(f"{bad}\n{long_line}\n")
        markers = [f for f in result["findings"] if f["rule"] == "marker"]
        assert len(markers) == 1 and expected in markers[0]["message"], (bad, result)
        assert markers[0]["level"] == "hard" and markers[0]["line"] == 1, markers
        assert "long-sentence" in _rules_of(result["findings"]), bad     # a faulty marker does not apply
    assert lint("<!-- tc but not a marker -->\nA clean line.")[0] == []
    assert lint("<!-- TC: not ours -->\nA clean line.")[0] == []
    assert lint("`<!-- tc: wave because x -->` is the form.")[0] == []
    assert lint("```\n<!-- tc: wave because x -->\n```\nA clean line.")[0] == []
    # off and pending regions are not checked, and end at on or at the end of the file
    result = analyse(f"<!-- tc: off because quoted -->\n{long_line}\n<!-- tc: on -->\n{long_line}\n")
    assert [f["line"] for f in result["findings"]] == [4], result
    result = analyse(f"{long_line}\n<!-- tc: off because quoted -->\n{long_line}\nMore; text.\n")
    assert [f["line"] for f in result["findings"]] == [1], result
    assert result["unclosed"] == "off", result
    result = analyse(f"<!-- tc: pending because later -->\n{long_line}\n<!-- tc: on -->\n")
    assert result["findings"] == [] and result["pending"] == 1 and result["words"] == 0, result
    assert result["unclosed"] is None, result
    result = analyse("<!-- tc: off because a -->\n<!-- tc: pending because b -->\n<!-- tc: on -->\n")
    assert [f["rule"] for f in result["findings"]] == ["marker"], result
    # exempt: a document is not checked
    result = analyse(f"{long_line}\n\n<!-- tc: exempt because the human asked for this form -->\n")
    assert result["findings"] == [] and result["exempt"] and result["words"] == 0, result
    # a reply: only exempt has an effect
    result = analyse(f"<!-- tc: off because x -->\n{long_line}\n", reply=True)
    assert _rules_of(result["findings"]) == ["long-sentence"], result
    result = analyse(f"{long_line}\n<!-- tc: exempt because x -->", reply=True)
    assert result["exempt"], result
    # the first prose line and the markers are visible to a gate
    result = analyse("<!-- tc: off because x -->\n\nText.\n")
    assert result["first_prose_line"] == 3 and result["markers"][0]["line"] == 1, result

    # exit codes and output
    with tempfile.TemporaryDirectory() as tmp:
        good = os.path.join(tmp, "good.md")
        poor = os.path.join(tmp, "poor.md")
        binary = os.path.join(tmp, "binary.md")
        with open(good, "w", encoding="utf-8") as fh:
            fh.write("A short line.\n")
        with open(poor, "w", encoding="utf-8") as fh:
            fh.write("A short; line.\n<!-- tc: depart semicolon because x -->\nAnother; one.\n")
        with open(binary, "wb") as fh:
            fh.write(b"\xff\xfe not utf-8 \x80\n")

        def run(args):
            out = io.StringIO()
            with contextlib.redirect_stdout(out), contextlib.redirect_stderr(io.StringIO()):
                try:
                    code = main(args)
                except SystemExit as exit_:
                    code = exit_.code
            return code, out.getvalue()

        assert run([good])[0] == 0
        code, out = run([poor])
        assert code == 1 and f"{poor}:1:8 semicolon:" in out, (code, out)
        assert "1 hard, 0 advisory, 1 accepted, 0 pending" in out, out
        assert run(["--baseline", "1", poor])[0] == 0
        assert run(["--disable", "semicolon", poor])[0] == 0
        assert run([binary])[0] == 2
        assert run([os.path.join(tmp, "missing.md")])[0] == 2
        code, out = run(["--json", poor])
        data = json.loads(out)
        assert code == 1 and data["hard"] == 1 and data["advisory"] == 0, data
        assert data["accepted"] == {"semicolon": 1} and data["pending"] == 0, data
        assert data["text_type"] == "auto" and data["baseline"] == 0, data
        assert data["findings"][0]["match"] == ";", data
        assert data["per_100"] == round(100 / data["words"], 1), data
        assert run(["--text-type", "instruction", good])[0] == 0
        code, out = run(["--markers", poor])
        assert code == 0 and f"{poor}:2 depart semicolon because x" in out, (code, out)
        faulty = os.path.join(tmp, "faulty.md")
        with open(faulty, "w", encoding="utf-8") as fh:
            fh.write("<!-- tc: depart semicolon -->\n")
        code, out = run(["--markers", faulty])
        assert code == 1 and f"{faulty}:1:1 marker:" in out, (code, out)
# ours: end


# ours: begin
def _selftest_log():
    """Assertions for the reply log. They write only into a temporary state directory."""
    saved = os.environ.get(STATE_DIR_VARIABLE)
    try:
        with tempfile.TemporaryDirectory() as tmp:
            os.environ[STATE_DIR_VARIABLE] = os.path.join(tmp, "state")
            sessions = os.path.join(tmp, "state", "sessions")

            def records(name):
                """The records of a session file; a line that is not JSON is skipped."""
                found = []
                with open(os.path.join(sessions, name), encoding="utf-8") as fh:
                    for line in fh:
                        try:
                            found.append(json.loads(line))
                        except ValueError:
                            assert not line.endswith("\n") or line.startswith('{"v":1,"kind":"rep'), line
                return found

            def run_log(args, stdin=""):
                saved_stdin = sys.stdin
                sys.stdin = io.TextIOWrapper(io.BytesIO(stdin.encode("utf-8")))
                try:
                    with contextlib.redirect_stdout(io.StringIO()) as out, \
                            contextlib.redirect_stderr(io.StringIO()) as err:
                        code = main(args)
                finally:
                    sys.stdin = saved_stdin
                assert out.getvalue() == "" and err.getvalue() == "", (out.getvalue(), err.getvalue())
                return code

            long_line = " ".join(["word"] * 30) + "."
            who = ["--harness", "codex", "--session", "a/b c"]
            # a reply before any start record: since_start equals seq
            assert run_log(["--log-reply", *who, "--model", "m1"], "A short line here.\n") == 0
            assert run_log(["--log-start", *who, "--source", "startup"]) == 0
            assert run_log(["--log-reply", *who], f"{long_line}\n") == 0
            assert run_log(["--log-reply", *who], "Fine; but not.\n") == 0
            # another agent counts apart, and a start resets only its own count
            assert run_log(["--log-start", *who, "--agent", "sub1", "--source", "subagent"]) == 0
            assert run_log(["--log-reply", *who, "--agent", "sub1"], "A short line here.\n") == 0
            assert run_log(["--log-start", *who, "--source", "compact"]) == 0
            assert run_log(["--log-reply", *who], "A short line here.\n") == 0
            got = records("codex--a_b_c.jsonl")
            assert [(r["kind"], r["agent"]) for r in got] == [
                ("reply", None), ("start", None), ("reply", None), ("reply", None),
                ("start", "sub1"), ("reply", "sub1"), ("start", None), ("reply", None)], got
            replies = [r for r in got if r["kind"] == "reply" and r["agent"] is None]
            assert [(r["seq"], r["since_start"]) for r in replies] == [
                (1, 1), (2, 1), (3, 2), (4, 1)], replies
            assert next(r for r in got if r["agent"] == "sub1" and r["kind"] == "reply"
                        )["seq"] == 1
            first = replies[0]
            assert first["v"] == 1 and first["model"] == "m1" and first["session"] == "a/b c", first
            assert first["words"] == 4 and first["hard"] == 0 and first["rules"] == {}, first
            assert first["exempt"] is False and first["checker"] == checker_digest(), first
            assert re.fullmatch(r"\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ", first["ts"]), first
            assert replies[1]["rules"] == {"long-sentence": 1} and replies[1]["per_100"] > 0, replies[1]
            assert replies[2]["rules"] == {"semicolon": 1} and replies[2]["hard"] == 1, replies[2]
            assert got[1]["source"] == "startup" and got[4]["source"] == "subagent", got
            # the log stores no text
            assert "word word" not in open(os.path.join(sessions, "codex--a_b_c.jsonl")).read()
            # an advisory finding is counted in the rules and not in per_100
            run_log(["--log-reply", "--harness", "pi", "--session", "s"],
                    "The panel is removed by the agent.\n")
            advisory = records("pi--s.jsonl")[0]
            assert advisory["advisory"] >= 1 and advisory["per_100"] == 0.0, advisory
            assert "passive-voice" in advisory["rules"], advisory
            # exempt: scored, and marked
            run_log(["--log-reply", "--harness", "pi", "--session", "s"],
                    f"{long_line}\n\n<!-- tc: exempt because the human asked -->\n")
            exempt = records("pi--s.jsonl")[1]
            assert exempt["exempt"] is True and exempt["words"] == 30 and exempt["hard"] == 1, exempt
            # a reply with no prose words writes no record
            for empty in ("", "\n", "```\ncode only\n```\n", "<!-- tc: exempt because x -->\n"):
                run_log(["--log-reply", "--harness", "pi", "--session", "s"], empty)
            assert len(records("pi--s.jsonl")) == 2
            # a missing session is "unknown"
            run_log(["--log-start", "--harness", "claude-code", "--source", "resume"])
            assert records("claude-code--unknown.jsonl")[0]["session"] == "unknown"
            # a record that a crash left without its newline does not join the next
            path = os.path.join(sessions, "pi--s.jsonl")
            with open(path, "ab") as fh:
                fh.write(b'{"v":1,"kind":"rep')
            run_log(["--log-reply", "--harness", "pi", "--session", "s"], "A short line here.\n")
            assert [r["seq"] for r in records("pi--s.jsonl")] == [1, 2, 3]
            assert open(path, encoding="utf-8").read().count("\n") == 4
            # concurrent writers do not corrupt the file or repeat a seq
            threads = [threading.Thread(target=log_reply, args=(
                "A short line here.\n", "pi", "race", None, None)) for _ in range(24)]
            for thread in threads:
                thread.start()
            for thread in threads:
                thread.join()
            raced = records("pi--race.jsonl")
            assert sorted(r["seq"] for r in raced) == list(range(1, 25)), raced
            # a failure writes nothing and exits 0
            blocker = os.path.join(tmp, "blocker")
            with open(blocker, "w", encoding="utf-8") as fh:
                fh.write("a file, not a directory")
            os.environ[STATE_DIR_VARIABLE] = os.path.join(blocker, "state")
            assert run_log(["--log-reply", *who], "A short line here.\n") == 0
            assert run_log(["--log-start", *who, "--source", "startup"]) == 0
            os.environ[STATE_DIR_VARIABLE] = os.path.join(tmp, "state")
            for bad in (["--log-reply", "--nonsense"], ["--log-start", "--log-reply"],
                        ["--log-reply", "--harness"]):
                assert run_log(bad, "A short line here.\n") == 0, bad
            # a reply that is not UTF-8 is still scored
            saved_stdin = sys.stdin
            sys.stdin = io.TextIOWrapper(io.BytesIO(b"A short \xff line here.\n"))
            try:
                assert main(["--log-reply", "--harness", "pi", "--session", "bytes"]) == 0
            finally:
                sys.stdin = saved_stdin
            # the replacement character for the bad byte is not a word
            assert records("pi--bytes.jsonl")[0]["words"] == 4
    finally:
        if saved is None:
            os.environ.pop(STATE_DIR_VARIABLE, None)
        else:
            os.environ[STATE_DIR_VARIABLE] = saved
    # without the variable the state directory follows XDG_STATE_HOME
    saved_xdg = os.environ.pop(STATE_DIR_VARIABLE, None), os.environ.get("XDG_STATE_HOME")
    try:
        os.environ["XDG_STATE_HOME"] = "/xdg"
        assert state_dir() == "/xdg/technical-communication", state_dir()
        os.environ.pop("XDG_STATE_HOME")
        assert state_dir() == os.path.join(os.path.expanduser("~"), ".local", "state",
                                           "technical-communication"), state_dir()
    finally:
        for name, value in zip((STATE_DIR_VARIABLE, "XDG_STATE_HOME"), saved_xdg):
            if value is not None:
                os.environ[name] = value
# ours: end


RULE_IDS = {rule_id for rule_id, *_ in RULES} | {
    # ours: the new rules
    "long-sentence", "long-paragraph", "synonym-rotation", "dangling-conjunction",
    "spelling", "marker"}


def _rule_list(value):
    rules = {r for r in value.split(",") if r}
    unknown = rules - RULE_IDS
    if unknown:
        raise argparse.ArgumentTypeError(
            f"unknown rule(s): {', '.join(sorted(unknown))}. "
            f"Known rules: {', '.join(sorted(RULE_IDS))}")
    return rules


def _non_negative_int(value):
    try:
        n = int(value)
    except ValueError:
        n = -1
    if n < 0:
        raise argparse.ArgumentTypeError(f"expected a non-negative integer, got {value!r}")
    return n


def main(argv):
    # ours: the log modes take their own arguments and never fail
    if "--log-start" in argv or "--log-reply" in argv:
        return log_main(argv)
    parser = argparse.ArgumentParser(
        prog="ste-lint.py", allow_abbrev=False,
        description="Deterministic checker for the structural rules of the "  # ours
                    "technical-communication skill. "
                    "Reads stdin when no FILE is given.")
    parser.add_argument("paths", nargs="*", metavar="FILE")
    parser.add_argument("--json", action="store_true", help="print findings as JSON")
    parser.add_argument("--baseline", type=_non_negative_int, default=0, metavar="N",
                        help="pass unless hard findings exceed N (default 0)")  # ours
    parser.add_argument("--disable", type=_rule_list, default=set(), metavar="RULES",
                        help="comma-separated rule IDs to silence")
    # ours: begin the new options
    parser.add_argument("--text-type", choices=TEXT_TYPES, default="auto",
                        help="auto (default) tells instructions from descriptions "
                             "by sentence; instruction and description set one "
                             "type for all the text; commit reads a commit message")
    parser.add_argument("--markers", action="store_true",
                        help="list each marker and each faulty marker, and check nothing else")
    # ours: end
    parser.add_argument("--selftest", action="store_true", help="run the built-in tests")
    # intermixed parsing keeps flags valid between file paths
    args = parser.parse_intermixed_args(argv)
    if args.selftest:
        selftest()
        return 0
    as_json, baseline, disabled, paths = args.json, args.baseline, args.disable, args.paths

    # ours: begin read each text as UTF-8; a file that is not UTF-8 is a usage error
    sources = []
    if paths:
        for p in paths:
            try:
                with open(p, "rb") as fh:
                    data = fh.read()
            except OSError as e:
                parser.error(f"cannot read {p}: {e.strerror}")
            try:
                sources.append((p, data.decode("utf-8")))
            except UnicodeDecodeError:
                parser.error(f"cannot read {p}: not UTF-8")
    else:
        try:
            sources.append(("<stdin>", sys.stdin.buffer.read().decode("utf-8")))
        except UnicodeDecodeError:
            parser.error("cannot read standard input: not UTF-8")
    sources = [(name, text[1:] if text.startswith("﻿") else text)
               for name, text in sources]

    if args.markers:
        entries = []
        for name, text in sources:
            for m in analyse(text, name, args.text_type)["markers"]:
                entries.append({"file": name, "line": m["line"], "col": m["col"],
                                "verb": m["verb"], "rules": m["rules"],
                                "reason": m["reason"], "error": m["error"]})
        report_markers(entries, as_json)
        return 1 if any(e["error"] is not None for e in entries) else 0

    results = []
    for name, text in sources:
        result = analyse(text, name, args.text_type)
        result["findings"] = [f for f in result["findings"] if f["rule"] not in disabled]
        results.append(result)
    total = _tally(results)
    report(total, args.text_type, as_json, baseline)
    return 1 if total["hard"] > baseline else 0
    # ours: end


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
