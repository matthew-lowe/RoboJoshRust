use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use serenity::utils::MessageBuilder;

use rand::seq::SliceRandom;

const RESPONSES: &'static [&str] = &["Absolutely", "Not happening", "Unlikely", "Likely", "Focus harder and ask again"];

#[command]
#[aliases("8ball", "eightball", "8_ball")]
pub async fn eight_ball(context: &Context, message: &Message, args: Args) -> CommandResult {
    let response = if args.is_empty() {
        MessageBuilder::new()
            .mention(&message.author)
            .push(" Ask a question idiot!")
            .build()
    } else {
        let chance = RESPONSES.choose(&mut rand::thread_rng()).unwrap();

        MessageBuilder::new()
            .mention(&message.author)
            .push(" The magic 8-ball responds with ")
            .push_bold_safe(chance)
            .build()
    };

    message.channel_id.say(&context.http, response).await?;

    Ok(())
}