use serenity::all::{
    Colour, CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use tracing::{error, info};

use crate::database::list_custom_messages;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    match list_custom_messages().await {
        Ok(messages) => {
            if messages.is_empty() {
                let embed = CreateEmbed::new()
                    .colour(Colour::new(0xFFA500))
                    .title("📝 Messages personnalisés")
                    .description("Aucun message personnalisé trouvé.");

                interaction
                    .create_response(
                        &ctx,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .embed(embed)
                                .ephemeral(true),
                        ),
                    )
                    .await?;
            } else {
                let mut embed = CreateEmbed::new()
                    .colour(Colour::new(0x00FF00))
                    .title("📝 Messages personnalisés");

                for (msg_type, message, user_id) in messages {
                    let display_type = match msg_type.as_str() {
                        "thirty_min" => "30 minutes",
                        "one_hour" => "1 heure",
                        "three_hours" => "3 heures",
                        _ => &msg_type,
                    };

                    // Truncate message if too long
                    let display_message = if message.len() > 1024 {
                        format!("{}...", &message[..1021])
                    } else {
                        message
                    };

                    embed = embed.field(
                        format!("{} (par <@{}>)", display_type, user_id),
                        display_message,
                        false,
                    );
                }

                interaction
                    .create_response(
                        &ctx,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .embed(embed)
                                .ephemeral(true),
                        ),
                    )
                    .await?;
            }
        }
        Err(e) => {
            error!("Failed to list custom messages: {}", e);
            interaction
                .create_response(
                    &ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(
                                "❌ Erreur lors de la récupération des messages personnalisés !",
                            )
                            .ephemeral(true),
                    ),
                )
                .await?;
        }
    }

    info!("User {} listed custom messages", interaction.user.name);

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("list_msg")
        .description("Lister les messages personnalisés pour les rappels de boire de l'eau")
}
