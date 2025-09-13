mod domain;
use domain::posts::Post;
use domain::sources;
use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, ADCO!");

    // Example usage of the domain module
    // (This is just a placeholder; actual implementation would go here)
    let post = Post::new(
        "My First Post".to_string(),
        "This is a draft.".to_string(),
        "This is the full post text.".to_string(),
        1,
    );
    println!("Created post: {:?}", post);

    let fetcher = sources::HttpFetcher::new();
    match fetcher.search_hackernews("rust").await {
        Ok(response) => println!("HackerNews response: {:?}", response),
        Err(e) => eprintln!("Error fetching HackerNews: {}", e),
    }
}