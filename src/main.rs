use reqwest::blocking::get;
use reqwest::StatusCode;
use std::io::{self, Write};

fn main() {
    print!("Enter a URL (with http:// or https://): ");
    io::stdout().flush().unwrap();

    let mut url = String::new();
    io::stdin().read_line(&mut url).unwrap();
    let url = url.trim();

    println!("Fetching: {}", url);

    match fetch_page(url) {
        Ok(content) => {
            println!("--- Parsed Content ---");
            let ad_count = parse_and_print(&content);
            if ad_count > 0 {
                println!();
                println!(
                    "⚠️  Detected {} potential ad/tracker link(s) or keywords on this page!",
                    ad_count
                );
            }
            println!("--- End ---");
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

// Improved error handling, including for 404s and connection errors.
fn fetch_page(url: &str) -> Result<String, String> {
    match get(url) {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                match response.text() {
                    Ok(text) => Ok(text),
                    Err(_) => Err("Failed to read response text.".into()),
                }
            } else {
                Err(format!("Page returned HTTP status code: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Failed to fetch page: {}", e)),
    }
}

// Returns the number of detected ad/tracker links/keywords
fn parse_and_print(html: &str) -> usize {
    let document = scraper::Html::parse_document(html);
    let mut ad_count = 0;

    // Ad/tracker keyword list
    let ad_keywords = [
        "ad", "banner", "doubleclick", "googletag", "tracker", "adsystem", "analytics", "pixel",
        "click", "sponsor",
    ];

    // <title>
    let title_selector = scraper::Selector::parse("title").unwrap();
    if let Some(title) = document.select(&title_selector).next() {
        println!("Title: {}", title.text().collect::<Vec<_>>().join(" "));
    }

    // <h1>
    let h1_selector = scraper::Selector::parse("h1").unwrap();
    for h1 in document.select(&h1_selector) {
        println!("H1: {}", h1.text().collect::<Vec<_>>().join(" "));
    }

    // <p>
    let p_selector = scraper::Selector::parse("p").unwrap();
    for p in document.select(&p_selector) {
        println!("Paragraph: {}", p.text().collect::<Vec<_>>().join(" "));
    }

    // <a> links
    let a_selector = scraper::Selector::parse("a").unwrap();
    for a in document.select(&a_selector) {
        let href = a.value().attr("href").unwrap_or("");
        let text = a.text().collect::<Vec<_>>().join(" ");
        println!("Link: {} ({})", text, href);

        // Ad/tracker check (in href or text)
        let haystack = format!("{} {}", href.to_lowercase(), text.to_lowercase());
        if ad_keywords.iter().any(|k| haystack.contains(k)) {
            ad_count += 1;
        }
    }

    // <img> tags (NEW!)
    let img_selector = scraper::Selector::parse("img").unwrap();
    for img in document.select(&img_selector) {
        let src = img.value().attr("src").unwrap_or("");
        let alt = img.value().attr("alt").unwrap_or("");
        if !alt.is_empty() {
            println!("Image: alt=\"{}\" src={}", alt, src);
        } else {
            println!("Image: src={}", src);
        }

        // You could also flag suspicious tracking pixels/images here if you want!
    }

    // <script> src detection
    let script_selector = scraper::Selector::parse("script").unwrap();
    for script in document.select(&script_selector) {
        if let Some(src) = script.value().attr("src") {
            println!("Script: {}", src);
            if ad_keywords.iter().any(|k| src.to_lowercase().contains(k)) {
                ad_count += 1;
            }
        }
    }

    ad_count
}

