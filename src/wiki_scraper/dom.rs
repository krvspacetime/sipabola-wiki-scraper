use crate::{models::PenaltyShootoutTaker, parser::clean_text};
use scraper::{ElementRef, Selector};

pub(super) fn element_text(element: &ElementRef) -> String {
    element
        .text()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

pub(super) fn non_empty_cell(cells: &[ElementRef], index: usize) -> Option<String> {
    cells
        .get(index)
        .map(element_text)
        .filter(|text| !text.is_empty())
}

pub(super) fn parse_shootout_takers(cell: &ElementRef) -> Vec<PenaltyShootoutTaker> {
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
