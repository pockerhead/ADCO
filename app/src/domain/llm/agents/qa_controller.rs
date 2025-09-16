use dotenvy::dotenv;
use rig::{
    client::CompletionClient,
    completion::Prompt,
    providers::openai::{self},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QAControllerResult {
    pub content: String,
}

pub struct QAController {}

impl QAController {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn qa(
        &self,
        post_content: String,
    ) -> Result<QAControllerResult, anyhow::Error> {
        dotenv().ok();
        let api_key = std::env::var("OPEN_AI_API_KEY")?;

        let system_prompt = "
You are a final quality control editor for translated Russian content. Fix awkward phrasing and make text natural for Russian speakers.

  FIX:
  - Clunky constructions: 'Пока не пересунул в шкаф, запись живёт' -> 'Запись остается в блокноте, пока её не переместят в архив'
  - Bureaucratic language & English calques
  - Unnatural word order
  - Run-on sentences without breaks
  - Repetitions & tautologies
  - Incorrect grammar or word usage like 'Зачем это важно?' -> 'Почему это важно?'


  PRESERVE:
  - Original meaning & key ideas
  - Channel style (translated Russian content)
  - Paragraph structure
  - All facts & data

  MAKE text:
  - Natural for spoken Russian
  - Easy to comprehend
  - Logically structured
  - Free of 'translation artifacts'

  Return ONLY corrected Russian text, no explanations. 
";
        // Create Anthropic client
        let agent = openai::Client::new(&api_key)
            .completion_model("gpt-5")
            .completions_api()
            .into_agent_builder()
            .preamble(system_prompt)
            .max_tokens(800)
            .build();

        let prompt = format!("Fix the following Russian text if needed:\n{post_content}.\n\nReturn ONLY corrected Russian text, no explanations.");

        // Prompt the agent and print the response
        let response = agent.prompt(prompt).await?;

        let result = QAControllerResult { content: response };

        Ok(result)
    }
}
