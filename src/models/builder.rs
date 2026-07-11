use super::match_record::{MatchRecord, MatchScoreRecord};
use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchRecordBuildError {
    MissingHomeTeam,
    MissingAwayTeam,
    MissingScore,
}

impl fmt::Display for MatchRecordBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingHomeTeam => f.write_str("match record is missing home team"),
            Self::MissingAwayTeam => f.write_str("match record is missing away team"),
            Self::MissingScore => f.write_str("match record is missing score"),
        }
    }
}

impl Error for MatchRecordBuildError {}

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

    pub fn build(self) -> Result<MatchRecord, MatchRecordBuildError> {
        if self.record.home_team.trim().is_empty() {
            return Err(MatchRecordBuildError::MissingHomeTeam);
        }
        if self.record.away_team.trim().is_empty() {
            return Err(MatchRecordBuildError::MissingAwayTeam);
        }
        if self.record.score.raw().trim().is_empty() {
            return Err(MatchRecordBuildError::MissingScore);
        }

        Ok(self.record)
    }
}
