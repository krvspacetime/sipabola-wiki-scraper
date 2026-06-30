use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::models::{GoalDetail, MatchRecord, MatchScoreRecord};

pub struct ParserConfig {
    pub year_header_class: String,
    pub event_header_class: String,
}

fn clean_text(text: &str) -> String {
    let re_citations = Regex::new(r"\[\d+\]").unwrap();
    let re_edit_links = Regex::new(r"\[edit\]").unwrap();

    let stepped = re_citations.replace_all(text, "");
    let stepped = re_edit_links.replace_all(&stepped, "");

    stepped.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn cell_text(cell: &ElementRef) -> String {
    cell.text().collect::<Vec<_>>().join(" ").trim().to_string()
}

fn non_empty_cell(cells: &[ElementRef], index: usize) -> Option<String> {
    cells.get(index).map(cell_text).filter(|s| !s.is_empty())
}

pub fn parse_matches(html: &str, config: &ParserConfig) -> anyhow::Result<Vec<MatchRecord>> {
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

        if let Some(record) = parse_event_block(node, &current_year) {
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

fn parse_event_block(node: ElementRef, year: &str) -> Option<MatchRecord> {
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

    if let Some(city) = first_row_cells
        .get(4)
        .map(cell_text)
        .filter(|s| !s.is_empty())
    {
        builder = builder.city_country(city);
    }

    if let Some(raw_stadium) = stadium_raw {
        let (stadium, attendance) = extract_stadium_and_attendance(&raw_stadium);
        if let Some(std) = stadium {
            builder = builder.stadium(std);
        }
        if let Some(att) = attendance {
            builder = builder.attendance(att);
        }
    }

    Some(builder.build())
}

// ============ Scorer / Score Parser ============

pub fn parse_scorers(text: &str) -> Vec<GoalDetail> {
    let mut goals = Vec::new();

    // Captures digits (and added time) cleanly, keeping quote characters outside the capture group
    let re_goal = Regex::new(r"(\d{1,3}(?:\+\d+)?)\s*['ʼ’](?:\s*\((pen|o\.g\.|og)\.?)?").unwrap();

    let mut last_scorer = String::new();
    let mut last_idx = 0;

    for caps in re_goal.captures_iter(text) {
        let mat = caps.get(0).unwrap();
        let match_start = mat.start();
        let match_end = mat.end();

        // Isolate the text segment between the end of the previous goal and the start of the current one
        let raw_segment = &text[last_idx..match_start];

        // Clean the segment of common lists/line delimiters: ;, *, •, and whitespace
        let cleaned_name = raw_segment
            .trim_matches(|c: char| {
                c == ';' || c == ',' || c == '*' || c == '•' || c.is_whitespace()
            })
            .to_string();

        let cleaned_name = clean_text(&cleaned_name);

        // If the segment is empty, this goal belongs to the previous scorer (multi-goal game)
        let scorer_name = if cleaned_name.is_empty() {
            last_scorer.clone()
        } else {
            cleaned_name
        };

        if !scorer_name.is_empty() {
            let minute = caps[1].to_string(); // Extracted without quotes
            let modifier = caps.get(2).map(|m| m.as_str().to_lowercase());

            let is_penalty = modifier.as_deref() == Some("pen");
            let is_own_goal = modifier
                .as_ref()
                .map(|m| m.starts_with("o.g") || m.starts_with("og"))
                .unwrap_or(false);

            goals.push(GoalDetail::new(
                scorer_name.clone(),
                minute,
                is_penalty,
                is_own_goal,
            ));
            last_scorer = scorer_name;
        }

        last_idx = match_end;
    }

    goals
}

pub fn parse_match_score(
    raw_score: &str,
    home_name: &str,
    away_name: &str,
    home_scorers_raw: Option<&str>,
    away_scorers_raw: Option<&str>,
) -> MatchScoreRecord {
    let mut record = MatchScoreRecord::new(raw_score, home_name, away_name);
    let raw_lower = raw_score.trim().to_lowercase();

    if raw_lower.contains("cancelled") {
        record.set_cancelled(true);
        return record;
    }

    if raw_lower.contains("postponed") {
        record.set_postponed(true);
        return record;
    }

    if raw_lower.contains("a.e.t.") || raw_lower.contains("after extra time") {
        record.set_extra_time(true);
    }

    let re_score = Regex::new(r"(\d+)\s*[–-]\s*(\d+)").unwrap();
    let mut matches = re_score.captures_iter(raw_score);

    if let Some(caps) = matches.next() {
        let home_goals = caps[1].parse::<u32>().ok();
        let away_goals = caps[2].parse::<u32>().ok();
        record.home_mut().set_total_goals(home_goals);
        record.away_mut().set_total_goals(away_goals);
    }

    let re_penalties = Regex::new(r"\(\s*(\d+)\s*[–-]\s*(\d+)\s*(?:p|pen|penalties)\s*\)").unwrap();
    if let Some(caps) = re_penalties.captures(raw_score) {
        let home_pen = caps[1].parse::<u32>().ok();
        let away_pen = caps[2].parse::<u32>().ok();
        record.home_mut().set_penalty_shootout_goals(home_pen);
        record.away_mut().set_penalty_shootout_goals(away_pen);
    } else if let Some(caps) = matches.next() {
        if raw_lower.contains('p') || raw_lower.contains("pen") {
            let home_pen = caps[1].parse::<u32>().ok();
            let away_pen = caps[2].parse::<u32>().ok();
            record.home_mut().set_penalty_shootout_goals(home_pen);
            record.away_mut().set_penalty_shootout_goals(away_pen);
        }
    }

    if let Some(home_text) = home_scorers_raw {
        record.home_mut().set_scorers(parse_scorers(home_text));
    }
    if let Some(away_text) = away_scorers_raw {
        record.away_mut().set_scorers(parse_scorers(away_text));
    }

    record
}

fn extract_stadium_and_attendance(text: &str) -> (Option<String>, Option<String>) {
    let text = clean_text(text);
    let mut stadium = None;
    let mut attendance = None;

    let parts: Vec<&str> = text.split("Attendance:").collect();
    if parts.len() > 1 {
        let att_raw = parts[1].trim();
        if !att_raw.is_empty() {
            attendance = Some(att_raw.to_string());
        }
    }

    let stadium_part = parts[0].trim();
    let clean_stadium = if stadium_part.starts_with("Stadium:") {
        stadium_part.replacen("Stadium:", "", 1).trim().to_string()
    } else {
        stadium_part.to_string()
    };

    if !clean_stadium.is_empty() {
        stadium = Some(clean_stadium);
    }

    (stadium, attendance)
}
