use anyhow::{Context, bail};

use crate::config::{ScraperConfig, ScraperConfigOverrides};

pub struct CliConfig {
    pub url: String,
    pub scraper_config: ScraperConfig,
}

impl CliConfig {
    pub fn load() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let mut args = std::env::args().skip(1);
        let url = args
            .next()
            .context("usage: wiki_scraper <wikipedia-url> [--config selectors.json]")?;

        let mut overrides = ScraperConfigOverrides::default();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--config" | "-c" => {
                    let path = args
                        .next()
                        .context("--config requires a JSON config file path")?;
                    let file = std::fs::File::open(&path)
                        .with_context(|| format!("failed to open config file `{path}`"))?;
                    overrides = serde_json::from_reader(file)
                        .with_context(|| format!("failed to parse config file `{path}`"))?;
                }
                other => bail!("unknown argument `{other}`"),
            }
        }

        let scraper_config = ScraperConfig::new().with_overrides(overrides);
        scraper_config.validate()?;

        Ok(Self {
            url,
            scraper_config,
        })
    }
}
