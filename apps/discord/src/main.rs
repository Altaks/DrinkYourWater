use std::{env, error::Error, sync::Arc};

use serenity::{
    all::{
        CreateInteractionResponse, CreateInteractionResponseMessage, GuildId, Interaction, Ready,
    },
    async_trait,
    prelude::*,
};
use tokio::spawn;
use tokio_schedule::{Job, every};

use crate::reminder::walk_reminders;

mod buttons;
mod commands;
mod registry;
mod reminder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let token = env::var("DISCORD_BOT_TOKEN")?;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await?;

    let cache_http = Arc::clone(&client.http);
    let every_15_min = every(3)
        .seconds()
        .perform(move || walk_reminders(cache_http.clone()));
    let _handle = spawn(every_15_min);

    if let Err(reason) = client.start_autosharded().await {
        println!("Client error while starting : {:?}", reason);
    }

    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);

        let Ok(Ok(guild_id)) = env::var("DISCORD_GUILD_ID").map(|it| it.parse::<u64>()) else {
            eprintln!("No discord guild ID has been found / it couldn't be parsed as u64 :(");
            return;
        };

        let guild_id = GuildId::new(guild_id);

        let commands = guild_id
            .set_commands(&ctx.http, vec![commands::register::register()])
            .await;

        println!("I now have the following guild slash commands : {commands:?}");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "register" => {
                    commands::register::run(&ctx, &command).await.unwrap();
                    None
                }
                _ => Some("This command is not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);

                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command : {why:#?}")
                }
            }
        }
    }
}
