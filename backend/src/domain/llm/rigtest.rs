use dotenvy::dotenv;
use rig::{
    client::CompletionClient, completion::Prompt, providers::{anthropic::{self, CLAUDE_3_5_SONNET}, openai::{self}}
};

pub struct RigTest {}

impl RigTest {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn test_openai(&self) -> Result<(), anyhow::Error> {
        dotenv().ok();
        let api_key = std::env::var("OPEN_AI_API_KEY")?;
        // Create Anthropic client
        let agent = openai::Client::new(&api_key)
            .completion_model("gpt-5-mini")
            .completions_api()
            .into_agent_builder()
            .preamble("You are a helpful assistant")
            .build();

        let res = agent.prompt("Hello world!").await.unwrap();

        println!("GPT-4o: {res}");

        Ok(())
    }

    pub async fn test_anthropic(&self) -> Result<(), anyhow::Error> {
        dotenv().ok();
        let client = anthropic::Client::new(
            &std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set"),
        );

        // Create agent with a single context prompt
        let agent = client
                .agent("claude-sonnet-4-20250514")
                .preamble("Be precise and concise.")
                .temperature(0.5)
                .build();

        // Prompt the agent and print the response
        let response = agent
            .prompt("When and where and what type is the next solar eclipse?")
            .await?;

        println!("{response}");

        Ok(())
    }
}
