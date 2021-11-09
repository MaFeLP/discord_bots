use std::env;
use tokio::runtime::Runtime;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, Permissions},
    prelude::*,
//    utils::MessageBuilder,
};

struct XDHandler;
struct KaenguruHandler;

#[async_trait]
impl EventHandler for XDHandler {
    async fn message(&self, _ctx: Context, mut _new_message: Message) {
        if _new_message.author.bot {
            return;
        }

        if _new_message.content.to_lowercase().contains("xd") {
            match _new_message.reply(_ctx, "XDDDDDDD").await {
                Ok(msg) => println!("[XD]:\tSend message \"XDDDDDDD\" to channel {}", msg.channel_id),
                Err(why) => println!("[XD]:\tError sending message: {:?}", why)
            };
        }
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("[XD]:\tLogged in as {}", _data_about_bot.user.name);

        let permissions = Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES | Permissions::EMBED_LINKS;
        match _data_about_bot.user.invite_url(&_ctx, permissions).await {
            Ok(url) => {
                println!("[XD]:\tBot invitation url is: {}", url);
                return;
            }
            Err(why) => {
                println!("[XD:]\tError getting invite url: {}", why);
                return;
            }
        };
    }
}

#[async_trait]
impl EventHandler for KaenguruHandler {
    async fn message(&self, _ctx: Context, mut _new_message: Message) {
        if _new_message.author.bot {
            return;
        }

        if _new_message.content.to_lowercase().contains("€") ||
            _new_message.content.to_lowercase().contains("eur") {
            match _new_message
                .channel_id
                .send_message(&_ctx.http, |m| {
                    m.embed(|e| {
                        //e.author();
                        e.description("So viele Euros???");

                        e
                    });

                    m
                }).await {
                    Ok(msg) => println!("[KG]:\tSending money Message to {}", msg.channel_id),
                    Err(why) => println!("[KG]:\tError sending message: {:?}", why),
            };
        }
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("[KG]:\tLogged in as {}", _data_about_bot.user.name);

        let permissions = Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES | Permissions::EMBED_LINKS;
        match _data_about_bot.user.invite_url(&_ctx, permissions).await {
            Ok(url) => println!("[KG]:\tBot invitation url is: {}", url),
            Err(why) => println!("[KG]:\tError getting invite url: {}", why),
        };
    }
}

async fn start_xd() {
    let xd_token = env::var("DISCORD_TOKEN_XD").expect("xd_token");
    let mut xd_client = Client::builder(xd_token)
        .event_handler(XDHandler)
        .await
        .expect("[XD]:\tError creating client");

    if let Err(why) = xd_client.start().await {
        println!("[KG]:\tAn error occurred while running the client: {:?}", why);
    }
}

async fn start_kg() {
    let kg_token = env::var("DISCORD_TOKEN_KAENGURU").expect("kg_token");
    let mut kg_client = Client::builder(kg_token)
        .event_handler(KaenguruHandler)
        .await
        .expect("[KG]:\tError creating client");

    if let Err(why) = kg_client.start().await {
        println!("[KG]:\tAn error occurred while running the client: {:?}", why);
    }
}

fn main() {
    println!("[MAIN]:\tStarting \"Känguru Rechenkencht\" and \"XD-Bot\"...");
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        tokio::spawn(async { start_xd().await});
        println!("[ASYN]:\tStarted \"XD-Bot\"!");
        tokio::spawn(async { start_kg().await});
        println!("[ASYN]:\tStarted \"Känguru Rechenknecht\"!");
    });
    println!("[MAIN]:\tStarted two bots.\n[MAIN]:\tThey should appear in you list shortly!");
    loop {}
}
