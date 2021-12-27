use std::{
    sync::Arc,
};
use rand::Rng;
use serenity::{
    model::{
        channel::{Channel, Message},
    },
    prelude::*,
};
use crate::config::{Response, CONFIG, Bots};

/// The Errors that might be thrown by [reply_to]
pub enum ReplyError {
    /// If the bot could not acquire a lock of the configuration
    MutexAcquisition,
    /// If no reply was found in this message.
    /// Can be ignored in most cases.
    NoReplyFound,
}

/// A function that searches _new\_message_ for replies configured in config.toml.
/// For this it will first acquire the Mutex Lock for the configuration.
///
/// # Arguments
///
/// * `ctx`: The [context](serenity::client::context) in which the bot operates.
/// * `new_message`: The message to filter and react to.
/// * `bot`: The bot which configures which replies are being searched in the config file.
///
/// returns: Result<String, ReplyError>
///
/// # Examples
///
/// ```
/// /// The default struct on which the bot is built
/// pub struct Bot;
///
/// #[async_trait]
/// /// The method that reacts to new messages.
/// /// This method is called by serenity.
/// ///
/// /// # Arguments
/// ///
/// /// * `ctx`: The context in which this message was sent. Contains information about the bot and its cache
/// /// * `new_message`: The message that was sent and to which this bot should react to.
/// impl EventHandler for Bot {
///     async fn message(&self, ctx: Context, mut new_message: Message) {
///         reply_to(&ctx, &new_message, Bots::Autokommentator).await;
///     }
/// }
/// ```
///
pub async fn reply_to(ctx: &Context, new_message: &Message, bot: Bots) -> Result<String, ReplyError> {
    let replies: Vec<Response> = {
        let config_arc = Arc::clone(&CONFIG);
        let config_lock = config_arc.lock();
        let reply_vector: Vec<Response> = match config_lock {
            Ok(config) => {
                // Copy replies vector
                match bot {
                    Bots::Autokommentator => config.autokommentator.replies.to_vec(),
                    Bots::KaenguruKnecht => config.kaenguru.replies.to_vec(),
                }
            },
            Err(why) => {
                eprintln!("Something went wrong internally: {:?}\nMutex is poisoned: {}", why, why);
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

    // We can not unwrap response_option everywhere where response is used, because
    // Option<String> does not implement the Copy trait, but String does. This is,
    // why a String is needed from here on.
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
