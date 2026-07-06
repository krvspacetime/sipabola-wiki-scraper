use anyhow::Context;

use crate::wiki_scraper::ScraperConfig;

pub struct Config {
    pub url: String,
    pub scraper_config: ScraperConfig,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let mut args = std::env::args().skip(1);
        let url = args
            .next()
            .context("usage: wiki_scraper <wikipedia-url> [--config selectors.json]")?;

        let mut scraper_config = ScraperConfig::default();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--config" | "-c" => {
                    let path = args
                        .next()
                        .context("--config requires a JSON config file path")?;
                    let file = std::fs::File::open(&path)
                        .with_context(|| format!("failed to open config file `{path}`"))?;
                    scraper_config = serde_json::from_reader(file)
                        .with_context(|| format!("failed to parse config file `{path}`"))?;
                }
                other => anyhow::bail!("unknown argument `{other}`"),
            }
        }

        scraper_config.validate()?;

        Ok(Self {
            url,
            scraper_config,
        })
    }
}
