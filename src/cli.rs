use crate::config::{ScraperConfig, ScraperConfigOverrides};
use anyhow::{Context, bail};

pub struct CliArgs {
    pub url: String,
    pub scraper_config: Option<ScraperConfig>,
}

impl CliArgs {
    pub fn args() -> anyhow::Result<Self> {
        let mut args = std::env::args().skip(1);
        let url = args
            .next()
            .context("usage: wiki_scraper <wikipedia-url> [--config selectors.json]")?;

        let mut scraper_config = None;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--config" | "-c" => {
                    let path = args
                        .next()
                        .context("--config requires a JSON config file path")?;
                    let file = std::fs::File::open(&path)
                        .with_context(|| format!("failed to open config file `{path}`"))?;
                    let overrides: ScraperConfigOverrides = serde_json::from_reader(file)
                        .with_context(|| format!("failed to parse config file `{path}`"))?;

                    let config = ScraperConfig::new().with_overrides(overrides);
                    config.validate()?;
                    scraper_config = Some(config);
                }
                other => bail!("unknown argument `{other}`"),
            }
        }

        Ok(Self {
            url,
            scraper_config,
        })
    }
}
