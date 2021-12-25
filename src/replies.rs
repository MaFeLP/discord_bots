use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use serenity::{
    model::{
        channel::{Channel, Message},
    },
    prelude::*,
};
use crate::config::{Response, CONFIG, Bots};

pub enum ReplyError {
    MutexAcquisition,
    NoReplyFound,
}

pub async fn reply_to(ctx: &Context, new_message: &Message, bot: Bots) -> Result<String, ReplyError> {
    let replies: Vec<Response> = {
        let config_arc = Arc::clone(&CONFIG);
        let mut config_lock = config_arc.lock();
        while config_lock.is_err() {
            dbg!("Could not acquire config lock. Waiting...");
            sleep(Duration::from_millis(5));
            config_lock = config_arc.lock();
        }
        let reply_vector: Vec<Response> = match config_lock {
            Ok(config) => {
                // Copy replies vector
                match bot {
                    Bots::Autokommentator => config.autokommentator.replies.to_vec(),
                    Bots::KaenguruKnecht => config.kaenguru.replies.to_vec(),
                }
            },
            Err(_) => {
                println!("Something went wrong internally...");
                return Err(ReplyError::MutexAcquisition);
            }
        };

        reply_vector
    };

    // Go through all the replies and then check if to reply to this message and with what
    // Get the name in a separate scope to not copy rng into the async part of message sending
    let response_option = {
        let mut out: Option<String> = None;

        for reply in replies {
            // If out has been set, set the reply and continue.
            if out.is_some() {
                break;
            }

            // Check if current key is part of the current message
            for trigger in &reply.trigger {
                if new_message
                    .content
                    .to_lowercase()
                    .contains(&trigger.as_str().unwrap().to_lowercase())
                {
                    // Select random answer from pool
                    let mut rng = rand::thread_rng();
                    let response_idx = rng.gen_range(0..reply.response.len());
                    let response_value: &toml::Value = reply.response.get(response_idx).unwrap();
                    out = Some(String::from(response_value.as_str().unwrap()));

                    break;
                }
            }
        }
        out
    }; // let response

    let response = match response_option {
        None => return Err(ReplyError::NoReplyFound),
        Some(s) => s,
    };

    // Get the channel and only react to private messages and server-messages
    let option_channel = ctx.cache.channel(new_message.channel_id).await;
    match new_message.reply(&ctx, &response).await {
        Ok(msg) => {
            let channel_name = match option_channel {
                Some(Channel::Private(c)) => format!("DM:{}", c.recipient.name),
                Some(Channel::Guild(c)) => c.name,
                Some(c) => format!("ID:{}", c.id()),
                None => "Not a channel".to_string(),
            };
            match bot {
                Bots::Autokommentator => {
                    println!(
                        "[XD]:\tSend message \"{}\" to channel {} ({})",
                        msg.content, channel_name, msg.channel_id
                    );
                },
                Bots::KaenguruKnecht => {
                    println!(
                        "[KG]:\tSend message \"{}\" to channel {} ({})",
                        msg.content, channel_name, msg.channel_id
                    );
                }
            };
        },
        Err(why) => {
            match bot {
                Bots::Autokommentator => println!("[XD]:\tError sending message: {:?}", why),
                Bots::KaenguruKnecht => println!("[KG]:\tError sending message: {:?}", why),
            }
        },
    };

    Ok(response)
}
