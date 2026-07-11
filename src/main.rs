use sipabola_wiki_scraper::{CliArgs, RecordsWriter, SipabolaWikiScraper};

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let args = CliArgs::args()?;

    let scraper = if let Some(config) = args.scraper_config {
        SipabolaWikiScraper::with_config(config)
    } else {
        SipabolaWikiScraper::new()
    };

    let records = scraper.scrape_url(&args.url)?;
    RecordsWriter::write_json(&records, "out/records.json")?;

    Ok(())
}
