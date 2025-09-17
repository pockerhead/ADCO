use adco_shared::source::Source;
use uuid::Uuid;
use thiserror::Error;

pub trait SourceRepository {
    async fn create_source(&self, source: &Source) -> Result<Uuid, SourceRepositoryError>;
    async fn get_source_by_id(&self, id: Uuid) -> Result<Option<Source>, SourceRepositoryError>;
    async fn update_source(&self, source: &Source) -> Result<(), SourceRepositoryError>;
    async fn delete_source(&self, id: Uuid) -> Result<(), SourceRepositoryError>;
}

#[derive(Error, Debug)]
pub enum SourceRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Source not found: {id}")]
    NotFound { id: Uuid },
}
