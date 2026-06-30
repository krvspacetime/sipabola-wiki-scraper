use regex::Regex;
use std::sync::OnceLock;

static RE_CITATIONS: OnceLock<Regex> = OnceLock::new();
static RE_EDIT_LINKS: OnceLock<Regex> = OnceLock::new();

pub fn clean_text(text: &str) -> String {
    let re_citations = RE_CITATIONS.get_or_init(|| Regex::new(r"\[\d+\]").unwrap());
    let re_edit_links = RE_EDIT_LINKS.get_or_init(|| Regex::new(r"\[edit\]").unwrap());

    // 1. Remove citations
    let stepped1 = re_citations.replace_all(text, "");

    // 2. Remove edit links (stepped2 safely borrows from stepped1, and both live until the end of this function)
    let stepped2 = re_edit_links.replace_all(&stepped1, "");

    // 3. Trim delimiters
    let trimmed = stepped2
        .trim_matches(|c: char| c == ';' || c == ',' || c == '*' || c == '•' || c.is_whitespace());

    // 4. Collapse whitespace and allocate a new owned String
    trimmed.split_whitespace().collect::<Vec<&str>>().join(" ")
}

pub fn clean_city_country(text: &str) -> String {
    let cleaned = clean_text(text);
    cleaned
        .replace(" ,", ",")
        .replace(" ;", ";")
        .replace(" (", " (")
        .trim()
        .to_string()
}

pub fn parse_stadium_details(text: &str) -> (Option<String>, Option<String>) {
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
