use crate::appstate;
use crate::domain::{
    content::{chunker::Chunk, EmbeddingsRepository, TextChunker},
    infra::postgres::{posts_repo::PostsRepositoryImpl, sources_repo::SourcesRepositoryImpl},
    llm::agents::{QAController, Researcher, Stylizer, TopicGenerator, TopicGeneratorResult},
    posts::{Post, PostRepository},
    publishing::Publisher,
    sources::{models::Source, HtmlParser, HttpFetcher, SourceRepository},
};
use std::path::Path;
use std::{
    fs::File,
    io::{self, Write},
};
use tracing::{error, info};

pub struct Orchestrator {}

impl Orchestrator {
    pub async fn orchestrate(&self) -> Result<(), anyhow::Error> {
        let topic_generator = TopicGenerator::new();
        let generator_result = topic_generator.generate_topic().await?;
        let topic = generator_result.topic.clone();
        info!("Generated topic: {}", generator_result.topic);
        info!("Search query: {}", generator_result.full_search_query);
        let now = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let chunks = self.retrieve_relevant_chunks(&generator_result).await?;
        info!("Retrieved {} relevant chunks", chunks.len());
        let context = chunks
            .iter()
            .map(|chunk| {
                format!(
                    "Source: {}\nURL: {}\n\n{}",
                    chunk.source_title, chunk.source_url, chunk.text
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        let stylizer_context = chunks
            .iter()
            .map(|chunk| {
                format!(
                    "Source: {}\nURL: {}\n\n",
                    chunk.source_title, chunk.source_url
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        info!("Context length: {} characters", context.len());
        let researcher = Researcher::new();
        let researcher_result = researcher
            .research(generator_result.topic, context.clone())
            .await?;
        info!(
            "Research completed, content length: {} characters",
            researcher_result.content.len()
        );
        let research_full_text = format!(
            "CONTEXT:\n{}\n\nRESEARCH:\n{}",
            context, researcher_result.content
        );
        self.save_document_to_file(
            &research_full_text.clone(),
            format!("posts/{}", now).as_str(),
            "research",
        )
        .await?;
        info!("Research saved to file");
        let stylizer = Stylizer::new();
        let stylizer_result = stylizer
            .stylize(researcher_result.content, stylizer_context)
            .await?;
        info!(
            "Post styling completed, content length: {} characters",
            stylizer_result.content.len()
        );

        self.save_document_to_file(
            &stylizer_result.content.clone(),
            format!("posts/{}", now).as_str(),
            "stylized_post",
        )
        .await?;
        info!("Stylized post saved to file");
        let qa_controller = QAController::new();
        let qa_controller_result = qa_controller.qa(stylizer_result.content).await?;
        info!(
            "QA controller completed, content length: {} characters",
            qa_controller_result.content.len()
        );
        let publisher_preamble =
            format!("Powered by ADCO (https://github.com/pockerhead/ADCO)\n\n");
        let post_text = format!("{}", publisher_preamble + &qa_controller_result.content);
        let pg_pool = appstate::APP_STATE.get_pg_pool().await;
        let post_repo = PostsRepositoryImpl::new(&pg_pool);
        let post = Post::new(topic, post_text.clone(), post_text.clone(), "".to_string());
        post_repo.create_post(&post).await?;
        self.save_document_to_file(
            &post_text.clone(),
            format!("posts/{}", now).as_str(),
            "final_post",
        )
        .await?;
        info!("Post saved to file");
        if self.confirm_publish().await {
            let publisher = Publisher::from_env()?;
            publisher.publish(&post_text.clone()).await?;
            info!("Post published to Telegram");
        } else {
            info!("Post not published to Telegram");
        }
        Ok(())
    }

    async fn retrieve_relevant_chunks(
        &self,
        generator_result: &TopicGeneratorResult,
    ) -> Result<Vec<Chunk>, anyhow::Error> {
        let pg_pool = appstate::APP_STATE.get_pg_pool().await;
        let embeddings_repository = EmbeddingsRepository::new(&pg_pool);

        let mut sources: Vec<Source> = self.get_sources(generator_result).await?;
        info!("Found {} sources", sources.len());
        for source in sources.iter_mut() {
            let source_repo = SourcesRepositoryImpl::new(&pg_pool);
            match source_repo.create_source(&source).await {
                Ok(source_id) => {
                    source.id = Some(source_id.clone());
                }
                Err(e) => {
                    error!("Error creating source: {:?}", e);
                }
            }
            let chunker = TextChunker::new(1000, 250);
            let chunks = chunker.chunk_text_from_source(&source);
            info!("Created {} chunks for source", chunks.len());
            embeddings_repository.save_chunks(chunks).await?;
        }
        let results = embeddings_repository
            .search_chunks(&generator_result.full_search_query.clone(), 10)
            .await?;
        let mut chunks: Vec<Chunk> = Vec::new();
        for (_, _, chunk) in results {
            chunks.push(chunk);
        }
        Ok(chunks)
    }

    async fn confirm_publish(&self) -> bool {
        print!("Publish post? (y/N): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
    }

    async fn get_sources(
        &self,
        generator_result: &TopicGeneratorResult,
    ) -> Result<Vec<Source>, anyhow::Error> {
        let fetcher = HttpFetcher::new();
        let parser = HtmlParser::new();
        let mut sources = Vec::new();
        let mut urls_to_scrape: Vec<(String, Option<String>)> = Vec::new();

        // Collect URLs from HackerNews
        match fetcher
            .search_hackernews(&generator_result.short_search_query)
            .await
        {
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
        info!("Collected {} URLs from HackerNews", urls_to_scrape.len());
        // Collect URLs from arXiv (use PDF links)
        match fetcher
            .search_arxiv(&generator_result.full_search_query)
            .await
        {
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
        info!("Collected {} URLs to scrape", urls_to_scrape.len());
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
        info!("Scraped {} sources", sources.len());
        Ok(sources)
    }

    async fn save_document_to_file(
        &self,
        document: &str,
        path: &str,
        document_name: &str,
    ) -> Result<(), anyhow::Error> {
        let file_name = format!("{}.md", document_name);
        let file_path = format!("{}/{}", path, file_name);
        if !Path::new(path).exists() {
            std::fs::create_dir(path)?;
        }
        let mut file = File::create(file_path)?;
        file.write_all(document.as_bytes())?;
        Ok(())
    }
}
