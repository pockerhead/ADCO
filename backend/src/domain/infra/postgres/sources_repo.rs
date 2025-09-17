use crate::domain::sources::repository::{SourceRepository, SourceRepositoryError};
use adco_shared::source::{Source, SourceType};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SourcesRepositoryImpl {
    pool: PgPool,
}


impl SourcesRepositoryImpl {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

impl SourceRepository for SourcesRepositoryImpl {


    async fn create_source(&self, source: &Source) -> Result<Uuid, SourceRepositoryError> {
        let result = sqlx::query!(
            "INSERT INTO sources (url, title, source_type, fetched_at, raw_text) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            source.url,
            source.title,
            source.source_type.to_string(),
            source.fetched_at,
            source.raw_text,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result.id)
    }

    async fn update_source(&self, source: &Source) -> Result<(), SourceRepositoryError> {
        let _result = sqlx::query!(
            "UPDATE sources SET url = $1, title = $2, source_type = $3, fetched_at = $4, raw_text = $5 WHERE id = $6",
            source.url,
            source.title,
            source.source_type.to_string(),
            source.fetched_at,
            source.raw_text,
            source.id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    
    async fn delete_source(&self, id: Uuid) -> Result<(), SourceRepositoryError> {
        let _result = sqlx::query!("DELETE FROM sources WHERE id = $1", id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_source_by_id(&self, id: Uuid) -> Result<Option<Source>, SourceRepositoryError> {
        println!("Getting source by id: {:?}", id);
        let result = sqlx::query!("SELECT * FROM sources WHERE id = $1", id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(result.map(|r| Source {
            id: Some(r.id),
            url: r.url.unwrap_or_default(),
            title: r.title.unwrap_or_default(),
            source_type: SourceType::from(r.source_type.unwrap_or_default()),
            fetched_at: r.fetched_at,
            raw_text: r.raw_text.unwrap_or_default(),
        }))
    }
}