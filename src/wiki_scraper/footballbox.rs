// src/wiki_scraper/footballbox.rs
use super::{
    HtmlScraper, RawMatchData,
    dom::{element_text, parse_shootout_takers},
};
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
    fn can_scrape(&self, node: &ElementRef) -> bool {
        node.value().classes().any(|c| c == "footballbox")
    }

    fn extract_raw_match(&self, node: &ElementRef) -> Option<RawMatchData> {
        let table_selector = Selector::parse("table.fevent").ok()?;
        let row_selector = Selector::parse("tr").ok()?;

        let fleft_selector = Selector::parse(".fleft").unwrap();
        let fdate_selector = Selector::parse(".fdate").unwrap();
        let ftime_selector = Selector::parse(".ftime").unwrap();

        let fhome_selector = Selector::parse(".fhome").unwrap();
        let fscore_selector = Selector::parse(".fscore").unwrap();
        let faway_selector = Selector::parse(".faway").unwrap();

        let fhgoal_selector = Selector::parse(".fhgoal").unwrap();
        let fagoal_selector = Selector::parse(".fagoal").unwrap();

        let location_selector = Selector::parse("div[itemprop='location']").unwrap();
        let div_selector = Selector::parse("div").unwrap();

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

        if rows.len() >= 4 {
            if let Some(shootout_row) = rows.get(3) {
                let home_shoot_el = shootout_row.select(&fhgoal_selector).next();
                let away_shoot_el = shootout_row.select(&fagoal_selector).next();

                if let (Some(h_shoot_cell), Some(a_shoot_cell)) = (home_shoot_el, away_shoot_el) {
                    home_shootout = parse_shootout_takers(&h_shoot_cell);
                    away_shootout = parse_shootout_takers(&a_shoot_cell);
                }
            }
        }

        // 3. Extract Stadium, Attendance, and Referee from fright
        let mut city_country = String::new();
        let mut stadium_raw = String::new();

        let fright_selector = Selector::parse(".fright").unwrap();
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
            raw_home_shootout: home_shootout,
            raw_away_shootout: away_shootout,
        })
    }
}
