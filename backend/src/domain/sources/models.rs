//   CREATE TABLE sources (
//     id SERIAL PRIMARY KEY,
//     url TEXT,
//     title TEXT,
//     source_type TEXT,
//     fetched_at TIMESTAMPTZ,
//     raw_text TEXT
//   );

use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: Uuid,
    pub url: String,
    pub title: String,
    pub source_type: SourceType,
    pub fetched_at: Option<DateTime<Utc>>,
    pub raw_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SourceType {
    RSS,
    WebPage,
    API,
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
