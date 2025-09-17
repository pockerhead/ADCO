use adco_shared::post::Post;
use uuid::Uuid;
use thiserror::Error;

pub trait PostRepository {
    async fn create_post(&self, post: &Post) -> Result<Uuid, PostRepositoryError>;
    async fn get_post_by_id(&self, id: Uuid) -> Result<Option<Post>, PostRepositoryError>;
    async fn update_post(&self, post: &Post) -> Result<(), PostRepositoryError>;
    async fn delete_post(&self, id: Uuid) -> Result<(), PostRepositoryError>;
}

#[derive(Error, Debug)]
pub enum PostRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Post not found: {id}")]
    NotFound { id: Uuid },
}
