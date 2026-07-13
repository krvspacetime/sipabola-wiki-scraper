# Sipabola Wiki Scraper

Scrape football match records from Wikipedia pages that use common football match results and schedule templates.

#### Works for these kind of Wikipedia pages.

[World Cup 2026](https://en.wikipedia.org/wiki/Norway_national_football_team_results_(2020%E2%80%93present))

[World Cup 2022](https://en.wikipedia.org/wiki/2022_FIFA_World_Cup)

[Philippine Women's National Team Results from 1981-1999](https://en.wikipedia.org/wiki/Philippines_women%27s_national_football_team_results_(1981%E2%80%931999))

[Norway Men's Foothball Team Results from 2020-Present](https://en.wikipedia.org/wiki/Norway_national_football_team_results_(2020%E2%80%93present))

## Selectors
If the class "vevent" and "footballbox" wraps the details table, it will probably work.

```html
<div
    itemscope=""
    itemtype="http://schema.org/SportsEvent"
    class="footballbox"
    style="color: inherit"
    about="#mwt544"
    id="mwBlk"
>
    <div class="fleft">
        <time
            ><div class="fdate">
                30<span typeof="mw:Entity">&nbsp;</span>June<span
                    typeof="mw:Entity"
                    >&nbsp;</span
                >
```

```html
<div
    class="vevent"
    about="#mwt38"
    typeof="mw:Transclusion"
    id="mwJA"
    data-mw='{"parts":[{"template":{"target":{"wt":"football box collapsible\n","href":"./Template:Football_box_collapsible"},"params":{"round":{"wt":"[[1983 AFC Women&apos;s Championship|1983 AFC Women&apos;s Championship GS]]"},"date":{"wt":"April 14"},"time":{"wt":""},"team1":{"wt":"{{fbw-rt|PHI|1936}}"},"score":{"wt":"2–0"},"report":{"wt":"[https://www.rsssf.org/tablesa/aswomen.html#p83 Report]"},"team2":{"wt":"{{fbw|HKG|colonial}}"},"goals1":{"wt":"Laudeth Gonzalez {{goal|11}},{{goal|HT+39}}"},"goals2":{"wt":""},"stadium":{"wt":""},"location":{"wt":"Thailand"},"attendance":{"wt":""},"referee":{"wt":""},"result":{"wt":"W"}},"i":0}}]}'
>
    <span class="summary" style="display: none">
```

## Potential Issues
If the class name changes you can override them, especially the root selector. If the overall DOM structure changes though in conjuction with the class name changes, I can't gurantee everything will still work without code updates. For those cases open an issue/PR.

```rust
use sipabola_wiki_scraper::{
    FootballBoxOverrides, ScraperConfig, ScraperConfigOverrides, SipabolaWikiScraper,
};

let config = ScraperConfig::new()
    .with_overrides(ScraperConfigOverrides::new().footballbox(
        FootballBoxOverrides::new().root_selector(".footballboxnew"),
    ));

let scraper = SipabolaWikiScraper::with_config(config);
let records = scraper.scrape_url("https://en.wikipedia.org/wiki/...")?;

```

If you're using the cli, you can create a config file as pass it via the --config flag.
```json
{
  "footballbox": {
    "root_selector": "footballboxnew",
    "time_selector": ".ftimeanddate"
  }
}
```
## Library API

Use the client API. It returns `Vec<MatchRecord>` and does not write JSON or touch the filesystem.

```rust
use sipabola_wiki_scraper::SipabolaWikiScraper;

let scraper = SipabolaWikiScraper::new();
let records = scraper.scrape_url("https://en.wikipedia.org/wiki/...")?;
```

If you already have the raw HTML:

```rust
use sipabola_wiki_scraper::SipabolaWikiScraper;

let scraper = SipabolaWikiScraper::new();
let records = scraper.scrape_html(&html)?;
```

Or if you have the html file.

```rust
let records = scraper.scrape_html_file("path/to/file.html")?;
```

## Output
JSON output after running cli on the 2026 World Cup Wikipedia page.

```json
[
  {
    "raw_date": "June 28, 2026",
    "year": "2026",
    "full_date": "June 28, 2026",
    "competition": "Round of 32",
    "home_team": "South Africa",
    "away_team": "Canada",
    "score": {
      "raw": "0–1",
      "home": {
        "team_name": "South Africa",
        "total_goals": 0,
        "scorers": [],
        "penalty_shootout_goals": null,
        "shootout_takers": []
      },
      "away": {
        "team_name": "Canada",
        "total_goals": 1,
        "scorers": [
          {
            "scorer": "Eustáquio",
            "minute": "90+2",
            "is_penalty": false,
            "is_own_goal": false
          }
        ],
        "penalty_shootout_goals": null,
        "shootout_takers": []
      },
      "extra_time": false,
      "is_cancelled": false,
      "is_postponed": false
    },
    "city_country": "SoFi Stadium, Inglewood",
    "stadium": "SoFi Stadium",
    "attendance": "69,237",
    "time": "12:00 p.m. UTC−7",
    "referee": "João Pinheiro (Portugal)"
  },
```

## CLI

Default run:

```powershell
cargo run -- "https://en.wikipedia.org/wiki/..."
```

Run with a partial JSON config:

```powershell
cargo run -- "https://en.wikipedia.org/wiki/..." --config config.example.json
```

The CLI writes JSON to `out/records.json` by default but can be specified using the --output flag.
