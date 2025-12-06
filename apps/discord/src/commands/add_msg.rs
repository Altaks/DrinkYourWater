use serenity::all::{
    Colour, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedOption,
    ResolvedValue,
};
use tracing::{error, info};

use crate::database::add_custom_message;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let options = &interaction.data.options();

    let (message_type, message) = {
        let msg_type = options
            .iter()
            .find(|opt| opt.name == "type")
            .and_then(|opt| {
                if let ResolvedValue::String(s) = &opt.value {
                    Some(s)
                } else {
                    None
                }
            });

        let msg = options
            .iter()
            .find(|opt| opt.name == "message")
            .and_then(|opt| {
                if let ResolvedValue::String(s) = &opt.value {
                    Some(s)
                } else {
                    None
                }
            });

        let (msg_type, msg) = (msg_type, msg);

        if msg_type.is_none() || msg.is_none() {
            let error_msg = if msg_type.is_none() {
                "❌ Type de message manquant !"
            } else {
                "❌ Message manquant !"
            };

            interaction
                .create_response(
                    &ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(error_msg)
                            .ephemeral(true),
                    ),
                )
                .await?;
            return Ok(());
        }

        (msg_type.unwrap(), msg.unwrap())
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

    // Get user ID
    let user_id = interaction.user.id.get();

    // Save the custom message
    if let Ok(_) = add_custom_message(user_id, message_type, message).await {
        let format_user_mention = |user_id: u64| format!("<@{}>", user_id);

        let embed = CreateEmbed::new()
            .colour(Colour::new(0x00FF00))
            .title("✅ Message personnalisé ajouté !")
            .field("Type", message_type.to_string(), true)
            .field("Message", message.to_string(), false)
            .field("Ajouté par", format_user_mention(user_id), true);

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
        error!("Failed to add custom message");
        interaction
            .create_response(
                &ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("❌ Erreur lors de l'ajout du message personnalisé !")
                        .ephemeral(true),
                ),
            )
            .await?;
    }

    info!(
        "User {} added custom message for type '{}'",
        interaction.user.name, message_type
    );

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("add_msg")
        .description("Ajouter un message personnalisé pour les rappels de boire de l'eau")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "type",
                "Type de message (thirty_min, one_hour, three_hours)",
            )
            .required(true)
            .add_string_choice("30 minutes", "thirty_min")
            .add_string_choice("1 heure", "one_hour")
            .add_string_choice("3 heures", "three_hours"),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "message",
                "Le message personnalisé à afficher",
            )
            .required(true),
        )
}
