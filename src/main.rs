mod xd;
mod kaenguru;

use std::panic;
use std::borrow::Borrow;
use std::env;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use tokio::runtime::Runtime;

use serenity::{
    prelude::*,
};
use tokio::time::Instant;

async fn start_xd() {
    panic::set_hook(Box::new(|_| {
        eprintln!("Fatal: Not discord token XD found!\nFatal: Please set the DISCORD_TOKEN_XD environment variable to your discord token!\nFatal: More information can be found here: https://mafelp.github.io/MCDC/installation#get-a-discord-bot-token");
        exit(2);
    }));
    let xd_token = env::var("DISCORD_TOKEN_XD").expect("xd_token");
    let _ = panic::take_hook();
    let mut xd_client = Client::builder(xd_token)
        .event_handler(xd::XDHandler)
        .await
        .expect("[XD]:\tError creating client");

    if let Err(why) = xd_client.start().await {
        println!("[KG]:\tAn error occurred while running the client: {:?}", why);
    }
}

async fn start_kg() {
    panic::set_hook(Box::new(|_| {
        eprintln!("Fatal: Not discord token Känguru found!\nFatal: Please set the DISCORD_TOKEN_KAENGURU environment variable to your discord token!\nFatal: More information can be found here: https://mafelp.github.io/MCDC/installation#get-a-discord-bot-token");
        exit(2);
    }));
    let kg_token = env::var("DISCORD_TOKEN_KAENGURU").expect("kg_token");
    let _ = panic::take_hook();
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
    let start = Instant::now();
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        tokio::spawn(async { start_xd().await});
        println!("[ASYN]:\tStarted \"XD-Bot\"!");
        tokio::spawn(async { start_kg().await});
        println!("[ASYN]:\tStarted \"Känguru Rechenknecht\"!");
    });
    println!("[MAIN]:\tStarted two bots.\n[MAIN]:\tThey should appear in you list shortly!");
    ctrlc::set_handler(move || {
        println!("\n[MAIN]: Received Shutdown Signal.");
        let elapsed_seconds = &start.elapsed().as_secs();
        let hours = elapsed_seconds / 360;
        let mut s_hours = hours.to_string();
        if hours < 10 {
            s_hours = String::from("0");
            s_hours.push_str(hours.to_string().borrow());
        }
        let minutes = (elapsed_seconds - (hours * 360)) / 60;
        let mut s_minutes = minutes.to_string();
        if minutes < 10 {
            s_minutes = String::from("0");
            s_minutes.push_str(minutes.to_string().borrow());
        }
        let seconds = elapsed_seconds - (hours * 360) - (minutes * 60);
        let mut s_seconds = seconds.to_string();
        if seconds < 10 {
            s_seconds = String::from("0");
            s_seconds.push_str(seconds.to_string().borrow());
        }
        println!("[MAIN]: Ran for {}:{}:{}", s_hours, s_minutes, s_seconds);
        exit(0);
    }).expect("Error setting Ctrl-C handler");
    loop {
        sleep(Duration::from_secs(1))
    }
}
