use std::{collections::HashSet, env};

mod commands;
use commands::ping::*;

mod listeners;
use listeners::{
    handler::Handler,
};

use serenity::{
    prelude::*,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    http::Http
};

#[group]
#[commands(ping)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("Token not found!");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c 
            .owners(owners)
            .prefix("!rj "))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
