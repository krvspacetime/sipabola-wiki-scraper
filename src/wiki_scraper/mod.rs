// src/wiki_scraper/mod.rs
use crate::models::MatchRecord;
use scraper::Html;

pub mod footballbox;
pub mod vevent;

pub use footballbox::FootballBoxScraper;
pub use vevent::VeventScraper;

pub struct ScraperConfig {
    pub year_header_class: String,
    pub event_header_class: String,
}

pub trait HtmlScraper {
    /// Inspects the HTML DOM and returns true if this scraper can parse the page structure
    fn can_scrape(&self, document: &Html, config: &ScraperConfig) -> bool;

    /// Parses the raw HTML into a list of MatchRecords
    fn scrape(&self, html: &str, config: &ScraperConfig) -> anyhow::Result<Vec<MatchRecord>>;
}

/// Dynamically inspects the document structure and returns the correct Scraper engine
pub fn get_scraper(html: &str, config: &ScraperConfig) -> Box<dyn HtmlScraper> {
    let document = Html::parse_document(html);
    let vevent_scraper = VeventScraper;

    if vevent_scraper.can_scrape(&document, config) {
        Box::new(vevent_scraper)
    } else {
        Box::new(FootballBoxScraper)
    }
}
