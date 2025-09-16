use crate::domain::content::chunker::Chunk;
use rig::{
    embeddings::{EmbeddingsBuilder},
    providers::openai::{self},
    vector_store::{InsertDocuments, VectorSearchRequest, VectorStoreIndex},
};
use rig_postgres::PostgresVectorStore;
use sqlx::PgPool;

pub struct EmbeddingsRepository {
    pg_pool: PgPool,
    model: openai::EmbeddingModel,
}

impl EmbeddingsRepository {
    pub fn new(pg_pool: &PgPool) -> Self {
        let api_key = std::env::var("OPEN_AI_API_KEY").unwrap_or_default();
        let client = openai::Client::new(&api_key);
        let model =
            openai::EmbeddingModel::new(client, openai::embedding::TEXT_EMBEDDING_3_SMALL, 1536);
        Self { pg_pool: pg_pool.clone(), model }
    }

    pub async fn save_chunks(&self, chunks: Vec<Chunk>) -> Result<(), anyhow::Error> {
        let vector_store =
            PostgresVectorStore::with_defaults(self.model.clone(), self.pg_pool.clone());

        // Process chunks one by one to avoid token limit
        for chunk_batch in chunks.chunks(5) {
            let documents = EmbeddingsBuilder::new(self.model.clone())
                .documents(chunk_batch.to_vec())  // process chunks in batches
                .unwrap()
                .build()
                .await?;

            vector_store.insert_documents(documents).await?;
        }
        Ok(())
    }

    pub async fn search_chunks(
        &self,
        query: &str,
        top_k: usize,
    ) -> Result<Vec<(f64, String, Chunk)>, anyhow::Error> {
        let vector_store =
            PostgresVectorStore::with_defaults(self.model.clone(), self.pg_pool.clone());
        let results = vector_store
            .top_n::<Chunk>(
                VectorSearchRequest::builder()
                    .query(query)
                    .samples(top_k as u64)
                    .build()?,
            )
            .await?;
        Ok(results)
    }
}
