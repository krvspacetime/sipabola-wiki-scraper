# Sipabola Wiki Scraper

Scrape football match records from Wikipedia pages that use common football match templates.

## Library

Use the client API. It returns `Vec<MatchRecord>` and does not write JSON or touch the filesystem.

```rust
use sipabola_wiki_scraper::SipabolaWikiScraper;

let scraper = SipabolaWikiScraper::new();
let records = scraper.scrape_url("https://en.wikipedia.org/wiki/...")?;
```

If you already have the HTML:

```rust
use sipabola_wiki_scraper::SipabolaWikiScraper;

let scraper = SipabolaWikiScraper::new();
let records = scraper.scrape_html(&html)?;
```

## Selector Overrides

Selector overrides are for small Wikipedia class/selector changes inside a supported DOM structure. They are not meant to support a completely new match layout.

```rust
use sipabola_wiki_scraper::{
    FootballBoxOverrides, ScraperConfig, ScraperConfigOverrides, SipabolaWikiScraper,
};

let config = ScraperConfig::new()
    .with_overrides(ScraperConfigOverrides::new().footballbox(
        FootballBoxOverrides::new().time_selector(".ftimeanddate"),
    ));

let scraper = SipabolaWikiScraper::with_config(config);
let records = scraper.scrape_url("https://en.wikipedia.org/wiki/...")?;
```

## CLI

Default run:

```powershell
cargo run -- "https://en.wikipedia.org/wiki/..."
```

Run with a partial JSON config:

```powershell
cargo run -- "https://en.wikipedia.org/wiki/..." --config config.example.json
```

Example config:

```json
{
  "footballbox": {
    "time_selector": ".ftimeanddate"
  }
}
```

The CLI writes JSON to `out/output.json`.
