use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, CONNECTION, HOST, REFERER};
use scraper::{Html, Selector};
use std::error::Error;
use ua_generator::ua::spoof_firefox_linux_ua;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://inara.cz/elite/market-traderoutes/?ps1=Kamadhenu&pi10=19320&pi2=1000&pi5=8&pi3=3&pi9=0&pi4=0&pi7=50000&pi12=50000&pi14=0&pi15=0&pi1=0";

    let user_agent = spoof_firefox_linux_ua();
    let client = Client::builder().user_agent(user_agent).build()?;

    let response = client
        .get(url)
        .header(ACCEPT, "*/*")
        .header(ACCEPT_LANGUAGE, "en-US,en;q=0.9")
        .header(REFERER, url)
        .header(CONNECTION, "keep-alive")
        .header(HOST, "inara.cz")
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
    } else {
        panic!("Parsing error");
    }

    Ok(())
}
