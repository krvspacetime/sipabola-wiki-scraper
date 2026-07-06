# Usage

## Library

Use `scrape_html` when the caller already has Wikipedia HTML and wants records only:

```rust
use sipabola_scrape_historical_data::scrape_html;

let records = scrape_html(&html)?;
```

Use `scrape_url` when the caller wants the library to fetch the page:

```rust
use sipabola_scrape_historical_data::scrape_url;

let records = scrape_url("https://en.wikipedia.org/wiki/...")?;
```

Use `ScraperConfig` when Wikipedia changes a selector but keeps the same broad DOM structure:

```rust
use sipabola_scrape_historical_data::{ScraperConfig, scrape_html_with_config};

let mut config = ScraperConfig::default();
config.footballbox.time_selector = ".ftimeanddate".to_string();

let records = scrape_html_with_config(&html, &config)?;
```

## CLI

Default run:

```powershell
cargo run -- "https://en.wikipedia.org/wiki/..."
```

Run with selector overrides:

```powershell
cargo run -- "https://en.wikipedia.org/wiki/..." --config config.example.json
```

The config file is partial JSON. Unspecified selector fields use defaults.
