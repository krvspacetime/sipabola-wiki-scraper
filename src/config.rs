use anyhow::Context;

// ============ config ============
const DEFAULT_YEAR_HEADER_CLASS: &str = "mw-heading3";
const DEFAULT_EVENT_HEADER_CLASS: &str = "vevent";

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
