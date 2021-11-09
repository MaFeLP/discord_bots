mod xd;
mod kaenguru;

use std::env;
use tokio::runtime::Runtime;

use serenity::{
    prelude::*,
};

async fn start_xd() {
    let xd_token = env::var("DISCORD_TOKEN_XD").expect("xd_token");
    let mut xd_client = Client::builder(xd_token)
        .event_handler(xd::XDHandler)
        .await
        .expect("[XD]:\tError creating client");

    if let Err(why) = xd_client.start().await {
        println!("[KG]:\tAn error occurred while running the client: {:?}", why);
    }
}

async fn start_kg() {
    let kg_token = env::var("DISCORD_TOKEN_KAENGURU").expect("kg_token");
    let mut kg_client = Client::builder(kg_token)
        .event_handler(kaenguru::KaenguruHandler)
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
