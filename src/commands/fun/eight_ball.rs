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
pub async fn eight_ball(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        let response = MessageBuilder::new()
            .mention(&message.author)
            .push(" Ask a question, idiot!")
            .build();
        message.channel_id.say(&context.http, response).await?;
        Ok(())
    } else {
        let response = MessageBuilder::new()
            .mention(&message.author)
            .push(" The result is: ")
            .push_bold_safe(RESPONSES.choose(&mut rand::thread_rng()).unwrap())
            .build();
        
        message.channel_id.say(&context.http, response).await?;

        Ok(())
    }
}