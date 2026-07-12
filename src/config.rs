use scraper::Selector;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct ScraperConfig {
    pub(crate) footballbox: FootballBoxSelectors,
    pub(crate) vevent: VeventSelectors,
}

impl ScraperConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_overrides(mut self, overrides: ScraperConfigOverrides) -> Self {
        if let Some(footballbox) = overrides.footballbox {
            self.footballbox.apply_overrides(footballbox);
        }
        if let Some(vevent) = overrides.vevent {
            self.vevent.apply_overrides(vevent);
        }
        self
    }

    pub fn footballbox_time_selector(mut self, selector: impl Into<String>) -> Self {
        self.footballbox.time_selector = selector.into();
        self
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        let selectors = [
            self.footballbox.root_selector.as_str(),
            self.footballbox.table_selector.as_str(),
            self.footballbox.row_selector.as_str(),
            self.footballbox.left_selector.as_str(),
            self.footballbox.date_selector.as_str(),
            self.footballbox.time_selector.as_str(),
            self.footballbox.home_selector.as_str(),
            self.footballbox.score_selector.as_str(),
            self.footballbox.away_selector.as_str(),
            self.footballbox.home_goal_selector.as_str(),
            self.footballbox.away_goal_selector.as_str(),
            self.footballbox.shootout_score_selector.as_str(),
            self.footballbox.location_selector.as_str(),
            self.footballbox.details_root_selector.as_str(),
            self.footballbox.details_line_selector.as_str(),
            self.vevent.root_selector.as_str(),
            self.vevent.table_selector.as_str(),
            self.vevent.row_selector.as_str(),
            self.vevent.cell_selector.as_str(),
        ];

        for selector in selectors {
            Selector::parse(selector)
                .map_err(|err| anyhow::anyhow!("invalid selector `{selector}`: {err:?}"))?;
        }

        Ok(())
    }
}

