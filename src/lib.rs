pub mod config;
pub mod fetcher;
pub mod models;
pub mod parser;
pub mod wiki_scraper;

pub use models::{MatchRecord, MatchRecordBuildError};
pub use wiki_scraper::{
    ScrapeDiagnostics, ScrapeReport, ScraperConfig, scrape_matches, scrape_matches_with_diagnostics,
};

pub fn scrape_html(html: &str) -> anyhow::Result<Vec<MatchRecord>> {
    scrape_matches(html, &ScraperConfig::default())
}

pub fn scrape_html_with_config(
    html: &str,
    config: &ScraperConfig,
) -> anyhow::Result<Vec<MatchRecord>> {
    scrape_matches(html, config)
}

pub fn scrape_url(url: &str) -> anyhow::Result<Vec<MatchRecord>> {
    let html = fetcher::fetch_html(url)?;
    scrape_html(&html)
}

pub fn scrape_url_with_config(
    url: &str,
    config: &ScraperConfig,
) -> anyhow::Result<Vec<MatchRecord>> {
    let html = fetcher::fetch_html(url)?;
    scrape_html_with_config(&html, config)
}
