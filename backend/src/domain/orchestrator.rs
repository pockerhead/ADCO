use crate::domain::llm::agents::{TopicGenerator, Researcher, Stylizer};
use crate::domain::content::{TextChunker, EmbeddingsRepository, chunker::Chunk};
use crate::domain::sources::models::Source;
use crate::domain::sources::HttpFetcher;
use crate::domain::sources::HtmlParser;
use anyhow::anyhow;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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
        println!("========== Topic:\n{}", generator_result.topic);
        println!("========== Search Query:\n{}", generator_result.search_query);


        let chunks = self.retrieve_relevant_chunks(&generator_result.search_query).await?;
        println!("========== Chunks:\n{}", chunks.len());
        let context = chunks.iter().map(|chunk| chunk.text.clone()).collect::<Vec<String>>().join("\n");
        println!("========== Context:\n{}", context);
        let researcher = Researcher::new();
        let researcher_result = researcher.research(generator_result.topic, context).await?;
        println!("========== Researcher:\n{}", researcher_result.content);

        let stylizer = Stylizer::new();
        let stylizer_result = stylizer.stylize(researcher_result.content, "".to_string()).await?;
        println!("========== Stylizer:\n{}", stylizer_result.content);

        self.save_post_to_file(&stylizer_result.content).await?;

        Ok(())
    }

    async fn retrieve_relevant_chunks(&self, query: &str) -> Result<Vec<Chunk>, anyhow::Error> {

        let embeddings_repository = EmbeddingsRepository::new(self.pg_pool.clone());

        let sources: Vec<Source> = self.get_sources(query).await?;
        println!("Sources: {:?}", sources.len());
        for source in sources.iter() {
            let chunker = TextChunker::new(400, 100);
            let id = source.id.clone().unwrap_or_default().to_string();
            let chunks = chunker.chunk_text(&source.raw_text, id);
            println!("Chunks: {:?}", chunks.len());
            for chunk in chunks.iter() {
            }
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
        let mut urls_to_scrape = Vec::new();

        // Collect URLs from HackerNews
        match fetcher.search_hackernews(query).await {
            Ok(hn_response) => {
                for hit in hn_response.hits.iter().take(5) {
                    if let Some(url) = &hit.url {
                        urls_to_scrape.push(url.clone());
                    }
                }
            },
            Err(e) => {
                println!("Error fetching from HackerNews: {:?}", e);
            }
        }

        // Collect URLs from arXiv (use PDF links)
        match fetcher.search_arxiv(query).await {
            Ok(arxiv_entries) => {
                for entry in arxiv_entries.iter().take(5) {
                    // Convert arXiv id to PDF URL
                    // e.g. "http://arxiv.org/abs/2310.00266v1" -> "http://arxiv.org/pdf/2310.00266v1.pdf"
                    let pdf_url = entry.id.replace("/abs/", "/pdf/") + ".pdf";
                    urls_to_scrape.push(pdf_url);
                }
            },
            Err(e) => {
                println!("Error fetching from arXiv: {:?}", e);
            }
        }
        // Now scrape all collected URLs
        for url in urls_to_scrape.iter() {
            match parser.scrap_source_from_url(url).await {
                Ok(source) => {
                    sources.push(source);
                },
                Err(e) => {
                    println!("Error scraping source from url {}: {:?}", url, e);
                }
            }
        }

        Ok(sources)
    }

    async fn save_post_to_file(&self, post: &str) -> Result<(), anyhow::Error> {
        let file_name = format!("post_{}.md", chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string());
        let file_path = format!("posts/{}", file_name);
        if !Path::new("posts").exists() {
            std::fs::create_dir("posts")?;
        }
        let mut file = File::create(file_path)?;
        file.write_all(post.as_bytes())?;
        Ok(())
    }
}
