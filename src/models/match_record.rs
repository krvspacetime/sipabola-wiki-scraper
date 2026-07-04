use serde::Serialize;

use crate::models::MatchRecordBuilder; // Add this line

#[derive(Debug, Clone, Serialize, Default)]
pub struct GoalDetail {
    pub(super) scorer: String,
    pub(super) minute: String,
    pub(super) is_penalty: bool,
    pub(super) is_own_goal: bool,
}

impl GoalDetail {
    pub fn new(scorer: String, minute: String, is_penalty: bool, is_own_goal: bool) -> Self {
        Self {
            scorer,
            minute,
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

#[derive(Debug, Clone, Serialize, Default)]
pub struct PenaltyShootoutTaker {
    pub(super) taker: String,
    pub(super) is_scored: bool,
}

impl PenaltyShootoutTaker {
    pub fn new(taker: String, is_scored: bool) -> Self {
        Self { taker, is_scored }
    }

    pub fn taker(&self) -> &str {
        &self.taker
    }
    pub fn is_scored(&self) -> bool {
        self.is_scored
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TeamScoreDetail {
    pub(super) team_name: String,
    pub(super) total_goals: Option<u32>,
    pub(super) scorers: Vec<GoalDetail>,
    pub(super) penalty_shootout_goals: Option<u32>,
    pub(super) shootout_takers: Vec<PenaltyShootoutTaker>, // Added shootout takers list
}

impl TeamScoreDetail {
    pub fn new(team_name: String) -> Self {
        Self {
            team_name,
            total_goals: None,
            scorers: Vec::new(),
            penalty_shootout_goals: None,
            shootout_takers: Vec::new(),
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
    pub fn shootout_takers(&self) -> &[PenaltyShootoutTaker] {
        &self.shootout_takers
    }

    pub(crate) fn set_total_goals(&mut self, goals: Option<u32>) {
        self.total_goals = goals;
    }

    pub(crate) fn set_penalty_shootout_goals(&mut self, goals: Option<u32>) {
        self.penalty_shootout_goals = goals;
    }

    pub(crate) fn set_scorers(&mut self, scorers: Vec<GoalDetail>) {
        self.scorers = scorers;
    }

    pub(crate) fn set_shootout_takers(&mut self, takers: Vec<PenaltyShootoutTaker>) {
        self.shootout_takers = takers;
    }
}

impl Default for TeamScoreDetail {
    fn default() -> Self {
        Self::new(String::new())
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct MatchScoreRecord {
    pub(super) raw: String,
    pub(super) home: TeamScoreDetail,
    pub(super) away: TeamScoreDetail,
    pub(super) extra_time: bool,
    pub(super) is_cancelled: bool,
    pub(super) is_postponed: bool,
}

impl MatchScoreRecord {
    pub fn new(raw: String, home: TeamScoreDetail, away: TeamScoreDetail) -> Self {
        Self {
            raw,
            home,
            away,
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

    pub(crate) fn set_extra_time(&mut self, value: bool) {
        self.extra_time = value;
    }

    pub(crate) fn set_cancelled(&mut self, value: bool) {
        self.is_cancelled = value;
    }

    pub(crate) fn set_postponed(&mut self, value: bool) {
        self.is_postponed = value;
    }

    pub(crate) fn home_mut(&mut self) -> &mut TeamScoreDetail {
        &mut self.home
    }

    pub(crate) fn away_mut(&mut self) -> &mut TeamScoreDetail {
        &mut self.away
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct MatchRecord {
    pub(super) raw_date: String,
    pub(super) year: String,
    pub(super) full_date: String,
    pub(super) competition: String,
    pub(super) home_team: String,
    pub(super) away_team: String,
    pub(super) score: MatchScoreRecord,
    pub(super) status: String,
    pub(super) city_country: String,
    pub(super) stadium: Option<String>,
    pub(super) attendance: Option<String>,
    pub(super) time: Option<String>,    // Added match kickoff time
    pub(super) referee: Option<String>, // Added referee name
}

// Inside: impl MatchRecord inside src/models/match_record.rs
impl MatchRecord {
    // Add this associated function
    pub fn builder(year: impl Into<String>) -> MatchRecordBuilder {
        MatchRecordBuilder::new(year)
    }

    pub fn is_complete(&self) -> bool {
        !self.home_team.is_empty() && !self.away_team.is_empty() && !self.score.raw.is_empty()
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
    pub fn time(&self) -> Option<&str> {
        self.time.as_deref()
    }
    pub fn referee(&self) -> Option<&str> {
        self.referee.as_deref()
    }
}
