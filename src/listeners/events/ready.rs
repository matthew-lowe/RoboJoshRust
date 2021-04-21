use serenity::{
    client::Context,
    model::{
        gateway::{Activity, Ready},
        user::OnlineStatus,
    },
};

pub async fn ready(context: Context, ready: Ready) {
    let bot_owner = context.http.get_current_application_info().await.unwrap().owner;

    println!("Successfully logged in as user:");
    println!("Username: {}", ready.user.tag());
    println!("User ID: {}", ready.user.id);
    println!("Owner: {}", bot_owner.tag());

    context.set_presence(Some(Activity::playing("L")), OnlineStatus::Online).await;
}