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

    pub fn add_scorer(&mut self, goal: GoalDetail) {
        self.scorers.push(goal);
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

    pub fn city_country(mut self, value: impl Into<String>) -> Self {
        self.record.city_country = value.into();
        self
    }

    pub fn stadium(mut self, value: impl Into<String>) -> Self {
        self.record.stadium = Some(value.into());
        self
    }

    pub fn attendance(mut self, value: impl Into<String>) -> Self {
        self.record.attendance = Some(value.into());
        self
    }

    pub fn build(self) -> MatchRecord {
        self.record
    }
}
