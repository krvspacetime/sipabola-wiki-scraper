use crate::{
    models::{MatchScoreRecord, TeamScoreDetail},
    parser::parse_scorers,
};
use regex::Regex;
use std::sync::OnceLock;

static RE_SCORE: OnceLock<Regex> = OnceLock::new();
static RE_PENALTIES: OnceLock<Regex> = OnceLock::new();

pub fn parse_match_score(
    raw_score: &str,
    home_name: &str,
    away_name: &str,
    home_scorers_raw: Option<&str>,
    away_scorers_raw: Option<&str>,
    shootout_score_raw: Option<&str>,
) -> MatchScoreRecord {
    let home_detail = TeamScoreDetail::new(home_name.to_string());
    let away_detail = TeamScoreDetail::new(away_name.to_string());

    let mut record = MatchScoreRecord::new(raw_score.to_string(), home_detail, away_detail);
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

    let re_score = RE_SCORE.get_or_init(|| Regex::new(r"(\d+)\s*[–-]\s*(\d+)").unwrap());
    let mut matches = re_score.captures_iter(raw_score);

    if let Some(caps) = matches.next() {
        let home_goals = caps[1].parse::<u32>().ok();
        let away_goals = caps[2].parse::<u32>().ok();
        record.home_mut().set_total_goals(home_goals);
        record.away_mut().set_total_goals(away_goals);
    }

    let re_penalties = RE_PENALTIES.get_or_init(|| {
        Regex::new(r"\(\s*(\d+)\s*[–-]\s*(\d+)\s*(?:p|pen|penalties)\s*\)").unwrap()
    });

    if let Some(caps) = shootout_score_raw.and_then(|score| re_score.captures(score)) {
        let home_pen = caps[1].parse::<u32>().ok();
        let away_pen = caps[2].parse::<u32>().ok();
        record.home_mut().set_penalty_shootout_goals(home_pen);
        record.away_mut().set_penalty_shootout_goals(away_pen);
    } else if let Some(caps) = re_penalties.captures(raw_score) {
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
