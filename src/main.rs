use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, REFERER};
use scraper::{Html, Selector};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://inara.cz/elite/market-traderoutes/?ps1=Kamadhenu&pi10=19320&pi2=1000&pi5=8&pi3=3&pi9=0&pi4=0&pi7=50000&pi12=50000&pi14=0&pi15=0&pi1=0";

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) Gecko/20100101 Firefox/147.0")
        .build()?;

    let response = client
        .get(url)
        .header(
            ACCEPT,
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        )
        .header(ACCEPT_LANGUAGE, "en-US,en;q=0.9")
        .header(REFERER, "https://google.com")
        .send()?
        .text()?;

    let document = Html::parse_document(&response);
    let selector = Selector::parse(".traderouteboxprofit .itempairvalue").unwrap();

    if let Some(second) = document.select(&selector).nth(1) {
        println!(
            "{}",
            second
                .inner_html()
                .split_whitespace()
                .next()
                .unwrap()
                .trim()
        );
    }

    Ok(())
}
