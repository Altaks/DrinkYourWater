use serenity::all::{CommandInteraction, Context, CreateCommand, CreateInteractionResponseMessage};
use tracing::info;

use crate::registry::{lookup_active_reminders_count, remove_user_from_reminders};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let user = &interaction.user;

    // Check if user is registered
    let registered_users = crate::registry::REGISTRED_USERS.read().await;
    if !registered_users.contains_key(user) {
        interaction
            .create_response(
                &ctx,
                serenity::all::CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Vous n'êtes pas enregistré pour recevoir des rappels.")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }
    drop(registered_users);

    // Remove user from reminders
    remove_user_from_reminders(user).await;

    let msg = format!(
        "User {} with id {} has been unregistered from reminders",
        user.name, user.id
    );
    info!("{msg}");
    info!(
        "There are now {} active reminders !",
        lookup_active_reminders_count().await
    );

    interaction
        .create_response(
            &ctx,
            serenity::all::CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Vous avez été désinscrit des rappels de boisson d'eau.")
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}

pub fn register() -> serenity::all::CreateCommand {
    CreateCommand::new("unregister")
        .description("Unregister yourself from water drinking reminders")
}
