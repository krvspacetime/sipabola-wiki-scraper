use anyhow::Context;
use argh::FromArgs;
use std::path::PathBuf;

use crate::config::{ScraperConfig, ScraperConfigOverrides};

#[derive(FromArgs)]
/// Sipabola Wiki Scraper CLI
struct Cli {
    #[argh(positional)]
    pub url: String,

    /// optional path to a config file
    #[argh(option, short = 'c')]
    pub config: Option<PathBuf>,

    /// optional path for the output file
    #[argh(option, short = 'o')]
    pub output: Option<String>, // changed to String
}

pub struct CliArgs {
    pub url: String,
    pub scraper_config: Option<ScraperConfig>,
    pub output: Option<String>,
}

impl CliArgs {
    pub fn args() -> anyhow::Result<Self> {
        let raw_args: Cli = argh::from_env();

        let scraper_config = if let Some(ref path) = raw_args.config {
            let file = std::fs::File::open(path)
                .with_context(|| format!("failed to open config file `{}`", path.display()))?;
            let overrides: ScraperConfigOverrides = serde_json::from_reader(file)
                .with_context(|| format!("failed to parse config file `{}`", path.display()))?;

            let config = ScraperConfig::new().with_overrides(overrides);
            config.validate()?;
            Some(config)
        } else {
            None
        };

        Ok(Self {
            url: raw_args.url,
            scraper_config,
            output: raw_args.output,
        })
    }
}
