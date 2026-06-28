use anyhow::Context;
use serde::Serialize;
use std::{fs::File, io::BufWriter};

#[derive(Debug, Clone, Serialize, Default)]
pub struct MatchRecord {
    pub raw_date: String,
    pub year: String,
    pub full_date: String,
    pub competition: String,
    pub home_team: String,
    pub away_team: String,
    pub score: String,
    pub home_scorers: Option<String>,
    pub away_scorers: Option<String>,
    pub status: String,
    pub city_country: String,
    pub stadium: Option<String>,
    pub attendance: Option<String>,
}

impl MatchRecord {
    /// Starts building a record for the given year.
    pub fn builder(year: impl Into<String>) -> MatchRecordBuilder {
        MatchRecordBuilder::new(year)
    }

    /// A record is only useful once the core fixture details (the two
    /// teams and the score) have been extracted.
    pub fn is_complete(&self) -> bool {
        !self.home_team.is_empty() && !self.away_team.is_empty() && !self.score.is_empty()
    }
}

#[derive(Debug, Default)]
pub struct MatchRecordBuilder {
    record: MatchRecord,
}

impl MatchRecordBuilder {
    pub fn new(year: impl Into<String>) -> Self {
        Self {
            record: MatchRecord {
                year: year.into(),
                ..Default::default()
            },
        }
    }

    pub fn raw_date(mut self, value: impl Into<String>) -> Self {
        self.record.raw_date = value.into();
        self
    }

    pub fn full_date(mut self, value: impl Into<String>) -> Self {
        self.record.full_date = value.into();
        self
    }

    pub fn competition(mut self, value: impl Into<String>) -> Self {
        self.record.competition = value.into();
        self
    }

    pub fn home_team(mut self, value: impl Into<String>) -> Self {
        self.record.home_team = value.into();
        self
    }

    pub fn away_team(mut self, value: impl Into<String>) -> Self {
        self.record.away_team = value.into();
        self
    }

    pub fn score(mut self, value: impl Into<String>) -> Self {
        self.record.score = value.into();
        self
    }

    pub fn home_scorers(mut self, value: impl Into<String>) -> Self {
        self.record.home_scorers = Some(value.into());
        self
    }

    pub fn away_scorers(mut self, value: impl Into<String>) -> Self {
        self.record.away_scorers = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.record.status = value.into();
        self
    }

    pub fn city_country(mut self, value: impl Into<String>) -> Self {
        self.record.city_country = value.into();
        self
    }

    pub fn stadium(mut self, value: impl Into<String>) -> Self {
        self.record.stadium = Some(value.into());
        self
    }

    pub fn attendance(mut self, value: impl Into<String>) -> Self {
        self.record.attendance = Some(value.into());
        self
    }

    pub fn build(self) -> MatchRecord {
        self.record
    }
}

const OUTPUT_PATH: &str = "output.json";

fn main() -> anyhow::Result<()> {
    let config = Config::load()?;
    let html = fetch_html(&config.url)?;

    let parser_config = ParserConfig {
        year_header_class: config.year_header_class,
        event_header_class: config.event_header_class,
    };

    let records = parse_matches(&html, &parser_config)?;
    write_json(&records, OUTPUT_PATH)?;

    Ok(())
}

const DEFAULT_YEAR_HEADER_CLASS: &str = "mw-heading3";
const DEFAULT_EVENT_HEADER_CLASS: &str = "vevent";

/// Runtime configuration, sourced from the CLI arguments and optional
/// environment overrides (loaded from a `.env` file if present).
pub struct Config {
    pub url: String,
    pub year_header_class: String,
    pub event_header_class: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let url = std::env::args()
            .nth(1)
            .context("usage: wiki_scraper <wikipedia-url>")?;

        let year_header_class = std::env::var("YEAR_HEADER_CLASS")
            .unwrap_or_else(|_| DEFAULT_YEAR_HEADER_CLASS.to_string());
        let event_header_class = std::env::var("EVENT_HEADER_CLASS")
            .unwrap_or_else(|_| DEFAULT_EVENT_HEADER_CLASS.to_string());

