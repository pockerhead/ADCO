use crate::domain::posts::repository::{PostRepository, PostRepositoryError};
use crate::domain::posts::models::{Post, PostStatus};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PostsRepositoryImpl {
    pool: PgPool,
}


impl PostsRepositoryImpl {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

impl PostRepository for PostsRepositoryImpl {


    async fn create_post(&self, post: &Post) -> Result<Uuid, PostRepositoryError> {
        let result = sqlx::query!(
            "INSERT INTO posts (topic, draft, post_text, status, channel_id, scheduled_at, published_at, meta) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
            post.topic,
            post.draft,
            post.post_text,
            post.status.to_string(),
            post.channel_id,
            post.scheduled_at,
            post.published_at,
            post.meta,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result.id)
    }

    async fn update_post(&self, post: &Post) -> Result<(), PostRepositoryError> {
        let _result = sqlx::query!(
            "UPDATE posts SET topic = $1, draft = $2, post_text = $3, status = $4, channel_id = $5, scheduled_at = $6, published_at = $7, meta = $8 WHERE id = $9",
            post.topic,
            post.draft,
            post.post_text,
            post.status.to_string(),
            post.channel_id,
            post.scheduled_at,
            post.published_at,
            post.meta,
            post.id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    
    async fn delete_post(&self, id: Uuid) -> Result<(), PostRepositoryError> {
        let _result = sqlx::query!("DELETE FROM posts WHERE id = $1", id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_post_by_id(&self, id: Uuid) -> Result<Option<Post>, PostRepositoryError> {
        println!("Getting post by id: {:?}", id);
        let result = sqlx::query!("SELECT * FROM posts WHERE id = $1", id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result.map(|r| Post {
            id: Some(r.id),
            topic: r.topic.unwrap_or_default(),
            draft: r.draft.unwrap_or_default(),
            post_text: r.post_text.unwrap_or_default(),
            status: PostStatus::from(r.status.unwrap_or_default()),
            channel_id: r.channel_id.unwrap_or_default(),
            scheduled_at: r.scheduled_at,
            published_at: r.published_at,
            meta: r.meta,
        }))
    }
}