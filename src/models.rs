use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct GoalDetail {
    pub scorer: String,
    pub minute: String, // String to support added-time strings like "90+2'"
    pub is_penalty: bool,
    pub is_own_goal: bool,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct TeamScoreDetail {
    pub team_name: String,
    pub total_goals: Option<u32>,
    pub scorers: Vec<GoalDetail>,
    pub penalty_shootout_goals: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct MatchScoreRecord {
    pub raw: String,
    pub home: TeamScoreDetail,
    pub away: TeamScoreDetail,
    pub extra_time: bool,
    pub is_cancelled: bool,
    pub is_postponed: bool,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct MatchRecord {
    pub raw_date: String,
    pub year: String,
    pub full_date: String,
    pub competition: String,
    pub home_team: String,
    pub away_team: String,
    pub score: MatchScoreRecord, // Rich structured score record
    pub status: String,
    pub city_country: String,
    pub stadium: Option<String>,
    pub attendance: Option<String>,
}

impl MatchRecord {
    pub fn builder(year: impl Into<String>) -> MatchRecordBuilder {
        MatchRecordBuilder::new(year)
    }

    pub fn is_complete(&self) -> bool {
        !self.home_team.is_empty() && !self.away_team.is_empty() && !self.score.raw.is_empty()
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
