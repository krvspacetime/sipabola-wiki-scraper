use crate::{models::GoalDetail, parser::clean_text};
use regex::Regex;
use std::sync::OnceLock;

static RE_GOAL: OnceLock<Regex> = OnceLock::new();

pub fn parse_scorers(text: &str) -> Vec<GoalDetail> {
    let mut goals = Vec::new();

    // Allows optional whitespace inside the parentheses, e.g., "( pen. )" or "(o.g.)"
    let re_goal = RE_GOAL.get_or_init(|| {
        Regex::new(r"(\d{1,3}(?:\+\d+)?)\s*['ʼ’](?:\s*\(\s*(pen|penalty|o\.g\.|og|p)\.?\s*\))?")
            .unwrap()
    });

    let mut last_scorer = String::new();
    let mut last_idx = 0;

    for caps in re_goal.captures_iter(text) {
        let mat = caps.get(0).unwrap();
        let match_start = mat.start();
        let match_end = mat.end();

        let raw_segment = &text[last_idx..match_start];
        let cleaned_name = clean_text(raw_segment);

        let scorer_name = if cleaned_name.is_empty() {
            last_scorer.clone()
        } else {
            cleaned_name
        };

        if !scorer_name.is_empty() {
            let minute = caps[1].to_string(); // Captured without quote mark
            let modifier = caps.get(2).map(|m| m.as_str().to_lowercase());

            let is_penalty = modifier
                .as_ref()
                .map(|m| m.starts_with('p'))
                .unwrap_or(false);
            let is_own_goal = modifier
                .as_ref()
                .map(|m| m.starts_with('o'))
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
