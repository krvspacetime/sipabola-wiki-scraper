use std::{
    fs::{self, File},
    io::BufWriter,
    path::Path,
};

use sipabola_scrape_historical_data::{
    config::Config,
    fetcher::fetch_html,
    models::MatchRecord,
    wiki_scraper::{self, ScraperConfig},
};

const OUTPUT_PATH: &str = "out/output.json";

fn main() -> anyhow::Result<()> {
    let config = Config::load()?;
    let html = fetch_html(&config.url)?;

    let scraper_config = ScraperConfig {
        year_header_class: config.year_header_class,
        event_header_class: config.event_header_class,
    };

    let records = wiki_scraper::scrape_matches(&html, &scraper_config)?;

    write_json(&records, OUTPUT_PATH)?;

    Ok(())
}

pub fn write_json(records: &[MatchRecord], path: &str) -> anyhow::Result<()> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, records)?;

    Ok(())
}
