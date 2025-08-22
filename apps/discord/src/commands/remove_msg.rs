use serenity::all::{
    Colour, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedOption,
    ResolvedValue,
};
use tracing::{error, info};

use crate::database::remove_custom_message;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let options = &interaction.data.options();

    let message_type = if let Some(ResolvedOption {
        value: ResolvedValue::String(msg_type),
        ..
    }) = options.iter().find(|opt| opt.name == "type")
    {
        msg_type
    } else {
        interaction
            .create_response(
                &ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("❌ Type de message manquant !")
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    };

    // Validate message type
    let valid_types = ["thirty_min", "one_hour", "three_hours"];
    if !valid_types.contains(&message_type) {
        interaction
            .create_response(
                &ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!(
                            "❌ Type de message invalide ! Types valides : {}",
                            valid_types.join(", ")
                        ))
                        .ephemeral(true),
                ),
            )
            .await?;
        return Ok(());
    }

    // Remove the custom message
    if let Ok(_) = remove_custom_message(message_type).await {
        let embed = CreateEmbed::new()
            .colour(Colour::new(0xFF0000))
            .title("🗑️ Message personnalisé supprimé !")
            .field("Type", message_type.to_string(), true);

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
        error!("Failed to remove custom message");
        interaction
            .create_response(
                &ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("❌ Erreur lors de la suppression du message personnalisé !")
                        .ephemeral(true),
                ),
            )
            .await?;
    }

    info!(
        "User {} removed custom message for type '{}'",
        interaction.user.name, message_type
    );

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("remove_msg")
        .description("Supprimer un message personnalisé pour les rappels de boire de l'eau")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "type",
                "Type de message à supprimer (thirty_min, one_hour, three_hours)",
            )
            .required(true)
            .add_string_choice("30 minutes", "thirty_min")
            .add_string_choice("1 heure", "one_hour")
            .add_string_choice("3 heures", "three_hours"),
        )
}
