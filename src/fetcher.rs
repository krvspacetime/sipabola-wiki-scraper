const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) RustScraper";

pub fn fetch_html(url: &str) -> anyhow::Result<String> {
    let client = reqwest::blocking::Client::new();
    let html = client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()?
        .text()?;
    Ok(html)
}
