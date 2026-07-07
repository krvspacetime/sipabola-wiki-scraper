// src/wiki_scraper/vevent.rs
use super::{
    HtmlScraper, RawMatchData,
    dom::{element_text, has_configured_class, non_empty_cell, parse_shootout_takers},
};
use crate::config::ScraperConfig;
use scraper::{ElementRef, Html, Selector};

pub struct VeventScraper;

impl HtmlScraper for VeventScraper {
    fn root_selector<'a>(&self, config: &'a ScraperConfig) -> &'a str {
        &config.vevent.root_selector
    }

    fn can_scrape(&self, node: &ElementRef, config: &ScraperConfig) -> bool {
        has_configured_class(node, &config.vevent.root_selector)
    }

    fn extract_raw_match(&self, node: &ElementRef, config: &ScraperConfig) -> Option<RawMatchData> {
        let fragment = Html::parse_fragment(&node.html());
        let selectors = &config.vevent;
        let table_selector = Selector::parse(&selectors.table_selector).ok()?;
        let row_selector = Selector::parse(&selectors.row_selector).ok()?;
        let cell_selector = Selector::parse(&selectors.cell_selector).ok()?;

        let table = fragment.select(&table_selector).next()?;
        let rows: Vec<_> = table.select(&row_selector).collect();

        let first_row_cells: Vec<_> = rows.first()?.select(&cell_selector).collect();
        if first_row_cells.len() < 4 {
            return None;
        }

        let raw_date_and_comp_name_vec = first_row_cells[0].text().collect::<Vec<_>>();
        let raw_date = raw_date_and_comp_name_vec
            .first()
            .map(|text| text.trim().to_string())
            .unwrap_or_default();

        let home_name = element_text(&first_row_cells[1]);
        let away_name = element_text(&first_row_cells[3]);
        let raw_score = element_text(&first_row_cells[2]);

        let mut home_scorers_raw = None;
        let mut away_scorers_raw = None;
        let mut stadium_raw = None;
        let mut match_time = None;

        if let Some(second_row) = rows.get(1) {
            let second_row_cells: Vec<_> = second_row.select(&cell_selector).collect();
            if second_row_cells.len() >= 4 {
                match_time = non_empty_cell(&second_row_cells, 0);
                home_scorers_raw = non_empty_cell(&second_row_cells, 1);
                away_scorers_raw = non_empty_cell(&second_row_cells, 3);
                stadium_raw = non_empty_cell(&second_row_cells, 4);
            }
        }

        let mut home_shootout = Vec::new();
        let mut away_shootout = Vec::new();

        if rows.len() >= 4 {
            if let Some(shootout_row) = rows.get(3) {
                let shootout_cells: Vec<_> = shootout_row.select(&cell_selector).collect();
                if shootout_cells.len() >= 3 {
                    home_shootout = parse_shootout_takers(&shootout_cells[0]);
                    away_shootout = parse_shootout_takers(&shootout_cells[2]);
                }
            }
        }

        let city_raw = first_row_cells
            .get(4)
            .map(element_text)
            .filter(|s| !s.is_empty());

        Some(RawMatchData {
            raw_date,
            raw_time: match_time,
            raw_home_team: home_name,
            raw_away_team: away_name,
            raw_score,
            raw_home_scorers: home_scorers_raw,
            raw_away_scorers: away_scorers_raw,
            raw_city_country: city_raw,
            raw_stadium_details: stadium_raw,
            raw_shootout_score: None,
            raw_home_shootout: home_shootout,
            raw_away_shootout: away_shootout,
        })
    }
}
