// src/models/builder.rs
use super::match_record::{MatchRecord, MatchScoreRecord};

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

    pub fn stadium(mut self, value: Option<String>) -> Self {
        self.record.stadium = value;
        self
    }

    pub fn attendance(mut self, value: Option<String>) -> Self {
        self.record.attendance = value;
        self
    }

    pub fn time(mut self, value: Option<String>) -> Self {
        self.record.time = value;
        self
    }

    pub fn referee(mut self, value: Option<String>) -> Self {
        self.record.referee = value;
        self
    }

    pub fn build(self) -> MatchRecord {
        self.record
    }
}
