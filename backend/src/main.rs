mod domain;
use std::hint;

use domain::posts::Post;
use domain::sources;
use tokio;

use crate::domain::sources::parser;

#[tokio::main]
async fn main() {
    // let fetcher = sources::HttpFetcher::new();
    // match fetcher.search_hackernews("rust").await {
    //     Ok(response) => {
    //         for hit in response.hits.iter() {
    //             if let Some(url) = &hit.url {
    //                 match parser.scrap_text_from_url(url).await {
    //                     Ok(text) => println!("Scraped text from {}", url),
    //                     Err(e) => eprintln!("Error scraping text: {}", e),
    //                 }
    //             } else {
    //                 println!("First hit has no URL");
    //                 return;
    //             }
    //         }
    //     }
    //     Err(e) => eprintln!("Error fetching HackerNews: {}", e),
    // }

    // Заведомо статические сайты
    let static_sites = vec![
        "https://doc.rust-lang.org/book/", // Rust Book - статический
        "https://blog.rust-lang.org/2015/05/15/Rust-1.0.html", // старые посты rust-lang
        "https://fasterthanli.me/articles/a-half-hour-to-learn-rust", // блог amos
    ];
    let parser = sources::HtmlParser::new();

    // Заведомо SPA/динамические
    let spa_sites = vec![
        "https://github.com/rust-lang/rust", // GitHub - React SPA
        "https://discord.com",               // Discord - React
        "https://app.netlify.com",           // админка Netlify
    ];

    // for url in static_sites.iter() {
    //     match parser.scrap_text_from_url(url).await {
    //             Ok(text) => println!("Scraped text from {}: {}", url, text),
    //             Err(e) => eprintln!("Error scraping text: {}: {}", e),
    //         }
    // }

    for url in spa_sites.iter() {
        match parser.scrap_text_from_url(url).await {
            Ok(text) => {
                let preview = if text.len() > 500 {
                    format!("{}...", &text[..500])
                } else {
                    text
                };
                println!("Scraped text from {}: {}", url, preview);
            }
            Err(e) => eprintln!("Error scraping text: {}", e),
        }
    }
}
