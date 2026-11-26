"""The ``urg`` command line."""

from __future__ import annotations

from pathlib import Path
from typing import Annotated

import typer

from urg import changes, corrections, discovery, report, weighted_report
from urg.analysis import DEFAULT_INSTRUCTIONS, DEFAULT_MODEL_CONFIG, run_analysis
from urg.errors import UrgError
from urg.extraction import latest_extraction, load_config, render_anchor, run_extraction
from urg.manifest import run_import
from urg.model import load_model_config, model_from_config
from urg.store import ArtifactStore

app = typer.Typer(no_args_is_help=True, add_completion=False)


@app.callback()
def main() -> None:
    """Run the stages of the URG pipeline."""


@app.command("catalogue-manifest")
def catalogue_manifest(
    rdf: Annotated[Path, typer.Option(help="The Zotero RDF export.")],
    zotero_dir: Annotated[Path, typer.Option(help="The Zotero data directory.")],
    data_dir: Annotated[Path, typer.Option(help="The research-artifact store.")] = Path("data"),
) -> None:
    """Import the Zotero catalogue and write the source manifest."""
    try:
        result = run_import(rdf, zotero_dir.expanduser(), ArtifactStore(data_dir))
    except UrgError as e:
        typer.echo(f"error: {e}", err=True)
        raise typer.Exit(1) from e
    typer.echo(
        "reused the stored research artifacts" if result.reused else "wrote new research artifacts"
    )
    for artifact in (result.export_snapshot, result.source_manifest, result.alias_table):
        typer.echo(f"{artifact.record.dependency.artifact}: {artifact.directory}")
    manifest = result.source_manifest
    typer.echo(f"summary: {manifest.path('summary.md')}")


@app.command("chapter-extraction")
def chapter_extraction(
    selection: Annotated[Path, typer.Option(help="The chapter-selection record.")] = Path(
        "docs/probes/chapter-selection.json"
    ),
    data_dir: Annotated[Path, typer.Option(help="The research-artifact store.")] = Path("data"),
    config: Annotated[
        Path | None, typer.Option(help="A JSON file with settings and book_overrides.")
    ] = None,
    book: Annotated[
        list[str] | None, typer.Option(help="Extract only this Zotero item key. Repeatable.")
    ] = None,
) -> None:
    """Extract the selected chapter of each book into the store."""
    try:
        run = run_extraction(
            selection,
            ArtifactStore(data_dir),
            load_config(config) if config else None,
            set(book) if book else None,
        )
    except UrgError as e:
        typer.echo(f"error: {e}", err=True)
        raise typer.Exit(1) from e
    for outcome in run.outcomes:
        if outcome.error is not None:
            typer.echo(f"{outcome.item_key}: failed: {outcome.error}", err=True)
        elif outcome.artifact is not None:
            state = "reused" if outcome.reused else "extracted"
            typer.echo(f"{outcome.item_key}: {state}: {outcome.artifact.directory}")
    typer.echo(f"run report: {run.report.directory}")
    if run.failed:
        raise typer.Exit(1)


@app.command("episode-analysis")
def episode_analysis(
    instructions: Annotated[
        Path, typer.Option(help="The versioned analysis-instructions file.")
    ] = DEFAULT_INSTRUCTIONS,
    model_config: Annotated[
        Path, typer.Option(help="The model configuration file.")
    ] = DEFAULT_MODEL_CONFIG,
    data_dir: Annotated[Path, typer.Option(help="The research-artifact store.")] = Path("data"),
    book: Annotated[
        list[str] | None, typer.Option(help="Analyse only this Zotero item key. Repeatable.")
    ] = None,
    new_attempt: Annotated[
        bool, typer.Option(help="Call the model again, even when a stored response exists.")
    ] = False,
) -> None:
    """Analyse each extracted chapter into proposed interpretations."""
    try:
        model = model_from_config(load_model_config(model_config))
        run = run_analysis(
            ArtifactStore(data_dir), instructions, model, set(book) if book else (), new_attempt
        )
    except UrgError as e:
        typer.echo(f"error: {e}", err=True)
        raise typer.Exit(1) from e
    for outcome in run.outcomes:
        if outcome.error is not None:
            typer.echo(f"{outcome.item_key}: failed: {outcome.error}", err=True)
        if outcome.response is not None:
            state = "reused" if outcome.response_reused else "new"
            typer.echo(f"{outcome.item_key}: {state} response: {outcome.response.directory}")
        if outcome.analysis is not None:
            state = "reused" if outcome.analysis_reused else "analysed"
            typer.echo(f"{outcome.item_key}: {state}: {outcome.analysis.directory}")
    if run.report is not None:
        typer.echo(f"run report: {run.report.directory}")
    if run.failed:
        raise typer.Exit(1)


