#[macro_use]
extern crate lazy_static;

mod kaenguru;
mod xd;
mod config;
mod replies;

use std::{
    borrow::Borrow,
    env,
    process::exit,
    thread::sleep,
    time::Duration,
    sync::{Arc},
};
use serenity::prelude::*;
use tokio::{
    runtime::Runtime,
    time::Instant,
};

use crate::config::CONFIG;

/// Function to start a new instance of the autokommentator bot
async fn start_xd() {
    // Get the token from the configuration
    let xd_token = match env::var("DISCORD_TOKEN_XD") {
        Ok(s) => s,
        Err(_) => {
            let config_arc = Arc::clone(&CONFIG);
            let config_lock = config_arc.lock();
            let token = match config_lock {
                Ok(config) => {
                    match &config.autokommentator.token {
                        Some(s) => String::from(s),
                        None => {
                            println!("No token configured for the autokommentator");
                            return
                        }
                    }
                },
                Err(why) => {
                    panic!("Something went wrong internally: {:?}\nMutex is poisoned: {}", why, why);
                }
            };

            dbg!("Token is: {}", &token);
            token
        }
    };
    let mut xd_client = Client::builder(xd_token)
        .event_handler(xd::XDHandler)
        .await
        .expect("[XD]:\tError creating client");

    if let Err(why) = xd_client.start().await {
        println!(
            "[KG]:\tAn error occurred while running the client: {:?}",
            why
        )
    }
}

/// Function to start a new instance of the kaenguru bot
async fn start_kg() {
    // Get the token from the configuration
    let kg_token = match env::var("DISCORD_TOKEN_KAENGURU") {
        Ok(s) => s,
        Err(_) => {
            let config_arc = Arc::clone(&CONFIG);
            let config_lock = config_arc.lock();
            let token = match config_lock {
                Ok(config) => {
                    match &config.kaenguru.token {
                        Some(s) => String::from(s),
                        None => {
                            println!("No token configured for the Kaenguru");
                            return
                        }
                    }
                },
                Err(why) => {
                    panic!("Something went wrong internally: {:?}\nMutex is poisoned: {}", why, why);
                    return
                }
            };

            dbg!("Token is: {}", &token);
            token
        }
    };
    let mut kg_client = Client::builder(kg_token)
        .event_handler(kaenguru::KaenguruHandler)
        .await
        .expect("[KG]:\tError creating client");

    if let Err(why) = kg_client.start().await {
        println!(
            "[KG]:\tAn error occurred while running the client: {:?}",
            why
        )
    }
}

/// Main entry point to this program
fn main() {
    println!("[MAIN]:\tStarting \"Känguru Rechenkencht\" and \"XD-Bot\"...");
    // Use tokio to run multiple bots at the same time
    let start = Instant::now();
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        tokio::spawn(async { start_xd().await });
        println!("[ASYN]:\tStarted \"XD-Bot\"!");
        tokio::spawn(async { start_kg().await });
        println!("[ASYN]:\tStarted \"Känguru Rechenknecht\"!");
    });
    println!("[MAIN]:\tStarted two bots.\n[MAIN]:\tThey should appear in you list shortly!");

    // Set what happens when Ctrl+C or SIGINT is sent to this progess
    ctrlc::set_handler(move || {
        println!("\n[MAIN]: Received Shutdown Signal.");

        // Calculate how long this program ran.
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
    })
    .expect("Error setting Ctrl-C handler");

    // loop infinitely until process is forced to exit
    // We need to loop here, because the program would otherwise exit and disconnect all bots
    loop {
        // Sleep for a short amount of time to lower CPU usage
        // If we would not sleep, one CPU core would be at 100% usage
        sleep(Duration::from_secs(1))
    }
}
