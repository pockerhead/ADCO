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
        let prompt: String = format!(
            "Generate an abstract, timeless topic for a popular science blog post about {random_theme}.

            IMPORTANT RULES:
            - Create CONCEPTUAL topics, not news-based or time-specific ones
            - Focus on fundamental principles, mechanisms, and fascinating questions
            - Use intriguing formats like 'How does...', 'Why do...', 'What happens if...', 'The science behind...'
            - Avoid mentioning specific years, months, dates, or 'latest trends'
            - Generate topics that would be interesting in any year
            - Think about eternal questions that make people curious about science

            Examples of GOOD topics:
            - 'How do brain cells decide what to remember and what to forget?'
            - 'Why does quantum entanglement seem to break the rules of reality?'
            - 'What happens to consciousness when we fall asleep?'
            - 'The hidden mathematics behind viral spread'

            Examples of BAD topics (avoid these):
            - 'Latest AI breakthroughs in 2025'
            - 'Recent discoveries in neuroscience'
            - 'New trends in brain-computer interfaces'

            Provide result in pure JSON format with the following fields:
            topic (an intriguing, timeless question or concept about {random_theme}),
            search_query (abstract keywords for finding relevant scientific sources, no dates).

            Do not include any extra text, explanations, or markdown."
        );        // Prompt the agent and print the response
        let response = agent
            .prompt(prompt.to_string())
            .await?;

        let result = serde_json::from_str::<TopicGeneratorResult>(&response)?;

        Ok(result)
    }
}
