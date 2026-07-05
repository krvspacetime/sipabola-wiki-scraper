use sipabola_scrape_historical_data::{
    models::{MatchRecord, MatchRecordBuildError, MatchScoreRecord, TeamScoreDetail},
    parser::{parse_match_score, parse_scorers, parse_stadium_details},
};

#[test]
fn parses_score_status_and_shootout_score() {
    let score = parse_match_score(
        "1-1 (a.e.t.)",
        "Germany",
        "Argentina",
        Some("Klose 80'"),
        Some("Ayala 49'"),
        Some("4-2"),
    );

    assert_eq!(score.home().total_goals(), Some(1));
    assert_eq!(score.away().total_goals(), Some(1));
    assert!(score.is_extra_time());
    assert_eq!(score.home().penalty_shootout_goals(), Some(4));
    assert_eq!(score.away().penalty_shootout_goals(), Some(2));
    assert_eq!(score.home().scorers()[0].scorer(), "Klose");
    assert_eq!(score.away().scorers()[0].minute(), "49");
}

#[test]
fn parses_scorer_modifiers() {
    let goals = parse_scorers("Smith 12' (pen), Jones 45+2' (o.g.)");

    assert_eq!(goals.len(), 2);
    assert_eq!(goals[0].scorer(), "Smith");
    assert_eq!(goals[0].minute(), "12");
    assert!(goals[0].is_penalty());
    assert_eq!(goals[1].scorer(), "Jones");
    assert_eq!(goals[1].minute(), "45+2");
    assert!(goals[1].is_own_goal());
}

#[test]
fn parses_stadium_attendance_and_referee() {
    let (stadium, attendance, referee) =
        parse_stadium_details("Stadium: National Stadium Attendance: 12,345 Referee: Jane Doe");

    assert_eq!(stadium.as_deref(), Some("National Stadium"));
    assert_eq!(attendance.as_deref(), Some("12,345"));
    assert_eq!(referee.as_deref(), Some("Jane Doe"));
}

#[test]
fn builder_rejects_incomplete_records() {
    let score = MatchScoreRecord::new(
        "1-0".to_string(),
        TeamScoreDetail::new("Home".to_string()),
        TeamScoreDetail::new("Away".to_string()),
    );

    let result = MatchRecord::builder("2024")
        .home_team("Home")
        .score(score)
        .build();

    assert_eq!(result.unwrap_err(), MatchRecordBuildError::MissingAwayTeam);
}
