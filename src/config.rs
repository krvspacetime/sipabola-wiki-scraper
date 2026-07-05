use anyhow::Context;

use crate::wiki_scraper::ScraperConfig;

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

        let scraper_defaults = ScraperConfig::default();
        let year_header_class =
            std::env::var("YEAR_HEADER_CLASS").unwrap_or(scraper_defaults.year_header_class);
        let event_header_class =
            std::env::var("EVENT_HEADER_CLASS").unwrap_or(scraper_defaults.event_header_class);

        Ok(Self {
            url,
            year_header_class,
            event_header_class,
        })
    }
}