        Ok(Self {
            url,
            year_header_class,
            event_header_class,
        })
    }
}

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) RustScraper";

pub fn fetch_html(url: &str) -> anyhow::Result<String> {
    let client = reqwest::blocking::Client::new();
    let html = client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()?
        .text()?;
    Ok(html)
}

use scraper::{ElementRef, Html, Selector};

/// Class names used to locate year headings and individual fixture blocks.
/// Wikipedia's markup is brittle, so these are kept narrow and configurable
/// rather than relying on deeper, more specific selectors.
pub struct ParserConfig {
    pub year_header_class: String,
    pub event_header_class: String,
}

/// Walks the document, tracking the most recent year heading, and collects
/// a [`MatchRecord`] for every fixture block that yields usable data.
pub fn parse_matches(html: &str, config: &ParserConfig) -> anyhow::Result<Vec<MatchRecord>> {
    let document = Html::parse_document(html);

    let node_selector = Selector::parse(&format!(
        "div.{}, div.{}",
        config.year_header_class, config.event_header_class
    ))
    .map_err(|err| anyhow::anyhow!("invalid CSS selector: {err:?}"))?;

    let mut records = Vec::new();
    let mut current_year = String::new();

    for node in document.select(&node_selector) {
        if is_year_header(&node, &config.year_header_class) {
            if let Some(year) = node.text().next() {
                current_year = year.trim().to_string();
            }
            continue;
        }

        if let Some(record) = parse_event_block(node, &current_year) {
            if record.is_complete() {
                records.push(record);
            }
        }
    }

    Ok(records)
}

fn is_year_header(node: &ElementRef, year_header_class: &str) -> bool {
    node.value()
        .classes()
        .any(|class| class == year_header_class)
}

/// Parses a single fixture block. Wikipedia lays each fixture out as a
/// two-row table: the first row holds the date, teams and score (plus an
/// optional venue cell), and the second row holds scorers and venue detail.
fn parse_event_block(node: ElementRef, year: &str) -> Option<MatchRecord> {
    let fragment = Html::parse_fragment(&node.html());
    let table_selector = Selector::parse("table").ok()?;
    let row_selector = Selector::parse("tr").ok()?;
    let cell_selector = Selector::parse("td").ok()?;

    let table = fragment.select(&table_selector).next()?;
    let rows: Vec<_> = table.select(&row_selector).collect();

    let first_row_cells: Vec<_> = rows.first()?.select(&cell_selector).collect();
    if first_row_cells.len() < 4 {
        return None;
    }

    let mut builder = MatchRecord::builder(year)
        .raw_date(cell_text(&first_row_cells[0]))
        .home_team(cell_text(&first_row_cells[1]))
        .score(cell_text(&first_row_cells[2]))
        .away_team(cell_text(&first_row_cells[3]));

    if let Some(venue) = first_row_cells
        .get(4)
        .map(cell_text)
        .filter(|s| !s.is_empty())
    {
        builder = builder.stadium(venue);
    }

    if let Some(second_row) = rows.get(1) {
        let second_row_cells: Vec<_> = second_row.select(&cell_selector).collect();
        if second_row_cells.len() >= 4 {
            if let Some(home_scorers) = non_empty_cell(&second_row_cells, 1) {
                builder = builder.home_scorers(home_scorers);
            }
            if let Some(away_scorers) = non_empty_cell(&second_row_cells, 3) {
                builder = builder.away_scorers(away_scorers);
            }
            if let Some(stadium_detail) = non_empty_cell(&second_row_cells, 4) {
                builder = builder.stadium(stadium_detail);
            }
        }
    }

    Some(builder.build())
}

fn cell_text(cell: &ElementRef) -> String {
    cell.text().collect::<Vec<_>>().join(" ").trim().to_string()
}

fn non_empty_cell(cells: &[ElementRef], index: usize) -> Option<String> {
    cells.get(index).map(cell_text).filter(|s| !s.is_empty())
}

pub fn write_json(records: &[MatchRecord], path: &str) -> anyhow::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, records)?;
    Ok(())
}
