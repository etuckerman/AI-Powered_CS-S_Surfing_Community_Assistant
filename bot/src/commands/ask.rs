use log::{info, warn};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

pub fn run(options: &[ResolvedOption]) -> String {
    let Some(ResolvedOption {
        value: ResolvedValue::String(query),
        ..
    }) = options.first()
    else {
        warn!("Query could not be parsed: {options:?}");
        return "Sorry, I couldn't parse your query.".to_string();
    };

    info!("Got query: {query}");
    // TODO: Run this query through the model
    query.to_string()
}

pub fn register() -> CreateCommand {
    info!("Registering command: `ask`");
    // TODO: Localize for other languages?
    CreateCommand::new("ask")
        .description("Ask the bot a surf query")
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "query",
            "The query to ask",
        ))
}
