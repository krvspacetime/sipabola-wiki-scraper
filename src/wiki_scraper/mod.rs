use scraper::{ElementRef, Html, Selector};

use crate::models::{MatchRecord};
use crate::parser::{clean_city_country, parse_match_score, parse_stadium_details};

pub struct ScraperConfig {
    pub year_header_class: String,
    pub event_header_class: String,
}

fn cell_text(cell: &ElementRef) -> String {
    cell.text().collect::<Vec<_>>().join(" ").trim().to_string()
}

fn non_empty_cell(cells: &[ElementRef], index: usize) -> Option<String> {
    cells.get(index).map(cell_text).filter(|s| !s.is_empty())
}

pub fn scrape_matches(html: &str, config: &ScraperConfig) -> anyhow::Result<Vec<MatchRecord>> {
    let document = Html::parse_document(html);

    let node_selector = Selector::parse(&format!(
        "div.{}, div.{}",
        config.year_header_class, config.event_header_class
    ))
    .map_err(|err| anyhow::anyhow!("invalid CSS selector: {err:?}"))?;

    let mut records = Vec::new();
    let mut current_year = String::new();

    for node in document.select(&node_selector) {
        if is_year_header(&node, &config.year_header_class) {
            if let Some(year) = node.text().next() {
                current_year = year.trim().to_string();
            }
            continue;
        }

        if let Some(record) = scrape_event_block(node, &current_year) {
            if record.is_complete() {
                records.push(record);
            }
        }
    }

    Ok(records)
}

fn is_year_header(node: &ElementRef, year_header_class: &str) -> bool {
    node.value()
        .classes()
        .any(|class| class == year_header_class)
}

fn scrape_event_block(node: ElementRef, year: &str) -> Option<MatchRecord> {
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

    let home_name = cell_text(&first_row_cells[1]);
    let away_name = cell_text(&first_row_cells[3]);
    let raw_score = cell_text(&first_row_cells[2]);

    let mut home_scorers_raw = None;
    let mut away_scorers_raw = None;
    let mut stadium_raw = None;

    if let Some(second_row) = rows.get(1) {
        let second_row_cells: Vec<_> = second_row.select(&cell_selector).collect();
        if second_row_cells.len() >= 4 {
            home_scorers_raw = non_empty_cell(&second_row_cells, 1);
            away_scorers_raw = non_empty_cell(&second_row_cells, 3);
            stadium_raw = non_empty_cell(&second_row_cells, 4);
        }
    }

    // Call out to our parser module to build the structured scoreline
    let score_record = parse_match_score(
        &raw_score,
        &home_name,
        &away_name,
        home_scorers_raw.as_deref(),
        away_scorers_raw.as_deref(),
    );

    let mut builder = MatchRecord::builder(year)
        .raw_date(raw_date)
        .competition(comp_name)
        .full_date(full_date)
        .home_team(home_name)
        .away_team(away_name)
        .score(score_record);

    // Row 1 Column 4 typically holds City/Country.
    // We clean up spacing errors like "Phnom Penh , Cambodia" here.
    if let Some(city_raw) = first_row_cells
        .get(4)
        .map(cell_text)
        .filter(|s| !s.is_empty())
    {
        let clean_city = clean_city_country(&city_raw);
        builder = builder.city_country(clean_city);
    }

    // Row 2 Column 4 typically holds Stadium and Attendance.
    if let Some(raw_stadium) = stadium_raw {
        let (stadium, attendance) = parse_stadium_details(&raw_stadium);
        builder = builder.stadium(stadium).attendance(attendance);
    }

    Some(builder.build())
}
