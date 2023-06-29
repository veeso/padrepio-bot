#[macro_use]
extern crate log;

mod bot;
mod client;

use serenity::prelude::*;

use std::env;

use bot::Bot;

use client::Client as PadrepioClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let chatgpt_apikey =
        env::var("CHATGPT_API_KEY").expect("Expected CHATGPT_API_KEY in the environment");
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");
    let guild: u64 = env::var("SERVER_GUILD")
        .map(|s| s.parse().expect("invalid GUILD"))
        .expect("Expected SERVER_GUILD in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let bot = Bot::new(&chatgpt_apikey, guild)?;

    let mut client = Client::builder(&token, intents)
        .event_handler(bot)
        .await
        .expect("Err creating client");

    client.start().await?;

    Ok(())
}
