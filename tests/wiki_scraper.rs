use sipabola_scrape_historical_data::wiki_scraper::{self, ScraperConfig};

fn test_config() -> ScraperConfig {
    ScraperConfig {
        year_header_class: "mw-heading3".to_string(),
        event_header_class: "vevent".to_string(),
    }
}

#[test]
fn scrapes_vevent_match_fixture() {
    let html = r#"
        <html>
          <body>
            <h2>1983 AFC Women's Championship</h2>
            <div class="vevent">
              <table>
                <tbody>
                  <tr>
                    <td><span>April 14</span><small>1983 AFC Women's Championship GS</small></td>
                    <td class="vcard attendee"><span class="fn org"><b>Philippines</b></span></td>
                    <td><span><b>2-0</b></span></td>
                    <td class="vcard attendee"><span class="fn org"><b>Hong Kong</b></span></td>
                    <td>Thailand</td>
                  </tr>
                  <tr>
                    <td></td>
                    <td>Laudeth Gonzalez <span class="fb-goal"><span>11'</span></span>, <span class="fb-goal"><span>39'</span></span></td>
                    <td><a>Report</a></td>
                    <td></td>
                    <td></td>
                  </tr>
                </tbody>
              </table>
            </div>
          </body>
        </html>
    "#;

    let records = wiki_scraper::scrape_matches(html, &test_config()).unwrap();

    assert_eq!(records.len(), 1);
    let record = &records[0];
    assert_eq!(record.year(), "1983");
    assert_eq!(record.competition(), "1983 AFC Women's Championship");
    assert_eq!(record.raw_date(), "April 14");
    assert_eq!(record.full_date(), "April 14, 1983");
    assert_eq!(record.home_team(), "Philippines");
    assert_eq!(record.away_team(), "Hong Kong");
    assert_eq!(record.city_country(), "Thailand");
    assert_eq!(record.stadium(), Some("Thailand"));
    assert_eq!(record.score().home().total_goals(), Some(2));
    assert_eq!(record.score().away().total_goals(), Some(0));
    assert_eq!(record.score().home().scorers().len(), 2);
    assert_eq!(
        record.score().home().scorers()[0].scorer(),
        "Laudeth Gonzalez"
    );
    assert_eq!(record.score().home().scorers()[0].minute(), "11");
}

#[test]
fn scrapes_footballbox_match_fixture() {
    let html = r#"
        <html>
          <body>
            <h2>2006 FIFA World Cup knockout stage</h2>
            <div itemscope="" itemtype="http://schema.org/SportsEvent" class="footballbox">
              <div class="fleft">
                <time>
                  <div class="fdate">30 June 2006</div>
                  <div class="ftime">17:00</div>
                </time>
              </div>
              <table class="fevent">
                <tbody>
                  <tr itemprop="name">
                    <th class="fhome" itemprop="homeTeam"><span itemprop="name">Germany</span></th>
                    <th class="fscore">1-1 (a.e.t.)</th>
                    <th class="faway" itemprop="awayTeam"><span itemprop="name">Argentina</span></th>
                  </tr>
                  <tr class="fgoals">
                    <td class="fhgoal">
                      <div class="plainlist"><ul><li>Klose <span>80'</span></li></ul></div>
                    </td>
                    <th></th>
                    <td class="fagoal">
                      <div class="plainlist"><ul><li>Ayala <span>49'</span></li></ul></div>
                    </td>
                  </tr>
                  <tr>
                    <th colspan="3">Penalties</th>
                  </tr>
                  <tr class="fgoals">
                    <td class="fhgoal">
                      <div class="plainlist">
                        <ul>
                          <li>Neuville <img alt="football with check mark" title="Penalty scored"></li>
                          <li>Ballack <img alt="football with check mark" title="Penalty scored"></li>
                        </ul>
                      </div>
                    </td>
                    <th>4-2</th>
                    <td class="fagoal">
                      <div class="plainlist">
                        <ul>
                          <li><img alt="football with check mark" title="Penalty scored"> Cruz</li>
                          <li><img alt="football with red X" title="Penalty missed"> Ayala</li>
                        </ul>
                      </div>
                    </td>
                  </tr>
                </tbody>
              </table>
              <div class="fright">
                <div itemprop="location"><span itemprop="name address">Olympiastadion, Berlin</span></div>
                <div>Attendance: 72,000</div>
                <div>Referee: Lubos Michel (Slovakia)</div>
              </div>
            </div>
          </body>
        </html>
    "#;

    let records = wiki_scraper::scrape_matches(html, &test_config()).unwrap();

    assert_eq!(records.len(), 1);
    let record = &records[0];
    assert_eq!(record.year(), "2006");
    assert_eq!(record.competition(), "2006 FIFA World Cup knockout stage");
    assert_eq!(record.raw_date(), "30 June 2006");
    assert_eq!(record.full_date(), "30 June 2006");
    assert_eq!(record.time(), Some("17:00"));
    assert_eq!(record.home_team(), "Germany");
    assert_eq!(record.away_team(), "Argentina");
    assert_eq!(record.city_country(), "Olympiastadion, Berlin");
    assert_eq!(record.stadium(), Some("Olympiastadion"));
    assert_eq!(record.attendance(), Some("72,000"));
    assert_eq!(record.referee(), Some("Lubos Michel (Slovakia)"));
    assert_eq!(record.score().home().total_goals(), Some(1));
    assert_eq!(record.score().away().total_goals(), Some(1));
    assert!(record.score().is_extra_time());
    assert_eq!(record.score().home().scorers()[0].scorer(), "Klose");
    assert_eq!(record.score().away().scorers()[0].scorer(), "Ayala");
    assert_eq!(record.score().home().shootout_takers().len(), 2);
    assert_eq!(record.score().away().shootout_takers().len(), 2);
    assert!(record.score().home().shootout_takers()[0].is_scored());
    assert!(!record.score().away().shootout_takers()[1].is_scored());
}
