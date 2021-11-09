use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, mut _new_message: Message) {
        let lower_case = _new_message.content.to_lowercase();
        if lower_case.contains("~ping") {
            if let Err(why) = _new_message.reply(_ctx, "Pong!").await {
                println!("Error sending message: {:?}", why)
            };

            return;
        }
        if lower_case.contains("xd") && !_new_message.author.bot {
            if let Err(why) = _new_message.reply(_ctx, "XDDDDDDD").await {
                println!("Error sending message: {:?}", why)
            };

            return;
        }
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("Logged in as {}", _data_about_bot.user.name)
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
