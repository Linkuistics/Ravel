"""The arithmetic of the baseline and weighted pattern reports.

A report scores the fixed patterns of one discovery run. It does not discover patterns.
The formula, the normalisation and the denominators are in ``docs/stages/weighted-report.md``.

Four inputs stay apart, and no function here combines them into one score:

- A **preference weight** changes the share of a book or the weight of a section. Only
  preference weights enter the arithmetic.
- A **relevance assessment** is a category. The report counts supporting instances by level.
- **Extraction uncertainty** comes from the defect records of the chapter extraction.
- **Interpretation uncertainty** comes from the uncertainty field of each interpretation and
  from its alternative readings.
"""

from __future__ import annotations

import math
from collections import Counter
from collections.abc import Callable, Iterable, Mapping, Sequence
from dataclasses import dataclass
from typing import Any, Literal

from pydantic import BaseModel, ConfigDict, Field

from urg.errors import UrgError

Json = dict[str, Any]

OBSERVATION_LAYERS = ("structure-section", "interpretation")
NOT_ASSESSED = "not assessed"
MIXED = "mixed"

Status = Literal["illustrative", "human"]
Level = Literal["high", "medium", "low"]


class WeightingError(UrgError):
    """The weights or the relevance assessments cannot score the patterns."""


# --- Inputs ---------------------------------------------------------------------------


class WeightEntry(BaseModel):
    model_config = ConfigDict(extra="forbid")

    weight: float = Field(ge=0, allow_inf_nan=False)
    reason: str = Field(min_length=1)


class WeightConfig(BaseModel):
    """Preference weights for books and sections.

    ``status`` says who chose the weights. A report shows ``illustrative`` weights as
    illustrative and never as the preferences of the human.
    """

    model_config = ConfigDict(extra="forbid")

    version: int = Field(ge=1)
    status: Status
    statement: str = Field(min_length=1)
    default_book_weight: float = Field(default=1.0, ge=0, allow_inf_nan=False)
    books: dict[str, WeightEntry] = Field(default_factory=dict[str, WeightEntry])
    default_section_weight: float = Field(default=1.0, ge=0, allow_inf_nan=False)
    sections: dict[str, WeightEntry] = Field(default_factory=dict[str, WeightEntry])

    def book_weight(self, item_key: str) -> float:
        entry = self.books.get(item_key)
        return entry.weight if entry is not None else self.default_book_weight

    def section_weight(self, section_id: str, parents: Mapping[str, str | None]) -> float:
        """Return the weight of the section, or of its nearest weighted ancestor."""
        for candidate in lineage(section_id, parents):
            entry = self.sections.get(candidate)
            if entry is not None:
                return entry.weight
        return self.default_section_weight


class Assessment(BaseModel):
    model_config = ConfigDict(extra="forbid")

    level: Level
    reason: str = Field(min_length=1)


class RelevanceConfig(BaseModel):
    """Relevance assessments of books and sections. They never change a weight."""

    model_config = ConfigDict(extra="forbid")

    version: int = Field(ge=1)
    status: Status
    statement: str = Field(min_length=1)
    books: dict[str, Assessment] = Field(default_factory=dict[str, Assessment])
    sections: dict[str, Assessment] = Field(default_factory=dict[str, Assessment])

    def level(
        self, item_key: str, section_id: str | None, parents: Mapping[str, str | None]
    ) -> str:
        """Return the level of the section, its nearest assessed ancestor, or else the book."""
        if section_id is not None:
            for candidate in lineage(section_id, parents):
                entry = self.sections.get(candidate)
                if entry is not None:
                    return entry.level
        book = self.books.get(item_key)
        return book.level if book is not None else NOT_ASSESSED


def lineage(section_id: str, parents: Mapping[str, str | None]) -> Iterable[str]:
    """Yield the section and then its ancestors, nearest first."""
    current: str | None = section_id
    seen: set[str] = set()
    while current is not None and current not in seen:
        seen.add(current)
        yield current
        current = parents.get(current)


# --- Observations ---------------------------------------------------------------------


@dataclass(frozen=True)
class Observation:
    """One observation that a pattern can cite, with what the report needs to know about it.

    ``sections`` holds the section itself, or the sections of the evidence that an
    interpretation cites. ``defects`` holds the IDs of the extraction defects that touch the
    evidence of the observation. ``interpretation_uncertainty`` is ``None`` for a section.
    """

    id: str
    layer: str
    item_key: str
    label: str
    sections: tuple[str, ...]
    defects: tuple[str, ...] = ()
    interpretation_uncertainty: str | None = None
    alternative_readings: tuple[str, ...] = ()


