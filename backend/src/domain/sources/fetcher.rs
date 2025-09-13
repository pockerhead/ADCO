use reqwest;
use serde::{Serialize, Deserialize};
use crate::domain::sources::models::{HackerNewsResponse};

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

    pub async fn fetch(&self, url: &str) -> Result<String, reqwest::Error> {
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        Ok(body)
    }

    pub async fn search_hackernews(&self, query: &str) -> Result<HackerNewsResponse, reqwest::Error> {
        let url = format!("http://hn.algolia.com/api/v1/search?query={}&tags=story", query);
        let response = self.client.get(&url).send().await?;
        let body = response.json::<HackerNewsResponse>().await?;
        Ok(body)
    }
}
