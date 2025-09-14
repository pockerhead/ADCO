use dotenvy::dotenv;
use rig::{
    client::CompletionClient, completion::Prompt, providers::{anthropic::{self, CLAUDE_3_5_SONNET}}
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResearcherResult {
    pub content: String,
}

const RESEARCH_MAX_WORDS_LENGTH: usize = 600;

pub struct Researcher {}

impl Researcher {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn research(&self, topic: String, context: String) -> Result<ResearcherResult, anyhow::Error> {
        dotenv().ok();
        let client = anthropic::Client::new(
            &std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set"),
        );

        // Create agent with a single context prompt
        let agent = client
                .agent("claude-sonnet-4-20250514")
                .preamble("You are a helpful assistant that researches a topic with the given context and provides a summary of the research.")
                .max_tokens(800)
                .temperature(0.5)
                .build();

        let prompt = format!("Research the topic:\n\n{topic}.\nContext from internet search:\n\n{context}.\nProvide just plain text without any other text or markdown. Research must be less than {RESEARCH_MAX_WORDS_LENGTH} words.");

        // Prompt the agent and print the response
        let response = agent
            .prompt(prompt)
            .await?;

        let result = ResearcherResult {
            content: response,
        };

        Ok(result)
    }
}
