use crate::commands;

use log::{error, info, warn};
use serenity::all::{
    Command, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction, Ready,
};
use serenity::async_trait;
use serenity::prelude::*;

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::Command(command) = interaction else {
            return;
        };

        // Find the command and store the response
        let name = command.data.name.as_str();
        let content = match name {
            "ask" => Some(commands::ask::run(&command.data.options())),
            _ => {
                warn!("Unknown command: {name}");
                return;
            }
        };

        // Did the command return a response?
        let Some(content) = content else {
            return;
        };

        let data = CreateInteractionResponseMessage::new().content(content);
        let builder = CreateInteractionResponse::Message(data);
        if let Err(why) = command.create_response(&ctx.http, builder).await {
            error!("Cannot respond to slash command: {why}");
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected with name: {}", ready.user.name);

        let ask_command =
            Command::create_global_command(&ctx.http, commands::ask::register()).await;

        info!("Created global slash command: {ask_command:#?}");
    }
}
