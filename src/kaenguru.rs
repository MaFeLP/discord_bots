mod euro_to_mark;

use crate::config::Bots;
use crate::kaenguru::euro_to_mark::get_euro;
use crate::replies::reply_to;
use log::{debug, error, info, trace};
use serenity::{
    async_trait,
    builder::{CreateAllowedMentions, CreateEmbed, CreateEmbedFooter, CreateMessage},
    model::{channel::Message, gateway::Ready, prelude::*},
    prelude::*,
};

/// The default struct on which the bot is built
pub struct KaenguruHandler;

#[async_trait]
impl EventHandler for KaenguruHandler {
    /// The method that reacts to new messages.
    /// This method is called by serenity.
    ///
    /// # Arguments
    ///
    /// * `_ctx`: The context in which this message was sent. Contains information about the bot and its cache
    /// * `_new_message`: The message that was sent and to which this bot should react to.
    async fn message(&self, ctx: Context, new_message: Message) {
        // Do not do anything, if the message was sent by a bot.
        if new_message.author.bot {
            return;
        }

        if let Ok(_) = reply_to(&ctx, &new_message, Bots::KaenguruKnecht).await {
            return;
        }

        trace!("Checking for any amount of euros in the message...");
        // Check if a € symbol or EUR is in the message, if so try to parse the cash amount
        if new_message.content.to_lowercase().contains("€")
            || new_message.content.to_lowercase().contains("eur")
        {
            let number = get_euro(&new_message.content.to_lowercase());

            // Check if a number was present in the message
            if let Ok(number) = number {
                if number == 0 {
                    debug!("Message did not contain a number to convert to EUROs. Returning.");
                    return;
                }
                let description = match number > 100_000 {
                    // If the number is bigger than 100,000 send an "Error" message
                    true => format!("Huiuiui! So viele Schulden kann die DDR doch nicht haben!"),
                    // If the number is smaller than 100,000 send a computed message.
                    false => {
                        // If the number is also smaller than 10, append "Kleinvieh macht auch
                        // Mist!" to the message
                        match number < 10 {
                            true => format!(
                                "{} Euro? Das, das sind ja {} Mark! {} Ostmark! {} Ostmark aufm Schwarzmarkt!\n\nKleinvieh macht auch Mist!",
                                number,
                                number * 2,
                                number * 4,
                                number * 8
                            ),
                            false => format!(
                                "{} Euro? Das, das sind ja {} Mark! {} Ostmark! {} Ostmark aufm Schwarzmarkt!",
                                number,
                                number * 2,
                                number * 4,
                                number * 8
                            ),
                        }
                    }
                };

                let mut embed = CreateEmbed::new()
                    // TODO add Author to the bot instance
                    // Set the description of the description of above
                    .description(&description)
                    // Set the footer to "War ich ein guter Rechenknecht"?
                    .footer(CreateEmbedFooter::new("War ich ein guter Rechenknecht?"));
                // change the color to red if the number is bigger than 100,000
                if number > 100_000 {
                    embed = embed.color(Color::from_rgb(255, 0, 0));
                }

                // Send a reply message as an embed
                match new_message
                    .channel_id
                    .send_message(
                        &ctx.http,
                        CreateMessage::new()
                            .embed(embed)
                            // References the original message
                            .reference_message(&new_message)
                            // Need to set this to false, because it would otherwise change the message
                            // background yellow (for the user who wrote it).
                            .allowed_mentions(CreateAllowedMentions::new().replied_user(false)),
                    )
                    .await
                {
                    // TODO add channel name (utils::get_channel function)
                    Ok(msg) => info!(
                        "Sending \"{}\" + embed to {}",
                        description.replace("\n", "\\n"),
                        msg.channel_id
                    ),
                    Err(why) => error!("Error sending message: {:?}", why),
                };
            };
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
    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        info!("Logged in as {}", data_about_bot.user.name);

        //TODO log invite links
        //        let permissions = Permissions::default();
        //        match data_about_bot.user.invite_url(&ctx, permissions).await {
        //            Ok(url) => info!("Bot invitation url is: {}", url),
        //            Err(why) => error!("Error getting invite url: {}", why),
        //        };
    }
}
