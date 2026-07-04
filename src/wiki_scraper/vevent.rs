// src/wiki_scraper/vevent.rs
use super::{HtmlScraper, RawMatchData};
use crate::models::PenaltyShootoutTaker;
use crate::parser::clean_text;
use scraper::{ElementRef, Html, Selector};

pub struct VeventScraper;

impl VeventScraper {
    fn cell_text(&self, cell: &ElementRef) -> String {
        cell.text().collect::<Vec<_>>().join(" ").trim().to_string()
    }

    fn non_empty_cell(&self, cells: &[ElementRef], index: usize) -> Option<String> {
        cells
            .get(index)
            .map(|c| self.cell_text(c))
            .filter(|s| !s.is_empty())
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

impl HtmlScraper for VeventScraper {
    fn can_scrape(&self, node: &ElementRef) -> bool {
        node.value().classes().any(|c| c == "vevent")
    }

    fn extract_raw_match(&self, node: &ElementRef) -> Option<RawMatchData> {
        let fragment = Html::parse_fragment(&node.html());
        let table_selector = Selector::parse("table").ok()?;
        let row_selector = Selector::parse("tr").ok()?;
        let cell_selector = Selector::parse("td").ok()?;

        let table = fragment.select(&table_selector).next()?;
        let rows: Vec<_> = table.select(&row_selector).collect();

        let first_row_cells: Vec<_> = rows.first()?.select(&cell_selector).collect();
        if first_row_cells.len() < 4 {
            return None;
        }

        let raw_date_and_comp_name_vec = first_row_cells[0].text().collect::<Vec<_>>();
        let raw_date = raw_date_and_comp_name_vec[0].trim().to_string();

        let home_name = self.cell_text(&first_row_cells[1]);
        let away_name = self.cell_text(&first_row_cells[3]);
        let raw_score = self.cell_text(&first_row_cells[2]);

        let mut home_scorers_raw = None;
        let mut away_scorers_raw = None;
        let mut stadium_raw = None;
        let mut match_time = None;

        if let Some(second_row) = rows.get(1) {
            let second_row_cells: Vec<_> = second_row.select(&cell_selector).collect();
            if second_row_cells.len() >= 4 {
                match_time = self.non_empty_cell(&second_row_cells, 0);
                home_scorers_raw = self.non_empty_cell(&second_row_cells, 1);
                away_scorers_raw = self.non_empty_cell(&second_row_cells, 3);
                stadium_raw = self.non_empty_cell(&second_row_cells, 4);
            }
        }

        let mut home_shootout = Vec::new();
        let mut away_shootout = Vec::new();

        if rows.len() >= 4 {
            if let Some(shootout_row) = rows.get(3) {
                let shootout_cells: Vec<_> = shootout_row.select(&cell_selector).collect();
                if shootout_cells.len() >= 3 {
                    home_shootout = self.parse_shootout_takers(&shootout_cells[0]);
                    away_shootout = self.parse_shootout_takers(&shootout_cells[2]);
                }
            }
        }

        let city_raw = first_row_cells
            .get(4)
            .map(|c| self.cell_text(c))
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
            raw_home_shootout: home_shootout,
            raw_away_shootout: away_shootout,
        })
    }
}
