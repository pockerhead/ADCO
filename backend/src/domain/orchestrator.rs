use crate::domain::{
    content::{chunker::Chunk, EmbeddingsRepository, TextChunker},
    infra::postgres::{posts_repo::PostsRepositoryImpl, sources_repo::SourcesRepositoryImpl},
    llm::agents::{Researcher, Stylizer, TopicGenerator},
    posts::{Post, PostRepository},
    publishing::Publisher,
    sources::{models::Source, HtmlParser, HttpFetcher, SourceRepository},
};
use anyhow::anyhow;
use log::info;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tracing::{error, warn};

pub struct Orchestrator {
    pg_pool: sqlx::pool::Pool<sqlx::postgres::Postgres>,
}

impl Orchestrator {
    pub fn new(pg_pool: sqlx::pool::Pool<sqlx::postgres::Postgres>) -> Self {
        Self { pg_pool }
    }

    pub async fn orchestrate(&self) -> Result<(), anyhow::Error> {
        let topic_generator = TopicGenerator::new();
        let generator_result = topic_generator.generate_topic().await?;
        let topic = generator_result.topic.clone();
        info!("Generated topic: {}", generator_result.topic);
        info!("Search query: {}", generator_result.search_query);

        let chunks = self
            .retrieve_relevant_chunks(&generator_result.search_query)
            .await?;
        info!("Retrieved {} relevant chunks", chunks.len());
        let context = chunks
            .iter()
            .map(|chunk| format!("Source: {}\nTitle: {}\nURL: {}\n\n{}", chunk.source_title, chunk.source_url, chunk.source_url, chunk.text))
            .collect::<Vec<String>>()
            .join("\n");
        info!("Context length: {} characters", context.len());
        let researcher = Researcher::new();
        let researcher_result = researcher.research(generator_result.topic, context).await?;
        info!(
            "Research completed, content length: {} characters",
            researcher_result.content.len()
        );

        let stylizer = Stylizer::new();
        let stylizer_result = stylizer
            .stylize(researcher_result.content, "".to_string())
            .await?;
        info!(
            "Post styling completed, content length: {} characters",
            stylizer_result.content.len()
        );
        let publisher_preamble =
            format!("Powered by ADCO (https://github.com/pockerhead/ADCO)\n\n");
        let post_text = format!("{}", publisher_preamble + &stylizer_result.content);
        let post_repo = PostsRepositoryImpl::new(&self.pg_pool);
        let post = Post::new(topic, post_text.clone(), post_text.clone(), "".to_string());
        post_repo.create_post(&post).await?;
        self.save_post_to_file(&post_text.clone()).await?;
        info!("Post saved to file");
        // let publisher = Publisher::from_env()?;
        // publisher.publish(&post_text.clone()).await?;
        // info!("Post published to Telegram");

        Ok(())
    }

    async fn retrieve_relevant_chunks(&self, query: &str) -> Result<Vec<Chunk>, anyhow::Error> {
        let embeddings_repository = EmbeddingsRepository::new(self.pg_pool.clone());

        let mut sources: Vec<Source> = self.get_sources(query).await?;
        info!("Found {} sources", sources.len());
        for mut source in sources.iter_mut() {
            let source_repo = SourcesRepositoryImpl::new(&self.pg_pool);
            let source_id = source_repo.create_source(&source).await?;
            source.id = Some(source_id.clone());
            let chunker = TextChunker::new(400, 100);
            let chunks = chunker.chunk_text_from_source(&source);
            info!("Created {} chunks for source", chunks.len());
            embeddings_repository.save_chunks(chunks).await?;
        }
        let results = embeddings_repository.search_chunks(query, 10).await?;
        let mut chunks: Vec<Chunk> = Vec::new();
        for (_, _, chunk) in results {
            chunks.push(chunk);
        }
        Ok(chunks)
    }

    async fn get_sources(&self, query: &str) -> Result<Vec<Source>, anyhow::Error> {
        let fetcher = HttpFetcher::new();
        let parser = HtmlParser::new();
        let mut sources = Vec::new();
        let mut urls_to_scrape: Vec<(String, Option<String>)> = Vec::new();

        // Collect URLs from HackerNews
        match fetcher.search_hackernews(query).await {
            Ok(hn_response) => {
                for hit in hn_response.hits.iter().take(5) {
                    if let Some(url) = &hit.url {
                        urls_to_scrape.push((url.clone(), None));
                    }
                }
            }
            Err(e) => {
                error!("Error fetching from HackerNews: {:?}", e);
            }
        }

        // Collect URLs from arXiv (use PDF links)
        match fetcher.search_arxiv(query).await {
            Ok(arxiv_entries) => {
                for entry in arxiv_entries.iter().take(5) {
                    // Convert arXiv id to PDF URL
                    // e.g. "http://arxiv.org/abs/2310.00266v1" -> "http://arxiv.org/pdf/2310.00266v1.pdf"
                    let pdf_url = entry.id.replace("/abs/", "/pdf/") + ".pdf";
                    urls_to_scrape.push((pdf_url, Some(entry.title.clone())));
                }
            }
            Err(e) => {
                error!("Error fetching from arXiv: {:?}", e);
            }
        }
        // Now scrape all collected URLs
        for url in urls_to_scrape.iter() {
            match parser.scrap_source_from_url(&url.0, url.1.as_deref()).await {
                Ok(source) => {
                    sources.push(source);
                }
                Err(e) => {
                    error!("Error scraping source from url {}: {:?}", url.0, e);
                }
            }
        }

        Ok(sources)
    }

    async fn save_post_to_file(&self, post: &str) -> Result<(), anyhow::Error> {
        let file_name = format!(
            "post_{}.md",
            chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string()
        );
        let file_path = format!("posts/{}", file_name);
        if !Path::new("posts").exists() {
            std::fs::create_dir("posts")?;
        }
        let mut file = File::create(file_path)?;
        file.write_all(post.as_bytes())?;
        Ok(())
    }
}
