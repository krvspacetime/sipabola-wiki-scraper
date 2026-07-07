use std::{
    fs::{self, File},
    io::BufWriter,
    path::Path,
};

use sipabola_wiki_scraper::{CliConfig, MatchRecord, SipabolaWikiScraper};

const OUTPUT_PATH: &str = "out/output.json";

fn main() -> anyhow::Result<()> {
    let config = CliConfig::load()?;
    let scraper = SipabolaWikiScraper::with_config(config.scraper_config);
    let records = scraper.scrape_url(&config.url)?;

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
