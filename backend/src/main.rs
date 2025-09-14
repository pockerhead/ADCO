mod domain;

use domain::sources::models::Source;
use tokio;
use sqlx::PgPool;
use dotenvy::dotenv;
use domain::infra::postgres::posts_repo::PostsRepositoryImpl;
use domain::posts::repository::PostRepository;
use domain::posts::models::Post;
use uuid::Uuid;
use domain::infra::postgres::sources_repo::SourcesRepositoryImpl;
use domain::sources::repository::SourceRepository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    println!("Database URL: {:?}", database_url);
    let pool: PgPool = PgPool::connect(&database_url).await?;
    println!("Pool created");
    test_posts_repo(&pool).await?;
    test_sources_repo(&pool).await?;
    return Ok(())
}

async fn create_post(pool: &PgPool) -> Result<Uuid, anyhow::Error> {
    let posts_repo: PostsRepositoryImpl = PostsRepositoryImpl::new(&pool);
    let post = Post::new("Test".to_string(), "Test".to_string(), "Test".to_string(), "1".to_string());
    let post_id = posts_repo.create_post(&post).await?;
    println!("Post created with id: {:?}", post_id);
    Ok(post_id)
}

async fn get_post(pool: &PgPool, post_id: Uuid) -> Result<Option<Post>, anyhow::Error> {
    let posts_repo: PostsRepositoryImpl = PostsRepositoryImpl::new(&pool);
    let post = posts_repo.get_post_by_id(post_id).await?;
    Ok(post)
}

async fn test_posts_repo(pool: &PgPool) -> Result<(), anyhow::Error> {
    let post_id = create_post(&pool).await?;
    let post = get_post(&pool, post_id).await?;
    println!("Post: {:?}", post);
    Ok(())
}

async fn test_sources_repo(pool: &PgPool) -> Result<(), anyhow::Error> {
    let sources = get_sources().await?;
    println!("Sources: {:?}", sources.len());
    let sources_repo: SourcesRepositoryImpl = SourcesRepositoryImpl::new(&pool);
    for source in sources.iter() {
        let source_id = sources_repo.create_source(source).await?;
        println!("Source created with id: {:?}", source_id);
    }
    Ok(())
}

async fn get_sources() -> Result<Vec<Source>, anyhow::Error> {
    let  fetcher = domain::sources::HttpFetcher::new();
    let parser = domain::sources::HtmlParser::new();
    let response = fetcher.search_hackernews("rust").await?;
    let mut sources = Vec::new();
    for hit in response.hits.iter() {
        if let Some(url) = &hit.url {
            match parser.scrap_source_from_url(url).await {
                Ok(source) => {
                    sources.push(source);
                }
                Err(e) => {
                    println!("Error scraping source from url: {:?}", e);
                }
            }
        }
    }
    Ok(sources)
}