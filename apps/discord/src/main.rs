use std::{env, sync::Arc};

use serenity::{
    all::{
        CreateInteractionResponse, CreateInteractionResponseMessage, GuildId, Interaction, Ready,
    },
    async_trait,
    prelude::*,
};
use tokio::spawn;
use tokio_schedule::{Job, every};
use tracing::{error, info};

use crate::{
    database::init_database, logging::init_logging_system, registry::load_users_from_database,
    reminder::walk_reminders,
};

mod buttons;
mod commands;
mod data;
mod database;
mod logging;
mod registry;
mod reminder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging_system();

    dotenvy::dotenv()?;
    info!("Environment variables have been loaded");

    // Initialize database
    init_database().await?;
    info!("Database initialized successfully");

    let token = env::var("DISCORD_BOT_TOKEN")?;
    info!("Discord bot token has been found, not checked tho.");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await?;
    info!("Client has been prepared");

    // Load existing users from database
    load_users_from_database(&client.http).await;
    info!("Loaded existing users from database");

    let cache_http = Arc::clone(&client.http);
    let every_1_min = every(1)
        .minutes()
        .perform(move || walk_reminders(cache_http.clone()));
    let _handle = spawn(every_1_min);
    info!("Walk reminder task has been started");

    info!("Starting client...");
    if let Err(reason) = client.start_autosharded().await {
        error!("Client error while starting : {:?}", reason);
    }

    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected", ready.user.name);

        let Ok(Ok(guild_id)) = env::var("DISCORD_GUILD_ID").map(|it| it.parse::<u64>()) else {
            error!("No discord guild ID has been found / it couldn't be parsed as u64 :(");
            return;
        };

        let guild_id = GuildId::new(guild_id);

        let commands = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::register::register(),
                    commands::unregister::register(),
                    commands::add_msg::register(),
                    commands::list_msg::register(),
                    commands::remove_msg::register(),
                ],
            )
            .await;

        info!("I now have the following guild slash commands : {commands:?}");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "register" => {
                    if let Err(why) = commands::register::run(&ctx, &command).await {
                        error!("The register command failed : {}", why);
                        return;
                    }
                    None
                }
                "unregister" => {
                    if let Err(why) = commands::unregister::run(&ctx, &command).await {
                        error!("The unregister command failed : {}", why);
                        return;
                    }
                    None
                }
                "add_msg" => {
                    if let Err(why) = commands::add_msg::run(&ctx, &command).await {
                        error!("The add_msg command failed : {}", why);
                        return;
                    }
                    None
                }
                "list_msg" => {
                    if let Err(why) = commands::list_msg::run(&ctx, &command).await {
                        error!("The list_msg command failed : {}", why);
                        return;
                    }
                    None
                }
                "remove_msg" => {
                    if let Err(why) = commands::remove_msg::run(&ctx, &command).await {
                        error!("The remove_msg command failed : {}", why);
                        return;
                    }
                    None
                }
                _ => Some("This command is not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);

                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    error!("Cannot respond to slash command : {why:#?}")
                }
            }
        }
    }
}
