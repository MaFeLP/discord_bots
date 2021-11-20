mod euro_to_mark;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, Permissions},
    prelude::*,
};
use serenity::utils::Color;
use crate::kaenguru::euro_to_mark::get_euro;

pub struct KaenguruHandler;

#[async_trait]
impl EventHandler for KaenguruHandler {
    async fn message(&self, _ctx: Context, mut _new_message: Message) {
        if _new_message.author.bot {
            return;
        }

        if _new_message.content.to_lowercase().contains("€")
            || _new_message.content.to_lowercase().contains("eur")
        {
            let number = get_euro(&_new_message.content.to_lowercase());

            if let Ok(number) = number {
                if number == 0 {
                    println!("[KG]:\tMessage did not contain a number to convert to EUROs. Returning.");
                    return;
                }
                let description = match number > 100_000 {
                    true => format!(
                        "Huiuiui! So viele Schulden kann die DDR doch nicht haben!"
                    ),
                    false => {
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

                match _new_message
                    .channel_id
                    .send_message(&_ctx.http, |m| {
                        m.embed(|e| {
                            //e.author();
                            e.description(&description);
                            e.footer(|f| {
                                f.text("War ich ein guter Rechenknecht?");
                                f
                            });
                            if number > 100_000 {
                                e.color(Color::from_rgb(255, 0, 0));
                            }

                            e
                        });
                        // References the original message
                        m.reference_message(&_new_message);
                        m.allowed_mentions(|f| {
                            // Need to set this to false, because it would otherwise change the message
                            // background yellow (for the user who wrote it).
                            f.replied_user(false);
                            f
                        });

                        m
                    })
                    .await
                {
                    Ok(msg) => println!("[KG]:\tSending money Message to {}", msg.channel_id),
                    Err(why) => println!("[KG]:\tError sending message: {:?}", why),
                };
            };
        }
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("[KG]:\tLogged in as {}", _data_about_bot.user.name);

        let permissions =
            Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES | Permissions::EMBED_LINKS;
        match _data_about_bot.user.invite_url(&_ctx, permissions).await {
            Ok(url) => println!("[KG]:\tBot invitation url is: {}", url),
            Err(why) => println!("[KG]:\tError getting invite url: {}", why),
        };
    }
}
