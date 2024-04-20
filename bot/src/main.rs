mod commands;
mod handler;

use std::env;

use log::error;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    // Login with a bot token from the environment
    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            error!("`DISCORD_TOKEN` environment variable is not present");
            return;
        }
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = match Client::builder(&token, intents)
        .event_handler(handler::Handler)
        .await
    {
        Ok(client) => client,
        Err(error) => {
            error!("Error creating client: {error:?}");
            return;
        }
    };

    // Start listening for events by starting a single shard
    if let Err(error) = client.start().await {
        error!("Client error: {error:?}");
    }
}
