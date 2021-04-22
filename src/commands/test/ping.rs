use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command
};

#[command]
pub async fn ping(context: &Context, message: &Message, _args: Args) -> CommandResult {
    message.channel_id.say(&context.http, "Pong!").await?;
    Ok(())
}