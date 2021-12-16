use crate::xd::replies::replies;
use serenity::{
    async_trait,
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        Permissions,
    },
    prelude::*,
};

mod replies;

/// The default struct on which the bot is built
pub struct XDHandler;

#[async_trait]
impl EventHandler for XDHandler {
    /// The method that reacts to new messages.
    /// This method is called by serenity.
    ///
    /// # Arguments
    ///
    /// * `_ctx`: The context in which this message was sent. Contains information about the bot and its cache
    /// * `_new_message`: The message that was sent and to which this bot should react to.
    async fn message(&self, _ctx: Context, mut _new_message: Message) {
        if _new_message.author.bot {
            return;
        }

        // Go through all the replies and then check if to reply to this message and with what
        for (key, value) in replies() {
            // Check if current key is part of the current message
            if _new_message
                .content
                .to_lowercase()
                .contains(&key.to_lowercase())
            {
                // Get the channel and only react to private messages and server-messages
                let option_channel = _ctx.cache.channel(_new_message.channel_id).await;
                match _new_message.reply(&_ctx, value).await {
                    Ok(msg) => {
                        let channel_name = match option_channel {
                            Some(Channel::Private(c)) => format!("DM:{}", c.recipient.name),
                            Some(Channel::Guild(c)) => c.name,
                            Some(c) => format!("ID:{}", c.id()),
                            None => "Not a channel".to_string(),
                        };
                        println!(
                            "[XD]:\tSend message \"{}\" to channel {} ({})",
                            msg.content, channel_name, msg.channel_id
                        )
                    }
                    Err(why) => println!("[XD]:\tError sending message: {:?}", why),
                };

                return;
            }
        }
    }

    /// Method to be called when the bot instance has been logged in.
    ///
    /// # Arguments
    ///
    /// * `_ctx`: The context in which this method was called.
    /// * `_data_about_bot`: Some normal data about the newly created instance
    ///
    /// returns: ()
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("[XD]:\tLogged in as {}", _data_about_bot.user.name);

        // Create invite links with only certain permissions
        let permissions =
            Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES | Permissions::EMBED_LINKS;
        match _data_about_bot.user.invite_url(&_ctx, permissions).await {
            Ok(url) => {
                println!("[XD]:\tBot invitation url is: {}", url);
            }
            Err(why) => {
                println!("[XD:]\tError getting invite url: {}", why);
                return;
            }
        };
    }
}
