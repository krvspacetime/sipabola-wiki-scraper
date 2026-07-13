use cli::CliArgs;
use sipabola_wiki_scraper::{RecordsWriter, SipabolaWikiScraper};

mod cli;

fn main() -> anyhow::Result<()> {
    let args = CliArgs::args()?;
    let CliArgs {
        url,
        scraper_config,
        output,
    } = args;

    let scraper = if let Some(config) = scraper_config {
        SipabolaWikiScraper::with_config(config)
    } else {
        SipabolaWikiScraper::new()
    };

    let records = scraper.scrape_url(&url)?;

    let target_path = output.as_deref().unwrap_or("out/records.json");

    RecordsWriter::write_json(&records, target_path)?;

    Ok(())
}
