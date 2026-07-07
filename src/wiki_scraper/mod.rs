// src/wiki_scraper/mod.rs
use crate::config::ScraperConfig;
use crate::models::{MatchRecord, PenaltyShootoutTaker};
use crate::parser::{clean_city_country, parse_match_score, parse_stadium_details};
use scraper::{ElementRef, Html, Selector};

mod footballbox;
mod vevent;

mod dom;

use footballbox::FootballBoxScraper;
use vevent::VeventScraper;

pub(crate) struct RawMatchData {
    pub(crate) raw_date: String,
    pub(crate) raw_time: Option<String>,
    pub(crate) raw_home_team: String,
    pub(crate) raw_away_team: String,
    pub(crate) raw_score: String,
    pub(crate) raw_home_scorers: Option<String>,
    pub(crate) raw_away_scorers: Option<String>,
    pub(crate) raw_city_country: Option<String>,
    pub(crate) raw_stadium_details: Option<String>,
    pub(crate) raw_shootout_score: Option<String>,
    pub(crate) raw_home_shootout: Vec<PenaltyShootoutTaker>,
    pub(crate) raw_away_shootout: Vec<PenaltyShootoutTaker>,
}

#[derive(Debug, Clone, Default)]
pub struct ScrapeDiagnostics {
    pub matched_nodes: usize,
    pub unsupported_nodes: usize,
    pub extracted_matches: usize,
    pub incomplete_records: usize,
    pub build_failures: usize,
}

#[derive(Debug, Clone)]
pub struct ScrapeReport {
    pub records: Vec<MatchRecord>,
    pub diagnostics: ScrapeDiagnostics,
}

trait HtmlScraper {
    /// CSS selector for nodes this scraper can parse.
    fn root_selector<'a>(&self, config: &'a ScraperConfig) -> &'a str;

    /// True if the CSS selectors of this scraper match the node
    fn can_scrape(&self, node: &ElementRef, config: &ScraperConfig) -> bool;

    /// Extracts raw strings from the DOM table
    fn extract_raw_match(&self, node: &ElementRef, config: &ScraperConfig) -> Option<RawMatchData>;
}

fn get_scraper_for_node<'a>(
    node: &ElementRef,
    scrapers: &'a [&dyn HtmlScraper],
    config: &ScraperConfig,
) -> Option<&'a dyn HtmlScraper> {
    scrapers
        .iter()
        .copied()
        .find(|scraper| scraper.can_scrape(node, config))
}

fn plausible_year(text: &str) -> Option<String> {
    let re_year = regex::Regex::new(r"\b(18[5-9]\d|19\d{2}|20\d{2}|2100)\b").unwrap();
    re_year.captures(text).map(|caps| caps[0].to_string())
}

fn is_heading_node(node: &ElementRef) -> bool {
    matches!(node.value().name(), "h2" | "h3" | "h4")
}

fn combined_node_selector(
    config: &ScraperConfig,
    scrapers: &[&dyn HtmlScraper],
) -> anyhow::Result<Selector> {
    let mut selectors = vec!["h2".to_string(), "h3".to_string(), "h4".to_string()];

    selectors.extend(
        scrapers
            .iter()
            .map(|scraper| scraper.root_selector(config).to_string()),
    );

    selectors.sort();
    selectors.dedup();

    Selector::parse(&selectors.join(", "))
        .map_err(|err| anyhow::anyhow!("invalid CSS selector: {err:?}"))
}

fn build_match_record(
    raw: RawMatchData,
    year: &str,
    competition: &str,
) -> anyhow::Result<MatchRecord> {
    let mut year_val = year.to_string();
    if year_val.is_empty() {
        if let Some(year) = plausible_year(&raw.raw_date) {
            year_val = year;
        }
    }

    let clean_date = raw
        .raw_date
        .split('(')
        .next()
        .unwrap_or("")
        .trim()
        .to_string();

    let full_date = if clean_date.contains(&year_val) {
        clean_date.clone()
    } else {
        format!("{}, {}", clean_date, year_val)
    };

    let clean_time_val = raw.raw_time.map(|t| crate::parser::text::clean_time(&t));

    let mut score_record = parse_match_score(
        &raw.raw_score,
        &raw.raw_home_team,
        &raw.raw_away_team,
        raw.raw_home_scorers.as_deref(),
        raw.raw_away_scorers.as_deref(),
        raw.raw_shootout_score.as_deref(),
    );

    if !raw.raw_home_shootout.is_empty() {
        score_record
            .home_mut()
            .set_shootout_takers(raw.raw_home_shootout);
    }
    if !raw.raw_away_shootout.is_empty() {
        score_record
            .away_mut()
            .set_shootout_takers(raw.raw_away_shootout);
    }

    let mut builder = MatchRecord::builder(&year_val)
        .raw_date(clean_date)
        .competition(competition)
        .full_date(full_date)
        .home_team(raw.raw_home_team)
        .away_team(raw.raw_away_team)
        .score(score_record)
        .time(clean_time_val);

    if let Some(city_raw) = raw.raw_city_country {
        let clean_city = clean_city_country(&city_raw);
        builder = builder.city_country(clean_city.clone());

        let parts: Vec<&str> = clean_city.split(',').collect();
        if !parts.is_empty() {
            builder = builder.stadium(Some(parts[0].trim().to_string()));
        }
    }

    if let Some(stadium_raw) = raw.raw_stadium_details {
        let (stadium, attendance, referee) = parse_stadium_details(&stadium_raw);
        if stadium.is_some() {
            builder = builder.stadium(stadium);
        }
        builder = builder.attendance(attendance).referee(referee);
    }

    Ok(builder.build()?)
}

pub fn scrape_matches_with_diagnostics(
    html: &str,
    config: &ScraperConfig,
) -> anyhow::Result<ScrapeReport> {
    config.validate()?;

    let document = Html::parse_document(html);
    let footballbox_scraper = FootballBoxScraper;
    let vevent_scraper = VeventScraper;
    let scrapers: [&dyn HtmlScraper; 2] = [&footballbox_scraper, &vevent_scraper];

    let node_selector = combined_node_selector(config, &scrapers)?;

    let mut records = Vec::new();
    let mut diagnostics = ScrapeDiagnostics::default();
    let mut current_year = String::new();
    let mut current_competition = String::from("International Match");

    for node in document.select(&node_selector) {
        if is_heading_node(&node) {
            let heading_text = crate::parser::clean_text(&node.text().collect::<String>());
            if let Some(year) = plausible_year(&heading_text) {
                current_year = year;
            }
            if heading_text != current_year && heading_text.len() > 4 {
                current_competition = heading_text;
            }
            continue;
        }

        let Some(scraper) = get_scraper_for_node(&node, &scrapers, config) else {
            diagnostics.unsupported_nodes += 1;
            continue;
        };
        diagnostics.matched_nodes += 1;

        if let Some(raw_data) = scraper.extract_raw_match(&node, config) {
            diagnostics.extracted_matches += 1;
            match build_match_record(raw_data, &current_year, &current_competition) {
                Ok(record) => records.push(record),
                Err(_) => {
                    diagnostics.incomplete_records += 1;
                    diagnostics.build_failures += 1;
                }
            }
        }
    }

    Ok(ScrapeReport {
        records,
        diagnostics,
    })
}
