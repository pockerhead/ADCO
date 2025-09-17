use dotenvy::dotenv;
use rand::rng;
use rand::seq::IndexedRandom;
use rig::{client::CompletionClient, completion::Prompt, providers::openai};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use tracing::info;
use crate::appstate::APP_STATE;

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicGeneratorResult {
    pub topic: String,
    pub full_search_query: String,
    pub short_search_query: String,
}

pub struct TopicGenerator {}

impl TopicGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn generate_topic(&self) -> Result<TopicGeneratorResult, anyhow::Error> {
        dotenv().ok();
        let api_key = std::env::var("ADCO_OPEN_AI_API_KEY")?;
        // Create Anthropic client
        let agent = openai::Client::new(&api_key)
            .completion_model("gpt-5-mini")
            .completions_api()
            .into_agent_builder()
            .preamble("
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
            topic (an intriguing, timeless question or concept about given theme),
            full_search_query (abstract keywords for finding relevant scientific sources, no dates),
            short_search_query (just one keyword or sentence, no dates).

            Do not include any extra text, explanations, or markdown.")
            .max_tokens(300)
            .build();
        let mut avoiding_topics: Vec<String> = Vec::new();
        info!("Interactive mode: {}", APP_STATE.is_interactive_mode);
        let mut result: TopicGeneratorResult;
        loop {
            let mut rng = rng();
            let random_theme = TOPIC_THEMES.choose(&mut rng).unwrap();
            info!("========== Random theme: {}", random_theme);
            if APP_STATE.is_interactive_mode {
                info!("Continue or generate new random theme? (y/N): ");
                if !self.confirm_continue().await {
                    info!("Generating new random theme...");
                    continue;
                }
            }
             // Prompt the agent and print the response
            loop {
                let avoiding_topics_str = {
                    if avoiding_topics.is_empty() {
                        "empty list".to_string()
                    } else {
                        avoiding_topics.join(", ")
                    }
                };
                let prompt: String = format!(
                    "Generate an abstract, timeless topic for a popular science blog post about {random_theme}.
                    Avoid topics that are already in the list of avoiding topics: {avoiding_topics_str}.
                    If the topic is already in the list of avoiding topics, generate a new one.
                    ",
                    );

                let response = agent.prompt(prompt.to_string()).await?;

                result = serde_json::from_str::<TopicGeneratorResult>(&response)?;
                info!("========== Result: {}", result.topic);
                info!("========== Full search query: {}", result.full_search_query);
                info!(
                    "========== Short search query: {}",
                    result.short_search_query
                );
                if APP_STATE.is_interactive_mode {
                    info!("Continue or generate new topic? (y/N): ");
                    if !self.confirm_continue().await {
                        info!("Generating new topic...");
                        avoiding_topics.push(result.topic.clone());
                        continue;
                    }
                }
                break;
            }
            break;
        }
        Ok(result)
    }

    async fn confirm_continue(&self) -> bool {
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
    }
}

const TOPIC_THEMES: &[&str] = &[
    // Мозг и нейротех
    "brain-computer interfaces",
    "neural decoding",
    "neuroplasticity",
    "synaptic mechanisms",
    "memory consolidation",
    "predictive brain",
    "consciousness",
    "sleep and dreams",
    // Когнитивные науки
    "cognitive biases",
    "decision making",
    "attention and focus",
    "language processing",
    "spatial navigation",
    "temporal perception",
    "social cognition",
    // Нейронауки
    "neurotransmitters",
    "brain networks",
    "neural development",
    "brain rhythms",
    "sensory processing",
    "motor control",
    "neural computation",
    // AI и машинное обучение
    "artificial neural networks",
    "deep learning",
    "reinforcement learning",
    "neural architecture",
    "brain-inspired AI",
    "spiking neural networks",
    // Физика и математика
    "quantum mechanics",
    "particle physics",
    "relativity",
    "thermodynamics",
    "waves and vibrations",
    "chaos theory",
    "symmetry in nature",
    "mathematical patterns",
    // Биология и эволюция
    "evolutionary biology",
    "genetic algorithms",
    "protein folding",
    "cellular automata",
    "ecosystem dynamics",
    "biomechanics",
    "molecular biology",
    // Космос и астрофизика
    "black holes",
    "dark matter",
    "exoplanets",
    "stellar evolution",
    "cosmic rays",
    "gravitational waves",
    "space-time",
    "astrobiology",
    "cosmic inflation",
    // Инженерия и технологии
    "robotics",
    "smart materials",
    "nanotechnology",
    "renewable energy",
    "architecture and design",
    "aerodynamics",
    "sensors and automation",
    "biomimetics",
];
