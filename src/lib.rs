pub mod config;
pub mod fetcher;
pub mod models;
pub mod parser;
pub mod wiki_scraper;

pub use models::{MatchRecord, MatchRecordBuildError};
pub use wiki_scraper::{
    ScrapeDiagnostics, ScrapeReport, ScraperConfig, scrape_matches, scrape_matches_with_diagnostics,
};
