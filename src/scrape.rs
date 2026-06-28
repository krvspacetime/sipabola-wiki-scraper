use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[derive(Debug, Serialize)]
struct MatchRecord {
    raw_date: String,
    year: String,
    full_date: String,
    competition: String,
    home_team: String,
    away_team: String,
    score: String,
    home_scorers: Option<String>,
    away_scorers: Option<String>,
    status: String,
    city_country: String,
    stadium: Option<String>,
    attendance: Option<String>,
}

fn clean_text(text: &str) -> String {
    let re_citations = Regex::new(r"\[\d+\]").unwrap();
    let re_edit_links = Regex::new(r"\[edit\]").unwrap();

    let stepped = re_citations.replace_all(text, "");
    let stepped = re_edit_links.replace_all(&stepped, "");

    stepped.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn extract_stadium_and_attendance(text: &str) -> (Option<String>, Option<String>) {
    let text = clean_text(text);
    let mut stadium = None;
    let mut attendance = None;

    // Split at the "Attendance:" label if present
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

fn parse_era(url: &str) -> Result<Vec<MatchRecord>, Box<dyn Error>> {
    println!(
        "Fetching era page: {}",
        url.split('_').last().unwrap_or("URL")
    );

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()?;

    let response = client
        .get(url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) RustScraper",
        )
        .send()?
        .text()?;

    let document = Html::parse_document(&response);

    let main_selector = Selector::parse("h2, h3, h4, div.vevent")?;
    let table_selector = Selector::parse("table")?;
    let tr_selector = Selector::parse("tr")?;
    let td_selector = Selector::parse("td")?;
    let small_selector = Selector::parse("small")?;
    let location_selector = Selector::parse("span.location")?;

    let re_year = Regex::new(r"\b(19\d{2}|20\d{2})\b")?;

    let mut records = Vec::new();
    let mut current_year = String::from("Unknown Year");
    let mut current_competition = String::from("International Match");

    for node in document.select(&main_selector) {
        let tag_name = node.value().name();

        if tag_name == "h2" || tag_name == "h3" || tag_name == "h4" {
            let heading_text = clean_text(&node.text().collect::<String>());

            if let Some(caps) = re_year.captures(&heading_text) {
                current_year = caps[0].to_string();
            }

            if heading_text != current_year && heading_text.len() > 4 {
                current_competition = heading_text;
            }
        } else if tag_name == "div" {
            if let Some(table) = node.select(&table_selector).next() {
                let mut rows = table.select(&tr_selector);

                // Parse Row 1 (Header Row)
                if let Some(first_row) = rows.next() {
                    let cells: Vec<ElementRef> = first_row.select(&td_selector).collect();

                    if cells.len() >= 4 {
                        // Cell 0: Date & Localized Competition
                        let cell_0_text = clean_text(&cells[0].text().collect::<String>());
                        let mut date_str = cell_0_text.clone();
                        let mut competition = current_competition.clone();

                        if let Some(small_el) = cells[0].select(&small_selector).next() {
                            let comp_text = clean_text(&small_el.text().collect::<String>());
                            if !comp_text.is_empty() {
                                competition = comp_text.clone();
                                date_str = clean_text(&cell_0_text.replace(&comp_text, ""));
                            }
                        }

                        // Cell 1: Home Team
                        let home_team = clean_text(&cells[1].text().collect::<String>());

                        // Cell 2: Score
                        let score = clean_text(&cells[2].text().collect::<String>());

                        // Cell 3: Away Team
                        let away_team = clean_text(&cells[3].text().collect::<String>());

                        // Cell 4: City / Country
                        let mut city_country = String::new();
                        if cells.len() > 4 {
                            let raw_city = clean_text(&cells[4].text().collect::<String>());
                            city_country = raw_city
                                .strip_prefix("hide")
                                .unwrap_or(&raw_city)
                                .trim()
                                .to_string();
                        }

                        // Initialize the optional details we pull from the second row
                        let mut home_scorers = None;
                        let mut away_scorers = None;
                        let mut stadium = None;
                        let mut attendance = None;

                        // Parse Row 2 (Details Row)
                        if let Some(second_row) = rows.next() {
                            let sec_cells: Vec<ElementRef> =
                                second_row.select(&td_selector).collect();

                            if sec_cells.len() >= 4 {
                                // Cell 1: Home Scorers
                                let home_sc = clean_text(&sec_cells[1].text().collect::<String>());
                                if !home_sc.is_empty() {
                                    home_scorers = Some(home_sc);
                                }

                                // Cell 3: Away Scorers
                                let away_sc = clean_text(&sec_cells[3].text().collect::<String>());
                                if !away_sc.is_empty() {
                                    away_scorers = Some(away_sc);
                                }

                                // Cell 4: Stadium & Attendance
                                if sec_cells.len() > 4 {
                                    let details_text = &sec_cells[4].text().collect::<String>();
                                    let (std, att) = extract_stadium_and_attendance(details_text);

                                    // Fallback to span.location extraction if the text parse was blank
                                    if std.is_some() {
                                        stadium = std;
                                    } else if let Some(stadium_el) =
                                        sec_cells[4].select(&location_selector).next()
                                    {
                                        stadium = Some(clean_text(
                                            &stadium_el.text().collect::<String>(),
                                        ));
                                    }

                                    attendance = att;
                                }
                            }
                        }

                        let status = if score.chars().any(|c| c.is_numeric()) {
                            "Finished".to_string()
                        } else {
                            "Scheduled".to_string()
                        };

                        let full_date = format!("{}, {}", date_str, current_year);

                        records.push(MatchRecord {
                            raw_date: date_str,
                            year: current_year.clone(),
                            full_date,
                            competition,
                            home_team,
                            away_team,
                            score,
                            home_scorers,
                            away_scorers,
                            status,
                            city_country,
                            stadium,
                            attendance,
                        });
                    }
                }
            }
        }
    }

    Ok(records)
}

fn main() -> Result<(), Box<dyn Error>> {
    let era_urls = [
        "https://en.wikipedia.org/wiki/Philippines_women%27s_national_football_team_results_(1981%E2%80%931999)",
        "https://en.wikipedia.org/wiki/Philippines_women%27s_national_football_team_results_(2000%E2%80%932009)",
        "https://en.wikipedia.org/wiki/Philippines_women%27s_national_football_team_results_(2010%E2%80%932019)",
        "https://en.wikipedia.org/wiki/Philippines_women%27s_national_football_team_results_(2020%E2%80%93present)",
    ];

    let mut database = Vec::new();

    for url in &era_urls {
        match parse_era(url) {
            Ok(mut matches) => {
                println!("Successfully parsed {} matches.", matches.len());
                database.append(&mut matches);
            }
            Err(e) => {
                eprintln!("Error scraping {}: {}", url, e);
            }
        }
        thread::sleep(Duration::from_secs(1));
    }

    println!("Total historical records parsed: {}", database.len());

    let output_path = Path::new("philippines_women_history.json");
    let mut file = File::create(output_path)?;
    let json_data = serde_json::to_string_pretty(&database)?;
    file.write_all(json_data.as_bytes())?;

    println!("Success! File written to {:?}", output_path);
    Ok(())
}
