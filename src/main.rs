use scraper::{Html, Selector};

mod scrape;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let url = args.get(1).expect("url not provided");

    let client = reqwest::blocking::Client::new();
    let html = client
        .get(url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) RustScraper",
        )
        .send()?
        .text()?;

    let document = Html::parse_document(&html);
    let event_selector = Selector::parse(".mw-heading3, .vevent").unwrap();

    for node in document.select(&event_selector) {
        let element_value = node.value();
        let is_header = element_value.classes().any(|c| c == "mw-heading3");
        let is_event_container = element_value.classes().any(|c| c == "vevent");

        if is_header {
            let year_text = node.text().collect::<Vec<&str>>();
            println!("{}", year_text[0].trim());
        }

        if is_event_container {
            let fragment = Html::parse_fragment(&node.html());

            let event_name_selector = Selector::parse("small").unwrap();
            let event_name = fragment.select(&event_name_selector).collect::<Vec<_>>();

            let teams_selector = Selector::parse("td.vcard.attendee").unwrap();
            let teams = fragment.select(&teams_selector).collect::<Vec<_>>();
            let link_selector = Selector::parse("a").unwrap();

            println!(
                "Event: {}",
                event_name[0].text().collect::<Vec<_>>()[0].trim()
            );

            for (i, team) in teams.iter().enumerate() {
                if let Some(link) = team.select(&link_selector).next() {
                    let name: String = link.text().collect();
                    println!("  {}: {}", i + 1, name.trim());
                } else {
                    println!("  {}: Unknown Team", i + 1);
                }
            }
        }
    }

    Ok(())
}
