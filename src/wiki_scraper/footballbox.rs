// src/wiki_scraper/footballbox.rs
use super::{HtmlScraper, ScraperConfig};
use crate::models::{MatchRecord, MatchRecordBuilder, PenaltyShootoutTaker};
use crate::parser::{clean_city_country, clean_text, parse_match_score, parse_stadium_details};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

pub struct FootballBoxScraper;

impl FootballBoxScraper {
    fn cell_text(&self, cell: &ElementRef) -> String {
        cell.text().collect::<Vec<_>>().join(" ").trim().to_string()
    }

    fn parse_date_and_year(&self, cell: &ElementRef) -> (String, String) {
        let text = clean_text(&cell.text().collect::<String>());
        let re_year = Regex::new(r"\b(19\d{2}|20\d{2})\b").unwrap();
        let year = if let Some(caps) = re_year.captures(&text) {
            caps[0].to_string()
        } else {
            String::new()
        };
        (text, year)
    }

    fn parse_shootout_takers(&self, cell: &ElementRef) -> Vec<PenaltyShootoutTaker> {
        let mut takers = Vec::new();
        let li_selector = Selector::parse("li").unwrap();
        let img_selector = Selector::parse("img").unwrap();

        for li in cell.select(&li_selector) {
            let taker_name = clean_text(&li.text().collect::<String>());
            if taker_name.is_empty() {
                continue;
            }

            let mut is_scored = false;
            if let Some(img) = li.select(&img_selector).next() {
                let alt = img.value().attr("alt").unwrap_or("").to_lowercase();
                let title = img.value().attr("title").unwrap_or("").to_lowercase();
                if alt.contains("check") || title.contains("scored") || title.contains("check") {
                    is_scored = true;
                }
            }

            takers.push(PenaltyShootoutTaker::new(taker_name, is_scored));
        }

        takers
    }
}

impl HtmlScraper for FootballBoxScraper {
    fn can_scrape(&self, document: &Html, _config: &ScraperConfig) -> bool {
        let selector = Selector::parse("div.footballbox").unwrap();
        document.select(&selector).next().is_some()
    }

