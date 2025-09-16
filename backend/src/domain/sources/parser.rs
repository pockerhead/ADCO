use super::fetcher::HttpFetcher;
use anyhow::Result;
use headless_chrome::{Browser, LaunchOptions};
use readability::extractor;
use reqwest::Url;
use scraper::{Html, Selector};
use super::models::{Source, SourceType};
use pdf_extract;
use std::io::Cursor;
use tracing::info;

enum ContentType {
    StaticPage { document: Html },
    DynamicPage,
    PdfFile { url: String, title: Option<String> },
}

pub struct HtmlParser;

impl HtmlParser {
    pub fn new() -> Self {
        HtmlParser {}
    }

    async fn parse_url(&self, url: &str) -> Result<String, anyhow::Error> {
        let fetcher = HttpFetcher::new();
        fetcher.fetch(url).await
    }

    async fn fetch_pdf_bytes(&self, url: &str) -> Result<Vec<u8>, anyhow::Error> {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;
        if response.status() != 200 {
            return Err(anyhow::anyhow!("Failed to fetch PDF: {}", response.status()));
        }
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    pub async fn scrap_source_from_url(&self, url: &str, title: Option<&str>) -> Result<Source, anyhow::Error> {
        match self.detect_content_type(url, title).await? {
            ContentType::StaticPage { document } => {
                info!("Static page detected");
                let html_or_text = document.html();
                let document = extractor::extract(&mut html_or_text.as_bytes(), &Url::parse(url)?)?;
                let text = document.text;
                return Ok(Source::new(url.to_string(), title.unwrap_or(&document.title).to_string(), SourceType::WebPage, text));
            }
            ContentType::DynamicPage => {
                // Placeholder for dynamic content handling
                // In a real implementation, you might use a headless browser here
                info!("Dynamic page detected");
                match self.crawl_text_from_url(url).await {
                    Ok(text) => {
                        let html_or_text = text;
                        let document = extractor::extract(&mut html_or_text.as_bytes(), &Url::parse(url)?)?;
                        let text = document.text;
                        info!("document title: {}", document.title);
                        info!("document text length: {}", text.len());
                        return Ok(Source::new(url.to_string(), title.unwrap_or(&document.title).to_string(), SourceType::WebPage, text));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            ContentType::PdfFile { url, title } => {
                return self.parse_pdf_from_url(&url, title.as_deref()).await;
            }
        }
    }

    async fn parse_pdf_from_url(&self, url: &str, title: Option<&str>) -> Result<Source, anyhow::Error> {
        info!("PDF detected, parsing...");
        let pdf_bytes = self.fetch_pdf_bytes(url).await?;
        let cursor = Cursor::new(pdf_bytes);
        let text = pdf_extract::extract_text_from_mem(cursor.get_ref())?;

        // Extract title from URL or use filename
        let pdf_title = url.split('/').last()
            .unwrap_or("PDF Document")
            .replace(".pdf", "");

        info!("PDF parsed, text length: {}", text.len());
        Ok(Source::new(url.to_string(), title.unwrap_or(&pdf_title).to_string(), SourceType::WebPage, text))
    }

    // 1. Подсчет символов текста - если в <body> меньше 500 символов → DynamicPage
    // 2. Поиск SPA-признаков - ищем <div id="root">, <div id="app">, много <script> тагов
    // 3. Анализ URL - некоторые паттерны сразу подсказывают тип
    async fn detect_content_type(&self, url: &str, title: Option<&str>) -> Result<ContentType, anyhow::Error> {
        if url.ends_with(".pdf") {
            info!("PDF detected with title {}, parsing...", title.unwrap_or("PDF Document"));
            return Ok(ContentType::PdfFile { url: url.to_string(), title: title.map(|s| s.to_string()) });
        }

        let html_content = self.parse_url(url).await?;
        let document = Html::parse_document(&html_content);
        let body_selector = Selector::parse("body").unwrap();
        let script_selector = Selector::parse("script").unwrap();
        let div_selector = Selector::parse("div").unwrap();

        // Check body length
        if let Some(body) = document.select(&body_selector).next() {
            let body_text = body.text().collect::<Vec<_>>().join(" ");
            info!("Body len: {}", body_text.len());
            if body_text.len() < 1000 {
                return Ok(ContentType::DynamicPage);
            }
        }

        // Check for SPA indicators
        let script_count = document.select(&script_selector).count();
        let div_ids: Vec<_> = document
            .select(&div_selector)
            .filter_map(|d| d.value().attr("id"))
            .collect();

        info!("Scripts len: {}", script_count);
        if script_count > 15 || div_ids.contains(&"root") || div_ids.contains(&"app") {
            return Ok(ContentType::DynamicPage);
        }

        Ok(ContentType::StaticPage { document })
    }

    async fn crawl_text_from_url(
        &self,
        url: &str,
    ) -> Result<String, anyhow::Error> {
        let browser = Browser::new(
            LaunchOptions::default_builder()
                .headless(true)
                .build()
                .unwrap(),
        )?;
        let tab = browser.new_tab()?;
        tab.navigate_to(url)?;

        // Ждем загрузки JavaScript
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Получаем HTML после рендеринга
        let html_content = tab.get_content()?;

        // Парсим HTML для извлечения текста
        let document = Html::parse_document(&html_content);
        Ok(document.html())
    }
}