@dataclass(frozen=True)
class Material:
    """The analysed chapter of one book."""

    item_key: str
    extraction_record: Json
    analysis_record: Json


@dataclass(frozen=True)
class Corpus:
    """The observations of one discovery run, and the facts about their chapters."""

    books: tuple[str, ...]
    observations: dict[str, Observation]
    parents: dict[str, str | None]
    defects: dict[str, tuple[Json, ...]]
    chapter_defects: dict[str, tuple[str, ...]]
    section_titles: dict[str, str]


def _defect_touches(defect: Json, lines: set[str], pages: set[int]) -> bool:
    evidence: list[str] = defect.get("evidence") or []
    if evidence:
        return bool(lines.intersection(evidence))
    page = defect.get("pdf_page_index")
    return page is not None and page in pages


def _touching(
    book_defects: Sequence[Json],
    elements: Mapping[str, Json],
    line_page: Mapping[str, int | None],
    element_ids: Iterable[str],
    line_ids: Iterable[str],
) -> tuple[str, ...]:
    """Return the defects that touch the evidence lines or pages of some elements and lines."""
    lines = set(line_ids)
    pages: set[int] = set()
    for element_id in element_ids:
        element = elements.get(element_id)
        if element is None:
            continue
        lines.update(element.get("evidence", []))
        if element.get("pdf_page_index") is not None:
            pages.add(element["pdf_page_index"])
    pages.update(p for line in lines if (p := line_page.get(line)) is not None)
    return tuple(d["id"] for d in book_defects if _defect_touches(d, lines, pages))


def build_corpus(record: Json, materials: Sequence[Material]) -> Corpus:
    """Collect the listed observations of a discovery record with their sections and doubts.

    The listed observations are the inventory entries in ``OBSERVATION_LAYERS``. They are
    the observations that a pattern can cite and that the unfitted list covers.
    """
    observations: dict[str, Observation] = {}
    parents: dict[str, str | None] = {}
    defects: dict[str, tuple[Json, ...]] = {}
    chapter_defects: dict[str, tuple[str, ...]] = {}
    titles: dict[str, str] = {}
    listed = {
        entry_id: entry
        for entry_id, entry in record["inventory"].items()
        if entry["layer"] in OBSERVATION_LAYERS
    }
    for material in materials:
        key = material.item_key
        structure = material.extraction_record["structure"]
        elements: dict[str, Json] = {e["id"]: e for e in structure["elements"]}
        element_section: dict[str, str] = {}
        section_elements: dict[str, list[str]] = {}
        for section in structure.get("sections", []):
            parents[section["id"]] = section.get("parent")
            titles[section["id"]] = section["title"]
            section_elements[section["id"]] = list(section.get("elements", []))
            for element_id in section.get("elements", []):
                element_section.setdefault(element_id, section["id"])
        for element in elements.values():
            if element.get("section") and element["id"] not in element_section:
                element_section[element["id"]] = element["section"]
        line_element = {
            line: element_id
            for element_id, element in elements.items()
            for line in element.get("evidence", [])
        }
        line_page: dict[str, int | None] = {
            line["id"]: line.get("pdf_page_index")
            for line in material.extraction_record.get("evidence", [])
        }
        book_defects: list[Json] = list(material.extraction_record.get("defects", []))
        defects[key] = tuple(book_defects)
        chapter_defects[key] = tuple(
            d["id"]
            for d in book_defects
            if not d.get("evidence") and d.get("pdf_page_index") is None
        )

        alternatives: dict[str, list[str]] = {}
        interpretations: dict[str, Json] = {}
        for interpretation in material.analysis_record["interpretations"]:
            interpretations[interpretation["id"]] = interpretation
            target = interpretation.get("alternative_to")
            if interpretation["kind"] == "alternative-reading" and target:
                alternatives.setdefault(target["id"], []).append(interpretation["id"])

        for entry_id, entry in listed.items():
            if entry["item_key"] != key:
                continue
            if entry["layer"] == "structure-section":
                observations[entry_id] = Observation(
                    id=entry_id,
                    layer=entry["layer"],
                    item_key=key,
                    label=entry.get("title", entry_id),
                    sections=(entry_id,),
                    defects=_touching(
                        book_defects, elements, line_page, section_elements.get(entry_id, []), []
                    ),
                )
                continue
            interpretation = interpretations.get(entry_id)
            if interpretation is None:
                raise WeightingError(
                    f"interpretation {entry_id} is in the discovery inventory "
                    "but not in the analysis of its book"
                )
            element_ids: list[str] = []
            line_ids: list[str] = []
            for evidence in interpretation["evidence"]:
                if evidence["layer"] == "structure-element":
                    element_ids.append(evidence["id"])
                elif evidence["layer"] == "evidence-line":
                    line_ids.append(evidence["id"])
                    if evidence["id"] in line_element:
                        element_ids.append(line_element[evidence["id"]])
            sections = tuple(
                dict.fromkeys(element_section[e] for e in element_ids if e in element_section)
            )
            observations[entry_id] = Observation(
                id=entry_id,
                layer=entry["layer"],
                item_key=key,
                label=interpretation["label"],
                sections=sections,
                defects=_touching(book_defects, elements, line_page, element_ids, line_ids),
                interpretation_uncertainty=interpretation["uncertainty"]["level"],
                alternative_readings=tuple(alternatives.get(entry_id, [])),
            )
    return Corpus(
        books=tuple(m.item_key for m in materials),
        observations=observations,
        parents=parents,
        defects=defects,
        chapter_defects=chapter_defects,
        section_titles=titles,
    )


