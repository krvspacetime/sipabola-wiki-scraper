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
If the class name changes you can override them using the API if you're using the library or a create a json config file if you're using the cli. If the structure changes though, I can't gurantee if everything will still work. Report the issue or open a PR in those cases.

Example config override using the cli.
```json
{
  "footballbox": {
    "time_selector": ".ftimeanddate"
  }
}
```

Using the library.

```rust
use sipabola_wiki_scraper::{
    FootballBoxOverrides, ScraperConfig, ScraperConfigOverrides, SipabolaWikiScraper,
};

let config = ScraperConfig::new()
    .with_overrides(ScraperConfigOverrides::new().footballbox(
        FootballBoxOverrides::new().time_selector(".ftimeanddate"),
    ));

let scraper = SipabolaWikiScraper::with_config(config);
let records = scraper.scrape_url("https://en.wikipedia.org/wiki/...")?;

```

## Use Cases
I personally used this tool to create a database for a project. I didn't want to pay for any subscription or API keys so this is what I used. Feel free to use it however you see fit.

## Library

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

## Selector Overrides

Selector overrides are for small Wikipedia class/selector changes inside a supported DOM structure. They are not meant to support a completely new match layout.

## CLI

Default run:

```powershell
cargo run -- "https://en.wikipedia.org/wiki/..."
```

Run with a partial JSON config:

```powershell
cargo run -- "https://en.wikipedia.org/wiki/..." --config config.example.json
```

Example config:

```json
{
  "footballbox": {
    "time_selector": ".ftimeanddate"
  }
}
```

The CLI writes JSON to `out/records.json` by default but can be specified using the --output flag.
