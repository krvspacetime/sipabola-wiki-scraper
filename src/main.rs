use sipabola_wiki_scraper::{CliConfig, RecordWriter, SipabolaWikiScraper};

fn main() -> anyhow::Result<()> {
    let config = CliConfig::load()?;
    let scraper = SipabolaWikiScraper::with_config(config.scraper_config);
    let records = scraper.scrape_url(&config.url)?;

    RecordWriter::write_json(&records, "out/records.json")?;

    Ok(())
}
