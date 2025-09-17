use dotenvy::dotenv;
use rig::{
    client::CompletionClient,
    completion::Prompt,
    providers::anthropic::{self},
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

    pub async fn research(
        &self,
        topic: String,
        context: String,
    ) -> Result<ResearcherResult, anyhow::Error> {
        dotenv().ok();
        let client = anthropic::Client::new(
            &std::env::var("ADCO_ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set"),
        );
        let system_prompt = "
Your job: gather and explain advanced AI/tech/science topics in a **clear scientific-popular style**.  

Rules:  
- Use accessible language — imagine a curious reader with no PhD.  
- Always **unpack complex terms and concepts** (give a short definition or analogy).  
- Structure output: TL;DR → key points → details with explanations and links to sources in the format: source_title - source_url.  
- Avoid slang, jokes, or philosophy. Neutral, precise, digestible.    
- Provide just plain text without any other text or markdown    
";
        // Create agent with a single context prompt
        let agent = client
            .agent("claude-sonnet-4-20250514")
            .preamble(system_prompt)
            .max_tokens(800)
            .temperature(0.5)
            .build();

        let prompt = format!("Research the topic:\n\n{topic}.\nContext from internet search:\n\n{context}.\nResearch must be less than {RESEARCH_MAX_WORDS_LENGTH} words.");

        // Prompt the agent and print the response
        let response = agent.prompt(prompt).await?;

        let result = ResearcherResult { content: response };

        Ok(result)
    }
}