@app.command("pattern-discovery")
def pattern_discovery(
    settings: Annotated[
        Path, typer.Option(help="The discovery settings file, with the preference weights.")
    ] = discovery.DEFAULT_SETTINGS,
    instructions: Annotated[
        Path, typer.Option(help="The versioned discovery-instructions file.")
    ] = discovery.DEFAULT_INSTRUCTIONS,
    model_config: Annotated[
        Path, typer.Option(help="The model configuration file.")
    ] = discovery.DEFAULT_MODEL_CONFIG,
    data_dir: Annotated[Path, typer.Option(help="The research-artifact store.")] = Path("data"),
    new_attempt: Annotated[
        bool, typer.Option(help="Call the model again, even when a stored response exists.")
    ] = False,
    dpi: Annotated[int, typer.Option(help="The resolution of the report renders.")] = (
        report.DEFAULT_DPI
    ),
) -> None:
    """Discover candidate patterns over the latest analysis run, and build their report."""
    try:
        store = ArtifactStore(data_dir)
        model = model_from_config(load_model_config(model_config))
        run = discovery.run_discovery(store, settings, instructions, model, new_attempt)
        typer.echo(
            f"{'reused' if run.response_reused else 'new'} response: "
            f"{run.record['discovery_run']['response_artifact']}"
        )
        state = "reused" if run.discovery_reused else "discovered"
        typer.echo(f"{state} discovery run: {run.discovery.directory}")
        counts = run.record["counts"]
        typer.echo(
            f"patterns: {counts['patterns']}, rejected: {counts['rejected']}, "
            f"unfitted observations: {counts['unfitted']}"
        )
        built = report.build_report(store, run.discovery, dpi)
    except UrgError as e:
        typer.echo(f"error: {e}", err=True)
        raise typer.Exit(1) from e
    state = "reused" if built.reused else "built"
    typer.echo(f"{state} report: {built.report.path(report.INDEX)}")


@app.command("pattern-report")
def pattern_report(
    discovery_key: Annotated[
        str | None, typer.Option("--discovery", help="The key of a stored discovery run.")
    ] = None,
    data_dir: Annotated[Path, typer.Option(help="The research-artifact store.")] = Path("data"),
    dpi: Annotated[int, typer.Option(help="The resolution of the report renders.")] = (
        report.DEFAULT_DPI
    ),
) -> None:
    """Build the report of one stored discovery run. The default is the newest run."""
    try:
        store = ArtifactStore(data_dir)
        built = report.build_report(store, report.latest_discovery(store, discovery_key), dpi)
    except UrgError as e:
        typer.echo(f"error: {e}", err=True)
        raise typer.Exit(1) from e
    state = "reused" if built.reused else "built"
    typer.echo(f"{state} report: {built.report.path(report.INDEX)}")


