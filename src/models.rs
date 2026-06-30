use regex::Regex;
use scraper::ElementRef;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct GoalDetail {
    scorer: String,
    minute: String, // String to support added-time strings like "90+2"
    is_penalty: bool,
    is_own_goal: bool,
}

impl GoalDetail {
    pub fn new(
        scorer: impl Into<String>,
        minute: impl Into<String>,
        is_penalty: bool,
        is_own_goal: bool,
    ) -> Self {
        Self {
            scorer: scorer.into(),
            minute: minute.into(),
            is_penalty,
            is_own_goal,
        }
    }

    pub fn scorer(&self) -> &str {
        &self.scorer
    }
    pub fn minute(&self) -> &str {
        &self.minute
    }
    pub fn is_penalty(&self) -> bool {
        self.is_penalty
    }
    pub fn is_own_goal(&self) -> bool {
        self.is_own_goal
    }

    /// Parses a raw scorers block text into structured GoalDetail records
    pub fn parse_many(text: &str) -> Vec<Self> {
        let mut goals: Vec<GoalDetail> = Vec::new();
        let re_goal =
            Regex::new(r"(\d{1,3}(?:\+\d+)?)\s*['ʼ’](?:\s*\((pen|o\.g\.|og)\.?)?").unwrap();

        let mut last_scorer = String::new();
        let mut last_idx = 0;

        for caps in re_goal.captures_iter(text) {
            let mat = caps.get(0).unwrap();
            let match_start = mat.start();
            let match_end = mat.end();

            let raw_segment = &text[last_idx..match_start];
            let cleaned_name = clean_text_helper(raw_segment);

            let scorer_name = if cleaned_name.is_empty() {
                last_scorer.clone()
            } else {
                cleaned_name
            };

            if !scorer_name.is_empty() {
                let minute = caps[1].to_string(); // Captured without quote mark
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
}

fn clean_text_helper(text: &str) -> String {
    let re_citations = Regex::new(r"\[\d+\]").unwrap();
    let re_edit_links = Regex::new(r"\[edit\]").unwrap();

    let stepped = re_citations.replace_all(text, "");
    let stepped = re_edit_links.replace_all(&stepped, "");

    let trimmed = stepped
        .trim_matches(|c: char| c == ';' || c == ',' || c == '*' || c == '•' || c.is_whitespace());
    trimmed.split_whitespace().collect::<Vec<&str>>().join(" ")
}

#[derive(Debug, Clone, Serialize)]
pub struct TeamScoreDetail {
    team_name: String,
    total_goals: Option<u32>,
    scorers: Vec<GoalDetail>,
    penalty_shootout_goals: Option<u32>,
}

impl TeamScoreDetail {
    pub fn new(team_name: impl Into<String>) -> Self {
        Self {
            team_name: team_name.into(),
            total_goals: None,
            scorers: Vec::new(),
            penalty_shootout_goals: None,
        }
    }

    pub fn team_name(&self) -> &str {
        &self.team_name
    }
    pub fn total_goals(&self) -> Option<u32> {
        self.total_goals
    }
    pub fn scorers(&self) -> &[GoalDetail] {
        &self.scorers
    }
    pub fn penalty_shootout_goals(&self) -> Option<u32> {
        self.penalty_shootout_goals
    }

    pub fn set_total_goals(&mut self, goals: Option<u32>) {
        self.total_goals = goals;
    }
    pub fn set_penalty_shootout_goals(&mut self, goals: Option<u32>) {
        self.penalty_shootout_goals = goals;
    }
    pub fn set_scorers(&mut self, scorers: Vec<GoalDetail>) {
        self.scorers = scorers;
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchScoreRecord {
    raw: String,
    home: TeamScoreDetail,
    away: TeamScoreDetail,
    extra_time: bool,
    is_cancelled: bool,
    is_postponed: bool,
}

impl Default for MatchScoreRecord {
    fn default() -> Self {
        Self {
            raw: String::new(),
            home: TeamScoreDetail::new(""),
            away: TeamScoreDetail::new(""),
            extra_time: false,
            is_cancelled: false,
            is_postponed: false,
        }
    }
}

impl MatchScoreRecord {
    pub fn new(raw: impl Into<String>, home_name: &str, away_name: &str) -> Self {
        Self {
            raw: raw.into(),
            home: TeamScoreDetail::new(home_name),
            away: TeamScoreDetail::new(away_name),
            extra_time: false,
            is_cancelled: false,
            is_postponed: false,
        }
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }
    pub fn home(&self) -> &TeamScoreDetail {
        &self.home
    }
    pub fn away(&self) -> &TeamScoreDetail {
        &self.away
    }
    pub fn is_extra_time(&self) -> bool {
        self.extra_time
    }
    pub fn is_cancelled(&self) -> bool {
        self.is_cancelled
    }
    pub fn is_postponed(&self) -> bool {
        self.is_postponed
    }

    pub fn set_extra_time(&mut self, value: bool) {
        self.extra_time = value;
    }
    pub fn set_cancelled(&mut self, value: bool) {
        self.is_cancelled = value;
    }
    pub fn set_postponed(&mut self, value: bool) {
        self.is_postponed = value;
    }
    pub fn home_mut(&mut self) -> &mut TeamScoreDetail {
        &mut self.home
    }
    pub fn away_mut(&mut self) -> &mut TeamScoreDetail {
        &mut self.away
    }

    /// Parses match scores and scorer lists into a complete score record
    pub fn parse(
        raw_score: &str,
        home_name: &str,
        away_name: &str,
        home_scorers_raw: Option<&str>,
        away_scorers_raw: Option<&str>,
    ) -> Self {
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

        let re_penalties =
            Regex::new(r"\(\s*(\d+)\s*[–-]\s*(\d+)\s*(?:p|pen|penalties)\s*\)").unwrap();
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
            record
                .home_mut()
                .set_scorers(GoalDetail::parse_many(home_text));
        }
        if let Some(away_text) = away_scorers_raw {
            record
                .away_mut()
                .set_scorers(GoalDetail::parse_many(away_text));
        }

        record
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct MatchRecord {
    raw_date: String,
    year: String,
    full_date: String,
    competition: String,
    home_team: String,
    away_team: String,
    score: MatchScoreRecord,
    status: String,
    city_country: String,
    stadium: Option<String>,
    attendance: Option<String>,
}

impl MatchRecord {
    pub fn builder(year: impl Into<String>) -> MatchRecordBuilder {
        MatchRecordBuilder::new(year)
    }

    pub fn is_complete(&self) -> bool {
        !self.home_team.is_empty() && !self.away_team.is_empty() && !self.score.raw().is_empty()
    }

    pub fn raw_date(&self) -> &str {
        &self.raw_date
    }
    pub fn year(&self) -> &str {
        &self.year
    }
    pub fn full_date(&self) -> &str {
        &self.full_date
    }
    pub fn competition(&self) -> &str {
        &self.competition
    }
    pub fn home_team(&self) -> &str {
        &self.home_team
    }
    pub fn away_team(&self) -> &str {
        &self.away_team
    }
    pub fn score(&self) -> &MatchScoreRecord {
        &self.score
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn city_country(&self) -> &str {
        &self.city_country
    }
    pub fn stadium(&self) -> Option<&str> {
        self.stadium.as_deref()
    }
    pub fn attendance(&self) -> Option<&str> {
        self.attendance.as_deref()
    }
}

#[derive(Debug, Default)]
pub struct MatchRecordBuilder {
    record: MatchRecord,
}

impl MatchRecordBuilder {
    pub fn new(year: impl Into<String>) -> Self {
        Self {
            record: MatchRecord {
                year: year.into(),
                ..Default::default()
            },
        }
    }

    pub fn raw_date(mut self, value: impl Into<String>) -> Self {
        self.record.raw_date = value.into();
        self
    }

    pub fn full_date(mut self, value: impl Into<String>) -> Self {
        self.record.full_date = value.into();
        self
    }

    pub fn competition(mut self, value: impl Into<String>) -> Self {
        self.record.competition = value.into();
        self
    }

    pub fn home_team(mut self, value: impl Into<String>) -> Self {
        self.record.home_team = value.into();
        self
    }

    pub fn away_team(mut self, value: impl Into<String>) -> Self {
        self.record.away_team = value.into();
        self
    }

    pub fn score(mut self, value: MatchScoreRecord) -> Self {
        self.record.score = value;
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.record.status = value.into();
        self
    }

    pub fn city_country(mut self, value: &ElementRef) -> Self {
        let city = value
            .text()
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
            .replace(" ,", ",");
        self.record.city_country = city;
        self
    }

    pub fn raw_stadium(mut self, value: &str) -> Self {
        let (stadium, attendance) = extract_stadium_and_attendance(value);
        self.record.stadium = stadium;
        self.record.attendance = attendance;
        self
    }

    pub fn build(self) -> MatchRecord {
        self.record
    }
}

fn extract_stadium_and_attendance(text: &str) -> (Option<String>, Option<String>) {
    let text = clean_text_helper(text);
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
