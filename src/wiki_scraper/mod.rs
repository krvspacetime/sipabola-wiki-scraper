// src/wiki_scraper/mod.rs
use crate::models::{MatchRecord, PenaltyShootoutTaker};
use crate::parser::{clean_city_country, parse_match_score, parse_stadium_details};
use scraper::{ElementRef, Html, Selector};

pub mod footballbox;
pub mod vevent;

pub use footballbox::FootballBoxScraper;
pub use vevent::VeventScraper;

pub struct ScraperConfig {
    pub year_header_class: String,
    pub event_header_class: String,
}

pub struct RawMatchData {
    pub raw_date: String,
    pub raw_time: Option<String>,
    pub raw_home_team: String,
    pub raw_away_team: String,
    pub raw_score: String,
    pub raw_home_scorers: Option<String>,
    pub raw_away_scorers: Option<String>,
    pub raw_city_country: Option<String>,
    pub raw_stadium_details: Option<String>,
    pub raw_home_shootout: Vec<PenaltyShootoutTaker>,
    pub raw_away_shootout: Vec<PenaltyShootoutTaker>,
}

pub trait HtmlScraper {
    /// True if the CSS selectors of this scraper match the node
    fn can_scrape(&self, node: &ElementRef) -> bool;

    /// Extracts raw strings from the DOM table
    fn extract_raw_match(&self, node: &ElementRef) -> Option<RawMatchData>;
}

/// Centralized factory to choose the parser dynamically
fn get_scraper_for_node(node: &ElementRef) -> Box<dyn HtmlScraper> {
    let fb_scraper = FootballBoxScraper;
    if fb_scraper.can_scrape(node) {
        Box::new(fb_scraper)
    } else {
        Box::new(VeventScraper)
    }
}

/// Centralized build sequence
fn build_match_record(raw: RawMatchData, year: &str, competition: &str) -> MatchRecord {
    // 1. Resolve year dynamically if it was empty from the headings (common on tournament pages)
    let mut year_val = year.to_string();
    if year_val.is_empty() {
        let re_year = regex::Regex::new(r"\b(19\d{2}|20\d{2})\b").unwrap();
        if let Some(caps) = re_year.captures(&raw.raw_date) {
            year_val = caps[0].to_string();
        }
    }

    // 2. Clean raw date (strip hidden parenthetical schema dates)
    let clean_date = raw
        .raw_date
        .split('(')
        .next()
        .unwrap_or("")
        .trim()
        .to_string();

    // 3. Format full_date without duplicating year if it already exists
    let full_date = if clean_date.contains(&year_val) {
        clean_date.clone()
    } else {
        format!("{}, {}", clean_date, year_val)
    };

    // 4. Clean kickoff time
    let clean_time_val = raw.raw_time.map(|t| crate::parser::text::clean_time(&t));

    // 5. Centralized parsing of score and scorers lists
    let mut score_record = parse_match_score(
        &raw.raw_score,
        &raw.raw_home_team,
        &raw.raw_away_team,
        raw.raw_home_scorers.as_deref(),
        raw.raw_away_scorers.as_deref(),
    );

    // 6. Attach shootout takers if they exist
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

    // 7. Clean City/Country and extract stadium fallback
    if let Some(city_raw) = raw.raw_city_country {
        let clean_city = clean_city_country(&city_raw);
        builder = builder.city_country(clean_city.clone());

        let parts: Vec<&str> = clean_city.split(',').collect();
        if !parts.is_empty() {
            builder = builder.stadium(Some(parts[0].trim().to_string()));
        }
    }

    // 8. Parse Stadium, Attendance, and Referee details
    if let Some(stadium_raw) = raw.raw_stadium_details {
        let (stadium, attendance, referee) = parse_stadium_details(&stadium_raw);
        if stadium.is_some() {
            builder = builder.stadium(stadium);
        }
        builder = builder.attendance(attendance).referee(referee);
    }

    builder.build()
}

pub fn scrape_matches(html: &str, config: &ScraperConfig) -> anyhow::Result<Vec<MatchRecord>> {
    let document = Html::parse_document(html);

    // Fixed: Scopes year headers, event headers (e.g. vevent), and footballbox classes [E0425]
    let node_selector = Selector::parse(&format!(
        "h2, h3, h4, div.{}, div.{}, div.footballbox",
        config.year_header_class, config.event_header_class
    ))
    .map_err(|err| anyhow::anyhow!("invalid CSS selector: {err:?}"))?;

    let mut records = Vec::new();
    let mut current_year = String::new();
    let mut current_competition = String::from("International Match");

    let re_year = regex::Regex::new(r"\b(19\d{2}|20\d{2})\b").unwrap();

    for node in document.select(&node_selector) {
        let tag_name = node.value().name();

        if tag_name == "h2"
            || tag_name == "h3"
            || tag_name == "h4"
            || node
                .value()
                .classes()
                .any(|c| c == config.year_header_class)
        {
            let heading_text = crate::parser::clean_text(&node.text().collect::<String>());
            if let Some(caps) = re_year.captures(&heading_text) {
                current_year = caps[0].to_string();
            }
            if heading_text != current_year && heading_text.len() > 4 {
                current_competition = heading_text;
            }
            continue;
        }

        let scraper = get_scraper_for_node(&node);
        if let Some(raw_data) = scraper.extract_raw_match(&node) {
            let record = build_match_record(raw_data, &current_year, &current_competition);
            if record.is_complete() {
                records.push(record);
            }
        }
    }

    Ok(records)
}
