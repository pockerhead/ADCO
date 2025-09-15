use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{ParseMode, Recipient},
    Bot,
};

pub struct Publisher {
    bot: Bot,
    channel_username: String,
}

impl Publisher {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        let bot = Bot::from_env();
        let channel_username = std::env::var("TELEGRAM_CHANNEL_USERNAME")?;
        Ok(Self { bot, channel_username: format!("@{}", channel_username) })
    }

    pub async fn publish(&self, post: &str) -> Result<(), anyhow::Error> {
        let message = post.to_string();
        self.bot
            .send_message(Recipient::ChannelUsername(self.channel_username.clone()), message)
            .parse_mode(ParseMode::Html)
            .await?;
        Ok(())
    }
}
