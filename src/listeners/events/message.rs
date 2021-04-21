use serenity::{client::Context, model::prelude::Message};

pub async fn message(context: Context, message: Message) {
    if message.content.eq("!rj ping") {
        message.channel_id.say(&context.http, "Poggers!").await.unwrap();
    }
}