# --- Arithmetic -----------------------------------------------------------------------


@dataclass(frozen=True)
class PatternCitations:
    """The cited observation IDs of one pattern, by role."""

    id: str
    supporting: tuple[str, ...]
    exceptions: tuple[str, ...] = ()
    unresolved: tuple[str, ...] = ()

    @staticmethod
    def of(pattern: Json) -> PatternCitations:
        def ids(role: str) -> tuple[str, ...]:
            return tuple(c["observation"]["id"] for c in pattern[role])

        return PatternCitations(
            id=pattern["id"],
            supporting=ids("supporting"),
            exceptions=ids("exceptions"),
            unresolved=ids("unresolved"),
        )


@dataclass(frozen=True)
class BookTerm:
    """The contribution of one book to the prevalence of one pattern."""

    item_key: str
    share: float
    supporting_weight: float
    exception_weight: float
    denominator: float

    @property
    def fraction(self) -> float:
        return self.supporting_weight / self.denominator if self.denominator > 0 else 0.0

    @property
    def exception_fraction(self) -> float:
        return self.exception_weight / self.denominator if self.denominator > 0 else 0.0


@dataclass(frozen=True)
class PatternScore:
    pattern: str
    terms: tuple[BookTerm, ...]
    uncounted: tuple[str, ...]

    @property
    def prevalence(self) -> float:
        """The share-weighted mean over books of the weighted supporting fraction."""
        return math.fsum(t.share * t.fraction for t in self.terms)

    @property
    def exception_prevalence(self) -> float:
        return math.fsum(t.share * t.exception_fraction for t in self.terms)

    @property
    def book_coverage(self) -> float:
        """The total share of the books that have weighted supporting material."""
        return math.fsum(t.share for t in self.terms if t.supporting_weight > 0)


@dataclass(frozen=True)
class Scores:
    book_weights: dict[str, float]
    shares: dict[str, float]
    denominators: dict[str, float]
    excluded: dict[str, str]
    patterns: dict[str, PatternScore]

    def ranks(self) -> dict[str, int]:
        """Rank the patterns by prevalence. Equal prevalences share the better rank."""
        ordered = sorted(self.patterns.values(), key=lambda s: (-s.prevalence, s.pattern))
        ranks: dict[str, int] = {}
        previous: float | None = None
        rank = 0
        for position, score in enumerate(ordered, start=1):
            if previous is None or not math.isclose(score.prevalence, previous, abs_tol=1e-12):
                rank = position
                previous = score.prevalence
            ranks[score.pattern] = rank
        return ranks


