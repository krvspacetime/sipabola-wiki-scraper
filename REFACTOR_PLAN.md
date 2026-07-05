# Refactor Plan

The refactor should move in small reviewable steps. Each step should compile and keep the scraper behavior covered by the fixtures in `tests/wiki_scraper.rs`.

## Step 1: Establish regression coverage

- Keep the `vevent` and `footballbox` integration fixtures under `tests/`.
- Add more fixture cases only when they protect a known behavior or bug.
- Avoid changing scraper behavior in this step except for fixes required by the tests.

## Step 2: Split library and CLI responsibilities

- Keep reusable scraping code exposed from `src/lib.rs`.
- Keep argument parsing, fetching, and file output in the CLI layer.
- Move `write_json` out of `main.rs` only if more output formats or CLI tests are added.

## Step 3: Introduce shared DOM helpers

- Add `src/wiki_scraper/dom.rs`.
- Move duplicated helpers such as text extraction and penalty shootout taker parsing there.
- Keep scraper-specific selectors inside each concrete scraper.

## Step 4: Replace fallback scraper selection

- Change scraper selection from `footballbox else vevent` to an explicit scraper registry.
- Make unmatched nodes skip cleanly or produce debug diagnostics.
- Add a test proving an unrelated node is ignored instead of parsed by the wrong scraper.

## Step 5: Move scraper root selectors into scraper implementations

- Extend `HtmlScraper` with a root selector or matcher metadata.
- Build the page-level selector from registered scrapers instead of hardcoding `div.vevent` and `div.footballbox` in the orchestrator.
- Keep heading selectors separate from match-card selectors.

## Step 6: Make heading detection semantic

- Stop requiring users to configure the year heading class for normal pages.
- Track years by parsing heading text and accepting plausible football years.
- Track competition names separately from years.
- Keep class mapping config as an advanced fallback.

## Step 7: Strengthen raw match data

- Add fields that are currently implicit or missing, such as shootout score.
- Keep `RawMatchData` as scraper output only; do not expose it as the public API unless needed.
- Prefer optional structured fields over packing multiple concepts into raw strings.

## Step 8: Harden parsers with unit tests

- Add focused parser tests for scores, extra time, penalties, scorer minutes, own goals, penalties, attendance, and referee parsing.
- Keep parser tests independent from HTML fixtures.
- Fix parser bugs in isolated commits before changing scraper structure.

## Step 9: Make the builder validate records

- Change `MatchRecordBuilder::build` to return `Result<MatchRecord, MatchRecordBuildError>`.
- Move completeness checks into builder validation.
- Keep `MatchRecord::is_complete` temporarily if useful, then remove it once callers rely on the builder result.

## Step 10: Define the public library API

- Decide which modules are public API and which remain internal.
- Re-export stable entry points from `lib.rs`.
- Hide implementation details such as concrete scraper modules unless users need extension points.

## Step 11: Add user config as an override layer

- Add optional config for selector/class mapping after semantic detection works.
- Treat config as an override for changed Wikipedia markup, not as the default path.
- Validate config early and return clear errors for invalid selectors.

## Step 12: Add diagnostics

- Return or log scraper statistics: matched nodes, skipped nodes, incomplete records, and parser failures.
- Keep the library quiet by default.
- Let the CLI expose diagnostics through a verbose flag later.
