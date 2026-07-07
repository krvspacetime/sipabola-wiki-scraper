// src/wiki_scraper/footballbox.rs
use super::{
    HtmlScraper, RawMatchData,
    dom::{element_text, has_configured_class, parse_shootout_takers},
};
use crate::config::ScraperConfig;
use crate::parser::clean_text;
use regex::Regex;
use scraper::{ElementRef, Selector};

pub struct FootballBoxScraper;

impl FootballBoxScraper {
    /// Safely isolates date, removes parentheticals, and extracts the year
    fn parse_date_and_year(&self, cell: &ElementRef) -> (String, String) {
        let text = clean_text(&cell.text().collect::<String>());
        let date_clean = text.split('(').next().unwrap_or("").trim().to_string();

        let re_year = Regex::new(r"\b(19\d{2}|20\d{2})\b").unwrap();
        let year = if let Some(caps) = re_year.captures(&date_clean) {
            caps[0].to_string()
        } else {
            String::new()
        };
        (date_clean, year)
    }
}

impl HtmlScraper for FootballBoxScraper {
    fn root_selector<'a>(&self, config: &'a ScraperConfig) -> &'a str {
        &config.footballbox.root_selector
    }

    fn can_scrape(&self, node: &ElementRef, config: &ScraperConfig) -> bool {
        has_configured_class(node, &config.footballbox.root_selector)
    }

    fn extract_raw_match(&self, node: &ElementRef, config: &ScraperConfig) -> Option<RawMatchData> {
        let selectors = &config.footballbox;
        let table_selector = Selector::parse(&selectors.table_selector).ok()?;
        let row_selector = Selector::parse(&selectors.row_selector).ok()?;

        let fleft_selector = Selector::parse(&selectors.left_selector).ok()?;
        let fdate_selector = Selector::parse(&selectors.date_selector).ok()?;
        let ftime_selector = Selector::parse(&selectors.time_selector).ok()?;

        let fhome_selector = Selector::parse(&selectors.home_selector).ok()?;
        let fscore_selector = Selector::parse(&selectors.score_selector).ok()?;
        let faway_selector = Selector::parse(&selectors.away_selector).ok()?;

        let fhgoal_selector = Selector::parse(&selectors.home_goal_selector).ok()?;
        let fagoal_selector = Selector::parse(&selectors.away_goal_selector).ok()?;
        let header_selector = Selector::parse(&selectors.shootout_score_selector).ok()?;

        let location_selector = Selector::parse(&selectors.location_selector).ok()?;
        let fright_selector = Selector::parse(&selectors.details_root_selector).ok()?;
        let div_selector = Selector::parse(&selectors.details_line_selector).ok()?;

        // 1. Extract Date and Time from fleft (routing fdate through parse_date_and_year to collapse whitespaces)
        let mut raw_date = String::new();
        let mut match_time = None;

        if let Some(fleft) = node.select(&fleft_selector).next() {
            if let Some(fdate) = fleft.select(&fdate_selector).next() {
                let (d, _) = self.parse_date_and_year(&fdate);
                raw_date = d;
            }
            if let Some(ftime) = fleft.select(&ftime_selector).next() {
                match_time = Some(element_text(&ftime));
            }
        }

        // 2. Extract Teams, Scores, Scorers from fevent table
        let table = node.select(&table_selector).next()?;
        let rows: Vec<_> = table.select(&row_selector).collect();
        let first_row = rows.first()?;

        let home_cell = first_row.select(&fhome_selector).next()?;
        let score_cell = first_row.select(&fscore_selector).next()?;
        let away_cell = first_row.select(&faway_selector).next()?;

        let home_name = element_text(&home_cell);
        let raw_score = element_text(&score_cell);
        let away_name = element_text(&away_cell);

        let mut home_scorers_raw = None;
        let mut away_scorers_raw = None;

        // Second row (tr.fgoals) contains goal scorers
        if let Some(second_row) = rows.get(1) {
            if let Some(hgoal_cell) = second_row.select(&fhgoal_selector).next() {
                let h_text = element_text(&hgoal_cell);
                if !h_text.is_empty() {
                    home_scorers_raw = Some(h_text);
                }
            }
            if let Some(agoal_cell) = second_row.select(&fagoal_selector).next() {
                let a_text = element_text(&agoal_cell);
                if !a_text.is_empty() {
                    away_scorers_raw = Some(a_text);
                }
            }
        }

        // If a fourth row (shootout) is present, parse penalty takers
        let mut home_shootout = Vec::new();
        let mut away_shootout = Vec::new();
        let mut shootout_score = None;

        if rows.len() >= 4 {
            if let Some(shootout_row) = rows.get(3) {
                let home_shoot_el = shootout_row.select(&fhgoal_selector).next();
                let away_shoot_el = shootout_row.select(&fagoal_selector).next();
                shootout_score = shootout_row
                    .select(&header_selector)
                    .next()
                    .map(|cell| element_text(&cell))
                    .filter(|text| !text.is_empty());

                if let (Some(h_shoot_cell), Some(a_shoot_cell)) = (home_shoot_el, away_shoot_el) {
                    home_shootout = parse_shootout_takers(&h_shoot_cell);
                    away_shootout = parse_shootout_takers(&a_shoot_cell);
                }
            }
        }

        // 3. Extract Stadium, Attendance, and Referee from fright
        let mut city_country = String::new();
        let mut stadium_raw = String::new();

        if let Some(fright) = node.select(&fright_selector).next() {
            if let Some(loc) = fright.select(&location_selector).next() {
                city_country = element_text(&loc);
            }

            for div in fright.select(&div_selector) {
                let text = clean_text(&div.text().collect::<String>());
                if div.value().attr("itemprop").is_none() && !text.is_empty() {
                    // Append metadata lines for the stadium parser
                    stadium_raw.push_str(&text);
                    stadium_raw.push(' ');
                }
            }
        }

        let stadium_details = if !stadium_raw.is_empty() {
            Some(stadium_raw)
        } else {
            None
        };

        Some(RawMatchData {
            raw_date,
            raw_time: match_time,
            raw_home_team: home_name,
            raw_away_team: away_name,
            raw_score,
            raw_home_scorers: home_scorers_raw,
            raw_away_scorers: away_scorers_raw,
            raw_city_country: if !city_country.is_empty() {
                Some(city_country)
            } else {
                None
            },
            raw_stadium_details: stadium_details,
            raw_shootout_score: shootout_score,
            raw_home_shootout: home_shootout,
            raw_away_shootout: away_shootout,
        })
    }
}
