use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        Permissions,
    },
    prelude::*,
};
use crate::config::Bots;
use crate::replies::reply_to;

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
    async fn message(&self, ctx: Context, new_message: Message) {
        if new_message.author.bot {
            return;
        }

        reply_to(&ctx, &new_message, Bots::Autokommentator).await;
    }

    /// Method to be called when the bot instance has been logged in.
    ///
    /// # Arguments
    ///
    /// * `_ctx`: The context in which this method was called.
    /// * `_data_about_bot`: Some normal data about the newly created instance
    ///
    /// returns: ()
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        println!("[XD]:\tLogged in as {}", data_about_bot.user.name);

        // Create invite links with only certain permissions
        let permissions =
            Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES | Permissions::EMBED_LINKS;
        match data_about_bot.user.invite_url(&ctx, permissions).await {
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
