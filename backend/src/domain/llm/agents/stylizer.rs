use dotenvy::dotenv;
use rig::{
    client::CompletionClient, completion::Prompt, providers::{openai::{self}}
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StylizerResult {
    pub content: String,
}

const POST_MAX_WORDS_LENGTH: usize = 600;


pub struct Stylizer {}

impl Stylizer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn stylize(&self, topic: String, context: String) -> Result<StylizerResult, anyhow::Error> {
        dotenv().ok();
        let api_key = std::env::var("OPEN_AI_API_KEY")?;
        // Create Anthropic client
        let agent = openai::Client::new(&api_key)
            .completion_model("gpt-5-mini")
            .completions_api()
            .into_agent_builder()
            .preamble("You are a helpful assistant that stylizes a given research result and stylish context. Final result must be in Russian language.")
            .max_tokens(800)
            .build();


        let style = "You are the writing engine for the Telegram channel \"Async Dev\".  
Your style:  
-Mix deep insights in AI, tech, and science with sharp wit and raw clarity.  
-Write structured, digestible posts (TL;DR vibe, lists, emojis).  
-Balance rigor with playfulness: philosopher-engineer meets laid-back slacker on a couch.  
-Always leave space for imperfection, doubt, and human messiness — no over-polish.  
-Tone: smart friend, slightly reckless, Saitama-like calm indifference under the surface.  
-Goal: Inform, entertain, and provoke thought, not just explain.  
";


        let prompt = format!("Write the post about the given research result:\n{topic}.\nStylish context:\n\n{style}.\n\nProvide Telegram post in MARKDOWN format. Post must be less than {POST_MAX_WORDS_LENGTH} words.");




        // Prompt the agent and print the response
        let response = agent
            .prompt(prompt)
            .await?;

        let result = StylizerResult {
            content: response,
        };

        Ok(result)
    }
}
