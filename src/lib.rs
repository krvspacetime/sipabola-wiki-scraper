mod client;
pub mod config;
mod fetcher;
pub mod models;
pub mod parser;
mod wiki_scraper;
pub mod writer;

pub use client::SipabolaWikiScraper;
pub use config::{FootballBoxOverrides, ScraperConfig, ScraperConfigOverrides, VeventOverrides};
pub use models::{MatchRecord, MatchRecordBuildError};
pub use wiki_scraper::{ScrapeDiagnostics, ScrapeReport};
pub use writer::RecordsWriter;
