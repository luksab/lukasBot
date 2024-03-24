use serenity::{
    client::Context,
    model::application::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
};

use crate::State;

pub async fn handle_joke_config_command(ctx: &Context, command: &ApplicationCommandInteraction) {
    // get command options
    let chance_option = command
        .data
        .options
        .iter()
        .find(|option| option.name == "chance")
        .and_then(|option| option.clone().value.unwrap().as_i64());
    let mut message_text: Vec<String> = Vec::new();
    if let Some(chance) = chance_option {
        // get guild id
        let guild_id = command.guild_id.unwrap().0 as i64;
        let chance = chance as f64 / 100.0;
        // update or insert chance
        sqlx::query!(
            "INSERT INTO JokeConfig (chance, guild_id) VALUES ($1, $2) ON CONFLICT (guild_id) DO UPDATE SET chance = $1",
            chance,
            guild_id
        )
        .execute(&ctx.data.read().await.get::<State>().unwrap().pool)
        .await
        .unwrap();
        message_text.push(format!(
            "The chance for a joke has been set to {}%",
            chance * 100.0
        ));
    }

    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(message_text.join("\n")))
        })
        .await
        .unwrap();
}