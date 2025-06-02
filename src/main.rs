use reqwest::blocking::get;
use scraper::{Html, Selector};
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
            parse_and_print(&content);
            println!("--- End ---");
        }
        Err(e) => eprintln!("Error fetching page: {}", e),
    }
}

fn fetch_page(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = get(url)?;
    let body = response.text()?;
    Ok(body)
}

fn parse_and_print(html: &str) {
    let document = Html::parse_document(html);

    // Print <title>
    let title_selector = Selector::parse("title").unwrap();
    if let Some(title) = document.select(&title_selector).next() {
        println!("Title: {}", title.text().collect::<Vec<_>>().join(" "));
    }

    // Print all <h1>
    let h1_selector = Selector::parse("h1").unwrap();
    for h1 in document.select(&h1_selector) {
        println!("H1: {}", h1.text().collect::<Vec<_>>().join(" "));
    }

    // Print all <p>
    let p_selector = Selector::parse("p").unwrap();
    for p in document.select(&p_selector) {
        println!("Paragraph: {}", p.text().collect::<Vec<_>>().join(" "));
    }

    // Print all <a> links
    let a_selector = Selector::parse("a").unwrap();
    for a in document.select(&a_selector) {
        let href = a.value().attr("href").unwrap_or("");
        let text = a.text().collect::<Vec<_>>().join(" ");
        println!("Link: {} ({})", text, href);
    }
}
