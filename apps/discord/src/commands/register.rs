use std::fmt::format;

use serenity::all::{
    CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage, Interaction, ResolvedOption,
    ResolvedValue, User, prelude::*,
};

use crate::registry::{insert_new_user_to_remind, lookup_active_reminders_count};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let options = &interaction.data.options();

    let target = if let Some(ResolvedOption {
        value: ResolvedValue::User(target, _),
        ..
    }) = options.first()
    {
        target
    } else {
        &interaction.user
    };

    let msg = format!(
        "User {} with id {} has been registered for reminders",
        target.name, target.id
    );
    insert_new_user_to_remind(target).await;
    println!("{msg}");
    println!(
        "There are now {} active reminders !",
        lookup_active_reminders_count().await
    );

    let data = CreateInteractionResponseMessage::new().content(msg);
    let builder = CreateInteractionResponse::Message(data);
    if let Err(why) = interaction.create_response(&ctx.http, builder).await {
        println!("Cannot respond to slash command: {why}");
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("register")
        .description("Register yourself to be reminded to drink enough water !")
        .add_option(CreateCommandOption::new(CommandOptionType::User, "target", "A user that needs to be reminded, if not specified, this will consider you as the user to remind").required(false))
}
