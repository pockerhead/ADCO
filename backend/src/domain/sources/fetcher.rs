use reqwest;
use adco_shared::source::{HackerNewsResponse, ArxivFeed, ArxivEntry};
use quick_xml::de::from_str;

#[derive(Debug, Clone)]
pub struct HttpFetcher {
    client: reqwest::Client,
}

impl HttpFetcher {
    pub fn new() -> Self {
        HttpFetcher {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch(&self, url: &str) -> Result<String, anyhow::Error> {
        let response = self.client.get(url).send().await?;
        let status = response.status().as_u16();
        if status != 200 {
            return Err(HttpFetchError::StatusCode(status).into());
        }
        let body = response.text().await?;
        if body.contains("You can’t perform that action") {
            return Err(HttpFetchError::BotDetection.into());
        }
        Ok(body)
    }

    pub async fn search_hackernews(&self, query: &str) -> Result<HackerNewsResponse, anyhow::Error> {
        let url = format!("http://hn.algolia.com/api/v1/search?query={}&tags=story", query);
        let response = self.client.get(&url).send().await?;
        let status = response.status().as_u16();
        if status != 200 {
            return Err(HttpFetchError::StatusCode(status).into());
        }
        let body = response.json::<HackerNewsResponse>().await?;
        Ok(body)
    }

    pub async fn search_arxiv(&self, query: &str) -> Result<Vec<ArxivEntry>, anyhow::Error> {
        let url = format!("http://export.arxiv.org/api/query?search_query=all:{}&start=0&max_results=10",
                         query.replace(" ", "+"));
        let response = self.client.get(&url).send().await?;
        let status = response.status().as_u16();
        if status != 200 {
            return Err(HttpFetchError::StatusCode(status).into());
        }
        let body = response.text().await?;
        // Parse Atom XML using quick_xml
        let feed: ArxivFeed = from_str(&body)?;
        Ok(feed.entry)
    }
}

#[derive(Debug)]
enum HttpFetchError {
    StatusCode(u16),
    BotDetection,
}

impl Into<anyhow::Error> for HttpFetchError {
    fn into(self) -> anyhow::Error {
        match self {
            HttpFetchError::StatusCode(code) => anyhow::anyhow!("Status code: {}", code),
            HttpFetchError::BotDetection => anyhow::anyhow!("Bot detection"),
        }
    }
}