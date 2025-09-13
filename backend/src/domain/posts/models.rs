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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub topic: String,
    pub draft: String,
    pub post_text: String,
    pub status: PostStatus,
    pub channel_id: i32,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PostStatus {
    Draft,
    Scheduled,
    Published,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostMeta {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

impl Post {

    pub fn new(topic: String, draft: String, post_text: String, channel_id: i32) -> Self {
        Post {
            id: Uuid::new_v4(),
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