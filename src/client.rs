use std::path::Path;

use crate::{
    config::ScraperConfig,
    fetcher::fetch_html,
    models::MatchRecord,
    wiki_scraper::{self, ScrapeReport},
};

#[derive(Debug, Clone, Default)]
pub struct SipabolaWikiScraper {
    config: ScraperConfig,
}

impl SipabolaWikiScraper {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(config: ScraperConfig) -> Self {
        Self { config }
    }

    pub fn scrape_url(&self, url: &str) -> anyhow::Result<Vec<MatchRecord>> {
        let html = fetch_html(url)?;
        self.scrape_raw_html(&html)
    }

    pub fn scrape_raw_html(&self, html: &str) -> anyhow::Result<Vec<MatchRecord>> {
        Ok(self.scrape_html_with_diagnostics(html)?.records)
    }

    pub fn scrape_html_with_diagnostics(&self, html: &str) -> anyhow::Result<ScrapeReport> {
        wiki_scraper::scrape_matches_with_diagnostics(html, &self.config)
    }

    pub fn scrape_html_file<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<Vec<MatchRecord>> {
        let path = path.as_ref();

        let html = std::fs::read_to_string(path)?;

        self.scrape_raw_html(&html)
    }
}
