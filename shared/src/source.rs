use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use sqlx::Type;

use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(Type))]
#[cfg_attr(feature = "backend", sqlx(type_name = "source"))]
pub struct Source {
    pub id: Option<Uuid>,
    pub url: String,
    pub title: String,
    pub source_type: SourceType,
    pub fetched_at: Option<DateTime<Utc>>,
    pub raw_text: String,
}

impl Source {
    pub fn new(url: String, title: String, source_type: SourceType, raw_text: String) -> Self {
        Self { id: None, url, title, source_type, fetched_at: Some(chrono::Utc::now()), raw_text }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "backend", derive(Type))]
#[cfg_attr(feature = "backend", sqlx(type_name = "source_type"))]
pub enum SourceType {
    RSS,
    WebPage,
    API,
    PDF,
}

impl From<String> for SourceType {
    fn from(source_type: String) -> Self {
        match source_type.as_str() {
            "rss" => SourceType::RSS,
            "web_page" => SourceType::WebPage,
            "api" => SourceType::API,
            _ => SourceType::RSS,
        }
    }
}

impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HackerNewsResponse {
    pub hits: Vec<HackerNewsStory>,
    pub exhaustive: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HackerNewsStory {
    pub title: String,
    pub author: String,
    pub url: Option<String>,
    pub points: u32,
    pub num_comments: u32,
    pub created_at: String,
    #[serde(rename = "objectID")]
    pub object_id: String,
    #[serde(rename = "_tags")]
    pub tags: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArxivResponse {
    pub feed: ArxivFeed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArxivFeed {
    #[serde(rename = "entry", default)]
    pub entry: Vec<ArxivEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArxivEntry {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub published: String,
    pub updated: String,
}