impl Default for ScraperConfig {
    fn default() -> Self {
        Self {
            footballbox: FootballBoxSelectors::default(),
            vevent: VeventSelectors::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct ScraperConfigOverrides {
    pub footballbox: Option<FootballBoxOverrides>,
    pub vevent: Option<VeventOverrides>,
}

impl ScraperConfigOverrides {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn footballbox(mut self, overrides: FootballBoxOverrides) -> Self {
        self.footballbox = Some(overrides);
        self
    }

    pub fn vevent(mut self, overrides: VeventOverrides) -> Self {
        self.vevent = Some(overrides);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct FootballBoxOverrides {
    pub root_selector: Option<String>,
    pub table_selector: Option<String>,
    pub row_selector: Option<String>,
    pub left_selector: Option<String>,
    pub date_selector: Option<String>,
    pub time_selector: Option<String>,
    pub home_selector: Option<String>,
    pub score_selector: Option<String>,
    pub away_selector: Option<String>,
    pub home_goal_selector: Option<String>,
    pub away_goal_selector: Option<String>,
    pub shootout_score_selector: Option<String>,
    pub location_selector: Option<String>,
    pub details_root_selector: Option<String>,
    pub details_line_selector: Option<String>,
}

impl FootballBoxOverrides {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn root_selector(mut self, selector: impl Into<String>) -> Self {
        self.root_selector = Some(selector.into());
        self
    }

    pub fn table_selector(mut self, selector: impl Into<String>) -> Self {
        self.table_selector = Some(selector.into());
        self
    }

    pub fn row_selector(mut self, selector: impl Into<String>) -> Self {
        self.row_selector = Some(selector.into());
        self
    }

    pub fn left_selector(mut self, selector: impl Into<String>) -> Self {
        self.left_selector = Some(selector.into());
        self
    }

    pub fn date_selector(mut self, selector: impl Into<String>) -> Self {
        self.date_selector = Some(selector.into());
        self
    }

    pub fn time_selector(mut self, selector: impl Into<String>) -> Self {
        self.time_selector = Some(selector.into());
        self
    }

    pub fn home_selector(mut self, selector: impl Into<String>) -> Self {
        self.home_selector = Some(selector.into());
        self
    }

    pub fn score_selector(mut self, selector: impl Into<String>) -> Self {
        self.score_selector = Some(selector.into());
        self
    }

    pub fn away_selector(mut self, selector: impl Into<String>) -> Self {
        self.away_selector = Some(selector.into());
        self
    }

    pub fn home_goal_selector(mut self, selector: impl Into<String>) -> Self {
        self.home_goal_selector = Some(selector.into());
        self
    }

    pub fn away_goal_selector(mut self, selector: impl Into<String>) -> Self {
        self.away_goal_selector = Some(selector.into());
        self
    }

    pub fn shootout_score_selector(mut self, selector: impl Into<String>) -> Self {
        self.shootout_score_selector = Some(selector.into());
        self
    }

    pub fn location_selector(mut self, selector: impl Into<String>) -> Self {
        self.location_selector = Some(selector.into());
        self
    }

    pub fn details_root_selector(mut self, selector: impl Into<String>) -> Self {
        self.details_root_selector = Some(selector.into());
        self
    }

    pub fn details_line_selector(mut self, selector: impl Into<String>) -> Self {
        self.details_line_selector = Some(selector.into());
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct VeventOverrides {
    pub root_selector: Option<String>,
    pub table_selector: Option<String>,
    pub row_selector: Option<String>,
    pub cell_selector: Option<String>,
}

impl VeventOverrides {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn root_selector(mut self, selector: impl Into<String>) -> Self {
        self.root_selector = Some(selector.into());
        self
    }

    pub fn table_selector(mut self, selector: impl Into<String>) -> Self {
        self.table_selector = Some(selector.into());
        self
    }

    pub fn row_selector(mut self, selector: impl Into<String>) -> Self {
        self.row_selector = Some(selector.into());
        self
    }

    pub fn cell_selector(mut self, selector: impl Into<String>) -> Self {
        self.cell_selector = Some(selector.into());
        self
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FootballBoxSelectors {
    pub(crate) root_selector: String,
    pub(crate) table_selector: String,
    pub(crate) row_selector: String,
    pub(crate) left_selector: String,
    pub(crate) date_selector: String,
    pub(crate) time_selector: String,
    pub(crate) home_selector: String,
    pub(crate) score_selector: String,
    pub(crate) away_selector: String,
    pub(crate) home_goal_selector: String,
    pub(crate) away_goal_selector: String,
    pub(crate) shootout_score_selector: String,
    pub(crate) location_selector: String,
    pub(crate) details_root_selector: String,
    pub(crate) details_line_selector: String,
}

impl FootballBoxSelectors {
    fn apply_overrides(&mut self, overrides: FootballBoxOverrides) {
        apply(&mut self.root_selector, overrides.root_selector);
        apply(&mut self.table_selector, overrides.table_selector);
        apply(&mut self.row_selector, overrides.row_selector);
        apply(&mut self.left_selector, overrides.left_selector);
        apply(&mut self.date_selector, overrides.date_selector);
        apply(&mut self.time_selector, overrides.time_selector);
        apply(&mut self.home_selector, overrides.home_selector);
        apply(&mut self.score_selector, overrides.score_selector);
        apply(&mut self.away_selector, overrides.away_selector);
        apply(&mut self.home_goal_selector, overrides.home_goal_selector);
        apply(&mut self.away_goal_selector, overrides.away_goal_selector);
        apply(
            &mut self.shootout_score_selector,
            overrides.shootout_score_selector,
        );
        apply(&mut self.location_selector, overrides.location_selector);
        apply(
            &mut self.details_root_selector,
            overrides.details_root_selector,
        );
        apply(
            &mut self.details_line_selector,
            overrides.details_line_selector,
        );
    }
}

impl Default for FootballBoxSelectors {
    fn default() -> Self {
        Self {
            root_selector: "div.footballbox".to_string(),
            table_selector: "table.fevent".to_string(),
            row_selector: "tr".to_string(),
            left_selector: ".fleft".to_string(),
            date_selector: ".fdate".to_string(),
            time_selector: ".ftime".to_string(),
            home_selector: ".fhome".to_string(),
            score_selector: ".fscore".to_string(),
            away_selector: ".faway".to_string(),
            home_goal_selector: ".fhgoal".to_string(),
            away_goal_selector: ".fagoal".to_string(),
            shootout_score_selector: "th".to_string(),
            location_selector: "div[itemprop='location']".to_string(),
            details_root_selector: ".fright".to_string(),
            details_line_selector: "div".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct VeventSelectors {
    pub(crate) root_selector: String,
    pub(crate) table_selector: String,
    pub(crate) row_selector: String,
    pub(crate) cell_selector: String,
}

impl VeventSelectors {
    fn apply_overrides(&mut self, overrides: VeventOverrides) {
        apply(&mut self.root_selector, overrides.root_selector);
        apply(&mut self.table_selector, overrides.table_selector);
        apply(&mut self.row_selector, overrides.row_selector);
        apply(&mut self.cell_selector, overrides.cell_selector);
    }
}

impl Default for VeventSelectors {
    fn default() -> Self {
        Self {
            root_selector: "div.vevent".to_string(),
            table_selector: "table".to_string(),
            row_selector: "tr".to_string(),
            cell_selector: "td".to_string(),
        }
    }
}

fn apply(target: &mut String, override_value: Option<String>) {
    if let Some(value) = override_value {
        *target = value;
    }
}