def score_patterns(
    corpus: Corpus,
    patterns: Sequence[PatternCitations],
    book_weight: Callable[[str], float],
    observation_weight: Callable[[Observation], float],
) -> Scores:
    """Score fixed patterns under one book weight and one observation weight.

    s_b = w_b / sum of w over the books with weighted material
    f_b(P) = sum of v_o over the supporting instances of P in b / sum of v_o over b
    prevalence(P) = sum over b of s_b * f_b(P)
    """
    weights = {o.id: observation_weight(o) for o in corpus.observations.values()}
    for observation_id, value in weights.items():
        if not math.isfinite(value) or value < 0:
            raise WeightingError(f"observation {observation_id} has weight {value}")
    denominators = {
        b: math.fsum(weights[o.id] for o in corpus.observations.values() if o.item_key == b)
        for b in corpus.books
    }
    book_weights = {b: book_weight(b) for b in corpus.books}
    excluded: dict[str, str] = {}
    for b in corpus.books:
        if denominators[b] <= 0:
            excluded[b] = "no listed observation of this book has a positive weight"
        elif book_weights[b] <= 0:
            excluded[b] = "the book weight is zero"
    total = math.fsum(book_weights[b] for b in corpus.books if b not in excluded)
    if total <= 0:
        raise WeightingError("the weights give no analysed book a positive share")
    shares = {b: (0.0 if b in excluded else book_weights[b] / total) for b in corpus.books}

    scored: dict[str, PatternScore] = {}
    for pattern in patterns:
        cited = [*pattern.supporting, *pattern.exceptions]
        uncounted = tuple(i for i in dict.fromkeys(cited) if i not in corpus.observations)

        def total_in(book: str, ids: Sequence[str]) -> float:
            return math.fsum(
                weights[i]
                for i in dict.fromkeys(ids)
                if i in corpus.observations and corpus.observations[i].item_key == book
            )

        terms = tuple(
            BookTerm(
                item_key=b,
                share=shares[b],
                supporting_weight=total_in(b, pattern.supporting),
                exception_weight=total_in(b, pattern.exceptions),
                denominator=denominators[b],
            )
            for b in corpus.books
        )
        scored[pattern.id] = PatternScore(pattern.id, terms, uncounted)
    return Scores(book_weights, shares, denominators, excluded, scored)


def baseline_scores(corpus: Corpus, patterns: Sequence[PatternCitations]) -> Scores:
    """Score the patterns under the equal-book baseline: every book and observation weighs 1."""
    return score_patterns(corpus, patterns, lambda _: 1.0, lambda _: 1.0)


def weighted_scores(
    corpus: Corpus, patterns: Sequence[PatternCitations], config: WeightConfig
) -> Scores:
    """Score the patterns under supplied preference weights.

    An observation weighs the mean of the weights of its sections. An observation without a
    located section weighs the default section weight.
    """

    def observation_weight(observation: Observation) -> float:
        if not observation.sections:
            return config.default_section_weight
        return math.fsum(
            config.section_weight(s, corpus.parents) for s in observation.sections
        ) / len(observation.sections)

    return score_patterns(corpus, patterns, config.book_weight, observation_weight)


# --- Separate columns -----------------------------------------------------------------


def relevance_levels(
    corpus: Corpus, observation_ids: Iterable[str], relevance: RelevanceConfig
) -> Counter[str]:
    """Count the cited observations by relevance level. Levels never enter a score."""
    counts: Counter[str] = Counter()
    for observation_id in dict.fromkeys(observation_ids):
        observation = corpus.observations.get(observation_id)
        if observation is None:
            continue
        sections: Sequence[str | None] = observation.sections or (None,)
        levels = {relevance.level(observation.item_key, s, corpus.parents) for s in sections}
        counts[levels.pop() if len(levels) == 1 else MIXED] += 1
    return counts


def interpretation_uncertainty_levels(
    corpus: Corpus, observation_ids: Iterable[str]
) -> tuple[Counter[str], int]:
    """Count the cited interpretations by uncertainty level, and their alternative readings."""
    counts: Counter[str] = Counter()
    alternatives = 0
    for observation_id in dict.fromkeys(observation_ids):
        observation = corpus.observations.get(observation_id)
        if observation is None or observation.interpretation_uncertainty is None:
            continue
        counts[observation.interpretation_uncertainty] += 1
        alternatives += len(observation.alternative_readings)
    return counts, alternatives


def extraction_uncertainty(corpus: Corpus, observation_ids: Iterable[str]) -> tuple[int, set[str]]:
    """Return how many cited observations rest on defective evidence, and which defects."""
    touched = 0
    defects: set[str] = set()
    for observation_id in dict.fromkeys(observation_ids):
        observation = corpus.observations.get(observation_id)
        if observation is None or not observation.defects:
            continue
        touched += 1
        defects.update(observation.defects)
    return touched, defects


def ignored_entries(corpus: Corpus, books: Iterable[str], sections: Iterable[str]) -> list[str]:
    """Name the configured books and sections that the discovery run did not analyse."""
    ignored = [
        f"book {b}: not analysed in this discovery run" for b in books if b not in corpus.books
    ]
    ignored += [
        f"section {s}: not a section of an analysed chapter"
        for s in sections
        if s not in corpus.parents
    ]
    return ignored