    fn scrape(&self, html: &str, config: &ScraperConfig) -> anyhow::Result<Vec<MatchRecord>> {
        let document = Html::parse_document(html);
        let box_selector = Selector::parse("div.footballbox").unwrap();

        let fleft_selector = Selector::parse(".fleft").unwrap();
        let fdate_selector = Selector::parse(".fdate").unwrap();
        let ftime_selector = Selector::parse(".ftime").unwrap();
        let fevent_selector = Selector::parse("table.fevent").unwrap();
        let tr_selector = Selector::parse("tr").unwrap();

        let fhome_selector = Selector::parse(".fhome").unwrap();
        let fscore_selector = Selector::parse(".fscore").unwrap();
        let faway_selector = Selector::parse(".faway").unwrap();

        let fhgoal_selector = Selector::parse(".fhgoal").unwrap();
        let fagoal_selector = Selector::parse(".fagoal").unwrap();

        let location_selector = Selector::parse("div[itemprop='location']").unwrap();
        let div_selector = Selector::parse("div").unwrap();

        let mut records = Vec::new();

        for node in document.select(&box_selector) {
            // 1. Extract Date and Time from fleft
            let mut raw_date = String::new();
            let mut year = String::new();
            let mut match_time = None;

            if let Some(fleft) = node.select(&fleft_selector).next() {
                if let Some(fdate) = fleft.select(&fdate_selector).next() {
                    let (d, y) = self.parse_date_and_year(&fdate);
                    raw_date = d;
                    year = y;
                }
                if let Some(ftime) = fleft.select(&ftime_selector).next() {
                    match_time = Some(self.cell_text(&ftime));
                }
            }

            // 2. Extract Teams, Scores, Scorers from fevent table
            if let Some(table) = node.select(&fevent_selector).next() {
                let rows: Vec<_> = table.select(&tr_selector).collect();
                if let Some(first_row) = rows.first() {
                    let home_el = first_row.select(&fhome_selector).next();
                    let score_el = first_row.select(&fscore_selector).next();
                    let away_el = first_row.select(&faway_selector).next();

                    if let (Some(home_cell), Some(score_cell), Some(away_cell)) =
                        (home_el, score_el, away_el)
                    {
                        let home_name = self.cell_text(&home_cell);
                        let raw_score = self.cell_text(&score_cell);
                        let away_name = self.cell_text(&away_cell);

                        let mut home_scorers_raw = None;
                        let mut away_scorers_raw = None;

                        // Second row (tr.fgoals) contains goal scorers
                        if let Some(second_row) = rows.get(1) {
                            if let Some(hgoal_cell) = second_row.select(&fhgoal_selector).next() {
                                let h_text = self.cell_text(&hgoal_cell);
                                if !h_text.is_empty() {
                                    home_scorers_raw = Some(h_text);
                                }
                            }
                            if let Some(agoal_cell) = second_row.select(&fagoal_selector).next() {
                                let a_text = self.cell_text(&agoal_cell);
                                if !a_text.is_empty() {
                                    away_scorers_raw = Some(a_text);
                                }
                            }
                        }

                        // Parse the scoreline
                        let mut score_record = parse_match_score(
                            &raw_score,
                            &home_name,
                            &away_name,
                            home_scorers_raw.as_deref(),
                            away_scorers_raw.as_deref(),
                        );

                        // If a fourth row (shootout) is present, parse penalty takers
                        if rows.len() >= 4 {
                            if let Some(shootout_row) = rows.get(3) {
                                let home_shoot_el = shootout_row.select(&fhgoal_selector).next();
                                let away_shoot_el = shootout_row.select(&fagoal_selector).next();

                                if let (Some(h_shoot_cell), Some(a_shoot_cell)) =
                                    (home_shoot_el, away_shoot_el)
                                {
                                    let home_shootout = self.parse_shootout_takers(&h_shoot_cell);
                                    let away_shootout = self.parse_shootout_takers(&a_shoot_cell);

                                    if !home_shootout.is_empty() {
                                        score_record.home_mut().set_shootout_takers(home_shootout);
                                    }
                                    if !away_shootout.is_empty() {
                                        score_record.away_mut().set_shootout_takers(away_shootout);
                                    }
                                }
                            }
                        }

                        let full_date = format!("{}, {}", raw_date, year);
                        let mut builder = MatchRecord::builder(&year)
                            .raw_date(raw_date)
                            .competition(&config.year_header_class) // Sourced from configuration header context
                            .full_date(full_date)
                            .home_team(home_name)
                            .away_team(away_name)
                            .score(score_record)
                            .time(match_time);

                        // 3. Extract Stadium, Attendance, and Referee from fright
                        let mut city_country = String::new();
                        let mut stadium = None;
                        let mut attendance = None;
                        let mut referee = None;

                        let fright_selector = Selector::parse(".fright").unwrap();
                        if let Some(fright) = node.select(&fright_selector).next() {
                            if let Some(loc) = fright.select(&location_selector).next() {
                                city_country = clean_city_country(&self.cell_text(&loc));
                            }

                            for div in fright.select(&div_selector) {
                                let text = clean_text(&div.text().collect::<String>());
                                if text.starts_with("Attendance:") {
                                    attendance = Some(
                                        text.replacen("Attendance:", "", 1).trim().to_string(),
                                    );
                                } else if text.starts_with("Referee:") {
                                    referee =
                                        Some(text.replacen("Referee:", "", 1).trim().to_string());
                                } else if div.value().attr("itemprop").is_none() && !text.is_empty()
                                {
                                    // Location string parsing fallback to stadium
                                    let parts: Vec<&str> = text.split(',').collect();
                                    if !parts.is_empty() {
                                        stadium = Some(parts[0].trim().to_string());
                                    }
                                }
                            }
                        }

                        builder = builder
                            .city_country(city_country)
                            .stadium(stadium)
                            .attendance(attendance)
                            .referee(referee);

                        let record = builder.build();
                        if record.is_complete() {
                            records.push(record);
                        }
                    }
                }
            }
        }

        Ok(records)
    }
}