@app.command("weighted-report")
def weighted_report_command(
    weights: Annotated[
        Path, typer.Option(help="The preference weight file for books and sections.")
    ] = weighted_report.DEFAULT_WEIGHTS,
    relevance: Annotated[
        Path, typer.Option(help="The relevance assessment file.")
    ] = weighted_report.DEFAULT_RELEVANCE,
    discovery_key: Annotated[
        str | None, typer.Option("--discovery", help="The key of a stored discovery run.")
    ] = None,
    data_dir: Annotated[Path, typer.Option(help="The research-artifact store.")] = Path("data"),
) -> None:
    """Reweight the fixed patterns of one discovery run. The default is the newest run.

    The command writes the equal-book baseline report and the weighted report with its
    comparison. It does not call the model.
    """
    try:
        store = ArtifactStore(data_dir)
        run = weighted_report.build_weighted_reports(
            store, report.latest_discovery(store, discovery_key), weights, relevance
        )
    except UrgError as e:
        typer.echo(f"error: {e}", err=True)
        raise typer.Exit(1) from e
    for name, artifact, reused, page in (
        ("baseline report", run.baseline, run.baseline_reused, weighted_report.BASELINE_PAGE),
        ("weighted report", run.weighted, run.weighted_reused, weighted_report.WEIGHTED_PAGE),
    ):
        typer.echo(f"{'reused' if reused else 'built'} {name}: {artifact.path(page)}")
    typer.echo(
        f"reused {len(run.upstream)} upstream research artifacts: "
        + ", ".join(i.locator for i in run.upstream)
    )


@app.command("corrections")
def corrections_command(
    corrections_dir: Annotated[
        Path, typer.Option(help="The directory of correction files.")
    ] = corrections.DEFAULT_DIRECTORY,
    data_dir: Annotated[Path, typer.Option(help="The research-artifact store.")] = Path("data"),
) -> None:
    """Apply the correction files to the latest episode-analysis run.

    The command exits with status 1 when it refuses a correction. It still applies the
    other corrections and records the run.
    """
    try:
        run = corrections.run_corrections(ArtifactStore(data_dir), corrections_dir)
    except UrgError as e:
        typer.echo(f"error: {e}", err=True)
        raise typer.Exit(1) from e
    for outcome in run.outcomes:
        if outcome["status"] == "applied":
            made = outcome["interpretation"]
            typer.echo(f"applied {outcome['file']}: {made['id']} version {made['version']}")
        else:
            typer.echo(f"refused {outcome['file']}: {outcome['reason']}", err=True)
    for book in run.corrected:
        state = "reused" if book.reused else "wrote"
        typer.echo(f"{state} corrected analysis {book.item_key}: {book.analysis.record.key}")
    state = "reused" if run.reused else "recorded"
    typer.echo(f"{state} corrections run: {run.run.path(corrections.SUMMARY_FILE)}")
    if run.refused:
        raise typer.Exit(1)


@app.command("change-summary")
def change_summary(
    before: Annotated[
        str | None, typer.Option(help="The discovery run before the corrections.")
    ] = None,
    after: Annotated[
        str | None, typer.Option(help="The discovery run after the corrections.")
    ] = None,
    data_dir: Annotated[Path, typer.Option(help="The research-artifact store.")] = Path("data"),
    dpi: Annotated[int, typer.Option(help="The resolution of the report renders.")] = (
        report.DEFAULT_DPI
    ),
) -> None:
    """Summarise what the latest corrections changed in patterns and the report."""
    try:
        run = changes.summarise_changes(ArtifactStore(data_dir), before, after, dpi)
    except UrgError as e:
        typer.echo(f"error: {e}", err=True)
        raise typer.Exit(1) from e
    state = "reused" if run.reused else "wrote"
    typer.echo(f"{state} change summary: {run.summary.path(changes.SUMMARY_FILE)}")


@app.command("render-anchor")
def render_anchor_command(
    item_key: Annotated[str, typer.Argument(help="The Zotero item key of the book.")],
    evidence_id: Annotated[str, typer.Argument(help="An evidence or element ID.")],
    output: Annotated[Path, typer.Option(help="The PNG file to write.")],
    data_dir: Annotated[Path, typer.Option(help="The research-artifact store.")] = Path("data"),
    dpi: Annotated[int, typer.Option(help="The render resolution.")] = 150,
) -> None:
    """Render the page region of one anchor from the latest extraction."""
    try:
        store = ArtifactStore(data_dir)
        artifact = latest_extraction(store, item_key)
        output.write_bytes(render_anchor(artifact, evidence_id, dpi, store=store))
    except (UrgError, OSError) as e:
        typer.echo(f"error: {e}", err=True)
        raise typer.Exit(1) from e
    typer.echo(f"wrote {output}")
