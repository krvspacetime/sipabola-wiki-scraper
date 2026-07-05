pub mod builder;
pub mod match_record;

pub use builder::{MatchRecordBuildError, MatchRecordBuilder};
pub use match_record::{
    GoalDetail, MatchRecord, MatchScoreRecord, PenaltyShootoutTaker, TeamScoreDetail,
};
