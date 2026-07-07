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
        self.scrape_html(&html)
    }

    pub fn scrape_html(&self, html: &str) -> anyhow::Result<Vec<MatchRecord>> {
        Ok(self.scrape_html_with_diagnostics(html)?.records)
    }

    pub fn scrape_html_with_diagnostics(&self, html: &str) -> anyhow::Result<ScrapeReport> {
        wiki_scraper::scrape_matches_with_diagnostics(html, &self.config)
    }
}
