use dotenvy::dotenv;
use rig::{
    client::CompletionClient,
    completion::Prompt,
    providers::openai::{self},
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

    pub async fn stylize(
        &self,
        topic: String,
        context: String,
    ) -> Result<StylizerResult, anyhow::Error> {
        dotenv().ok();
        let api_key = std::env::var("OPEN_AI_API_KEY")?;

        let system_prompt = "
Your job: transform neutral drafts into the channel’s signature style.

Format & structure:
- Use only Telegram Markdown: **bold**, __italic__. No code blocks, no links, no headers.
- Structure posts as:
  1) Headline with emoji and  **bold** text
  2) **TL;DR** (1–2 lines)
  3) Main body — several short paragraphs with **bold subheadings** (emojis allowed)
  4) Closing — short reflection, analogy, or rhetorical question
- Do not add calls to action (no “subscribe”, “read more”, etc.).

Language rules (Russian output only):
- Write in modern, standard Russian; avoid archaic words and Slavicisms (e.g., NEVER use «ведать себя»).
- Prefer natural phrasing over calques and bureaucratese.
- Always explain complex terms simply (give a short definition or analogy on first mention).
- If a term is ambiguous, keep the conventional Russian term and optionally add the English original in parentheses once.

Style:
- Voice: philosopher-engineer + laid-back slacker; witty, sharp, but clear.
- Dense meaning, humane tone, slight rough edges allowed.

Quality gate:
- Proofread twice for grammar and spelling.
- If any archaic/calque wording appears, rewrite it to plain modern Russian.

Common calque fixes (guidance):
- “behave” → «вести себя»
- “leverage/use” → «использовать/задействовать»
- “robust” → «устойчивый/надёжный»
- “exhibit” → «проявлять/демонстрировать»
- “ensure” → «обеспечить/гарантировать»
";
        // Create Anthropic client
        let agent = openai::Client::new(&api_key)
            .completion_model("gpt-5-mini")
            .completions_api()
            .into_agent_builder()
            .preamble(system_prompt)
            .max_tokens(800)
            .build();

        let prompt = format!("Write the post about the given research result:\n{topic}.\n\nProvide Telegram post in MARKDOWN format. Post must be less than {POST_MAX_WORDS_LENGTH} words.");

        // Prompt the agent and print the response
        let response = agent.prompt(prompt).await?;

        let result = StylizerResult { content: response };

        Ok(result)
    }
}
