// CREATE TABLE posts (
//   id UUID PRIMARY KEY,
//   topic TEXT,
//   draft TEXT,
//   post_text TEXT,
//   status TEXT,
//   channel_id TEXT,
//   scheduled_at TIMESTAMPTZ,
//   published_at TIMESTAMPTZ,
//   meta JSONB
// );
use serde::{Serialize, Deserialize};
use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "post")]
pub struct Post {
    pub id: Option<Uuid>,
    pub topic: String,
    pub draft: String,
    pub post_text: String,
    pub status: PostStatus,
    pub channel_id: String,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Type)]
#[sqlx(type_name = "post_status")]
pub enum PostStatus {
    Draft,
    Scheduled,
    Published,
}

impl From<String> for PostStatus {
    fn from(status: String) -> Self {
        match status.as_str() {
            "draft" => PostStatus::Draft,
            "scheduled" => PostStatus::Scheduled,
            "published" => PostStatus::Published,
            _ => PostStatus::Draft,
        }
    }
}

impl std::fmt::Display for PostStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PostStatus::Draft => write!(f, "draft"),
            PostStatus::Scheduled => write!(f, "scheduled"),
            PostStatus::Published => write!(f, "published"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "post_meta")]
pub struct PostMeta {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

impl Post {

    pub fn new(topic: String, draft: String, post_text: String, channel_id: String) -> Self {
        Post {
            id: None,
            topic,
            draft,
            post_text,
            status: PostStatus::Draft,
            channel_id,
            scheduled_at: None,
            published_at: None,
            meta: None,
        }
    }

    pub fn set_meta(&mut self, meta: PostMeta) {
        self.meta = Some(serde_json::to_value(meta).unwrap());
    }

    pub fn get_meta(&self) -> Option<PostMeta> {
        self.meta.as_ref().and_then(|m| serde_json::from_value(m.clone()).ok())
    }
}