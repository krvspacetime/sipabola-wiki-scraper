// src/wiki_scraper/vevent.rs
use super::{HtmlScraper, ScraperConfig};
use crate::models::{MatchRecord, MatchRecordBuilder, PenaltyShootoutTaker};
use crate::parser::{clean_city_country, clean_text, parse_match_score, parse_stadium_details};
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

    fn is_year_header(&self, node: &ElementRef, year_header_class: &str) -> bool {
        node.value()
            .classes()
            .any(|class| class == year_header_class)
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

    fn scrape_event_block(&self, node: ElementRef, year: &str) -> Option<MatchRecord> {
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
        let comp_name = raw_date_and_comp_name_vec[1..].join(" ").trim().to_string();
        let full_date = format!("{}, {}", raw_date, year);

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

        let mut score_record = parse_match_score(
            &raw_score,
            &home_name,
            &away_name,
            home_scorers_raw.as_deref(),
            away_scorers_raw.as_deref(),
        );

        if rows.len() >= 4 {
            if let Some(shootout_row) = rows.get(3) {
                let shootout_cells: Vec<_> = shootout_row.select(&cell_selector).collect();
                if shootout_cells.len() >= 3 {
                    let home_shootout = self.parse_shootout_takers(&shootout_cells[0]);
                    let away_shootout = self.parse_shootout_takers(&shootout_cells[2]);

                    if !home_shootout.is_empty() {
                        score_record.home_mut().set_shootout_takers(home_shootout);
                    }
                    if !away_shootout.is_empty() {
                        score_record.away_mut().set_shootout_takers(away_shootout);
                    }
                }
            }
        }

        let mut builder = MatchRecord::builder(year)
            .raw_date(raw_date)
            .competition(comp_name)
            .full_date(full_date)
            .home_team(home_name)
            .away_team(away_name)
            .score(score_record)
            .time(match_time);

        if let Some(city_raw) = first_row_cells
            .get(4)
            .map(|c| self.cell_text(c))
            .filter(|s| !s.is_empty())
        {
            let clean_city = clean_city_country(&city_raw);
            builder = builder.city_country(clean_city);
        }

        if let Some(raw_stadium) = stadium_raw {
            let (stadium, attendance, referee) = parse_stadium_details(&raw_stadium);
            builder = builder
                .stadium(stadium)
                .attendance(attendance)
                .referee(referee);
        }

        Some(builder.build())
    }
}

impl HtmlScraper for VeventScraper {
    fn can_scrape(&self, document: &Html, config: &ScraperConfig) -> bool {
        let selector_str = format!("div.{}", config.event_header_class);
        if let Ok(selector) = Selector::parse(&selector_str) {
            document.select(&selector).next().is_some()
        } else {
            false
        }
    }

    fn scrape(&self, html: &str, config: &ScraperConfig) -> anyhow::Result<Vec<MatchRecord>> {
        let document = Html::parse_document(html);

        let node_selector = Selector::parse(&format!(
            "div.{}, div.{}",
            config.year_header_class, config.event_header_class
        ))
        .map_err(|err| anyhow::anyhow!("invalid CSS selector: {err:?}"))?;

        let mut records = Vec::new();
        let mut current_year = String::new();

        for node in document.select(&node_selector) {
            if self.is_year_header(&node, &config.year_header_class) {
                if let Some(year) = node.text().next() {
                    current_year = year.trim().to_string();
                }
                continue;
            }

            if let Some(record) = self.scrape_event_block(node, &current_year) {
                if record.is_complete() {
                    records.push(record);
                }
            }
        }

        Ok(records)
    }
}
