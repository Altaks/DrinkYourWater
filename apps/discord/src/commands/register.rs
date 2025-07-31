use std::time::Duration;

use serenity::all::{
    Colour, CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage,
    ResolvedOption, ResolvedValue, prelude::*,
};

use crate::{
    buttons::register_buttons::{
        get_1h_button, get_3h_button, get_30min_button, resolve_user_choice,
    },
    registry::{insert_new_user_to_remind, lookup_active_reminders_count},
};

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

    let info_embed = CreateEmbed::new()
        .colour(Colour::new(0xFFFFFF))
        .title("Quelques informations")
        .field(
            r"L'eau est essentielle à la vie", "Le corps d'un adulte est composé d'eau à plus de **60%**. Tous les jours, nous en perdons une partie par les urines, la transpiration et la respiration (eau éliminée lors de l'expiration).

            Pour maintenir l'équilibre de l'organisme, il est recommandé de compenser ces pertes en buvant **1 à 1,5 litre d'eau par jour**, en dehors de toute limitation de la consommation par son médecin *(ce qui peut être nécessaire en cas d'insuffisance cardiaque, par exemple)*.

            *Pour boire l'eau nécessaire au fonctionnement de votre corps, adoptez les bons réflexes.*", false)
        .field("Une petite soif ?", "Si vous ressentez le besoin de vous désaltérer, cela signifie que vous manquez déjà d'eau. Pour éviter d'être déshydraté(e), buvez sans attendre d'avoir soif. Cette précaution est encore plus importante par temps chaud ou pendant la pratique d'une activité physique, situations où l'on perd davantage d'eau.", false)
        .field("Mais j'aime quand y'a du goût ...", "Si vous avez du mal à boire de l'eau nature, pensez aux tisanes et aux infusions, à consommer sans sucre. N'hésitez pas à varier le type de plante en fonction de vos envies. Par ailleurs, vous pouvez aromatiser l'eau de boisson avec des rondelles de citron ou des feuilles de menthe. Enfin, essayez de consommer des soupes et potages, riches en eau.", false)
        .field("Personnes fragiles", " Si vous avez plus de **55 ans**, veillez à vous **hydrater suffisamment** (la sensation de soif peut diminuer avec l'âge). Proposez souvent de l'eau aussi aux **enfants**, qui ne pensent pas toujours à boire régulièrement.", false)
        .url("https://www.ameli.fr/assure/sante/themes/alimentation-adulte/alimentation-adulte-types-aliments/eau");

    let choice_embed = CreateEmbed::new()
        .colour(Colour::new(0x0E87CC))
        .title("Choisissez à quelle fréquence vous souhaitez boire de l'eau : ")
        .field(
            "A votre rythme",
            "Adaptez votre alimentation à vos envies et votre rythme",
            false,
        );

    let data = CreateInteractionResponseMessage::new()
        .add_embed(info_embed)
        .ephemeral(true);

    let builder = CreateInteractionResponse::Message(data);
    if let Err(why) = interaction.create_response(&ctx.http, builder).await {
        println!("Cannot respond to slash command: {why}");
    }

    // TODO : MAKE IT A FOLLOW UP

    let msg = interaction
        .create_followup(
            &ctx,
            CreateInteractionResponseFollowup::new()
                .embed(choice_embed)
                .button(get_30min_button())
                .button(get_1h_button())
                .button(get_3h_button())
                .ephemeral(true),
        )
        .await?;

    let interaction = match msg
        .await_component_interaction(&ctx.shard)
        .timeout(Duration::from_secs(60 * 3))
        .await
    {
        Some(x) => x,
        None => {
            msg.reply_ping(&ctx, "Vous n'avez pas indiqué de valeur pendant 3 minutes, vous devrez vous réenregistrer la prochaine fois").await?;
            return Ok(());
        }
    };

    let choice = match &interaction.data.kind {
        serenity::all::ComponentInteractionDataKind::Button => &interaction.data.custom_id,
        _ => {
            msg.reply_ping(&ctx, "This interaction is not possible >:(")
                .await?;
            return Ok(());
        }
    };

    let frequency = resolve_user_choice(choice)?;
    insert_new_user_to_remind(target, frequency).await;

    let msg = format!(
        "User {} with id {} has been registered for reminders",
        target.name, target.id
    );
    println!("{msg}");
    println!(
        "There are now {} active reminders !",
        lookup_active_reminders_count().await
    );

    interaction
        .create_response(
            &ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(format!(
                        "Vous serez rappelé(e) toutes les {}",
                        frequency.to_string()
                    ))
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("register")
        .description("Register yourself to be reminded to drink enough water !")
        .add_option(CreateCommandOption::new(CommandOptionType::User, "target", "A user that needs to be reminded, if not specified, this will consider you as the user to remind").required(false))
}
