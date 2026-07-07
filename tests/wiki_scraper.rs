use sipabola_wiki_scraper::{
    FootballBoxOverrides, ScraperConfig, ScraperConfigOverrides, SipabolaWikiScraper,
};

fn test_scraper() -> SipabolaWikiScraper {
    SipabolaWikiScraper::new()
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

    let records = test_scraper().scrape_html(html).unwrap();

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

    let records = test_scraper().scrape_html(html).unwrap();

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
    assert_eq!(record.score().home().penalty_shootout_goals(), Some(4));
    assert_eq!(record.score().away().penalty_shootout_goals(), Some(2));
    assert!(record.score().home().shootout_takers()[0].is_scored());
    assert!(!record.score().away().shootout_takers()[1].is_scored());
}

#[test]
fn ignores_nodes_that_match_page_selector_but_no_registered_scraper() {
    let html = r#"
        <html>
          <body>
            <h2>1983 Friendly</h2>
            <div class="match-card">
              <table>
                <tbody>
                  <tr>
                    <td>April 14</td>
                    <td>Philippines</td>
                    <td>2-0</td>
                    <td>Hong Kong</td>
                    <td>Thailand</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </body>
        </html>
    "#;

    let records = test_scraper().scrape_html(html).unwrap();

    assert!(records.is_empty());
}

#[test]
fn reports_basic_scrape_diagnostics() {
    let html = r#"
        <html>
          <body>
            <h2>1983 Friendly</h2>
            <div class="match-card"></div>
            <div class="vevent">
              <table>
                <tbody>
                  <tr>
                    <td>April 14</td>
                    <td>Philippines</td>
                    <td>2-0</td>
                    <td>Hong Kong</td>
                    <td>Thailand</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </body>
        </html>
    "#;

    let report = test_scraper().scrape_html_with_diagnostics(html).unwrap();

    assert_eq!(report.records.len(), 1);
    assert_eq!(report.diagnostics.matched_nodes, 1);
    assert_eq!(report.diagnostics.unsupported_nodes, 0);
    assert_eq!(report.diagnostics.extracted_matches, 1);
    assert_eq!(report.diagnostics.build_failures, 0);
}

#[test]
fn public_api_scrapes_html_without_writing_json() {
    let html = r#"
        <html>
          <body>
            <h2>1983 Friendly</h2>
            <div class="vevent">
              <table>
                <tbody>
                  <tr>
                    <td>April 14</td>
                    <td>Philippines</td>
                    <td>2-0</td>
                    <td>Hong Kong</td>
                    <td>Thailand</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </body>
        </html>
    "#;

    let records = SipabolaWikiScraper::new().scrape_html(html).unwrap();

    assert_eq!(records.len(), 1);
    assert_eq!(records[0].home_team(), "Philippines");
}

#[test]
fn api_config_overrides_footballbox_time_selector() {
    let html = r#"
        <html>
          <body>
            <h2>2006 FIFA World Cup knockout stage</h2>
            <div class="footballbox">
              <div class="fleft">
                <div class="fdate">30 June 2006</div>
                <div class="ftimeanddate">17:00</div>
              </div>
              <table class="fevent">
                <tbody>
                  <tr>
                    <th class="fhome">Germany</th>
                    <th class="fscore">1-0</th>
                    <th class="faway">Argentina</th>
                  </tr>
                </tbody>
              </table>
            </div>
          </body>
        </html>
    "#;

    let config = ScraperConfig::new().with_overrides(
        ScraperConfigOverrides::new()
            .footballbox(FootballBoxOverrides::new().time_selector(".ftimeanddate")),
    );

    let records = SipabolaWikiScraper::with_config(config)
        .scrape_html(html)
        .unwrap();

    assert_eq!(records.len(), 1);
    assert_eq!(records[0].time(), Some("17:00"));
}

#[test]
fn validates_invalid_selector_overrides_early() {
    let config = ScraperConfig::new().footballbox_time_selector("[");

    let err = SipabolaWikiScraper::with_config(config)
        .scrape_html("<html></html>")
        .unwrap_err();

    assert!(err.to_string().contains("invalid selector"));
}
