use crate::listeners::events::{message, ready};

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::channel::Message,
    model::gateway::Ready,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
         ready::ready(context, ready).await;
    }

    async fn message(&self, context: Context, message: Message) {
        message::message(context, message).await;
    }
}
