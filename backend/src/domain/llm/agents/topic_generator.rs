use dotenvy::dotenv;
use rig::{
    client::CompletionClient, completion::Prompt, providers::openai
};
use serde::{Deserialize, Serialize};
use rand::seq::IndexedRandom;
use rand::rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicGeneratorResult {
    pub topic: String,
    pub search_query: String,
}

pub struct TopicGenerator {}

impl TopicGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn generate_topic(&self) -> Result<TopicGeneratorResult, anyhow::Error> {
        dotenv().ok();
        let api_key = std::env::var("OPEN_AI_API_KEY")?;
        // Create Anthropic client
        let agent = openai::Client::new(&api_key)
            .completion_model("gpt-5-mini")
            .completions_api()
            .into_agent_builder()
            .preamble("You are a helpful assistant that generates a topic for a blog post.")
            .max_tokens(300)
            .build();

        let today_date_string: &str = &(chrono::Utc::now().format("%Y-%m-%d").to_string());
        let themes: Vec<&str> = vec!["brain-computer interfaces", "physics", "brain biology", "neuroscience", "cognitive science", "AI"];
        let mut rng = rng();
        let random_theme = themes.choose(&mut rng).unwrap();
        println!("========== Random theme: {}", random_theme);
        let prompt: String = format!("Generate a topic for a blog post about the latest trends in {random_theme}. Today's date: {}. Provide result in JSON format with the following fields: topic, search_query WITHOUT ANY OTHER TEXT OR MARKDOWN. Search query should be a single word or phrase that can be used to search for relevant sources.", today_date_string);
        // Prompt the agent and print the response
        let response = agent
            .prompt(prompt.to_string())
            .await?;

        let result = serde_json::from_str::<TopicGeneratorResult>(&response)?;

        Ok(result)
    }
}
