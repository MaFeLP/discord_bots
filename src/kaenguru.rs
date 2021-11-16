use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, Permissions},
    prelude::*,
};
use serenity::utils::Color;

enum Euro {
    E,
    U,
    R,
}

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
            let mut number: u64 = 0;
            let mut over_100_000: bool = false;
            {
                let mut tens: u64 = 1;
                let mut in_euro = false;
                let mut first_space = true;
                let mut euro = Euro::R;
                for c in _new_message.content.to_lowercase().chars().rev() {
                    // To avoid buffer overflows, exit at over 100.000
                    if number > 100_000 {
                        over_100_000 = true;
                        break;
                    }
                    // Checks if EUR was written and then searches for a number
                    {
                        if c == 'r' {
                            euro = match euro {
                                Euro::R => Euro::U,
                                _ => Euro::R,
                            };
                            continue;
                        }
                        if c == 'u' {
                            euro = match euro {
                                Euro::U => Euro::E,
                                _ => Euro::R,
                            };
                            continue;
                        }
                        if c == 'e' {
                            euro = match euro {
                                Euro::E => {
                                    in_euro = true;
                                    Euro::R
                                },
                                _ => Euro::R,
                            };
                            continue;
                        }
                    }
                    if c == '€' {
                        in_euro = true;
                        continue;
                    }
                    if ! in_euro {
                        continue;
                    }
                    // Accepts one space between EUR/€ and the number
                    if c == ' ' {
                        if first_space {
                            first_space = false;
                            continue;
                        } else {
                            break;
                        }
                    // Accepts , as decimal separator
                    // Because we use german metrics, and don't want to have to deal with floats,
                    // The decimal point is reset.
                    } else if c == ',' {
                        tens = 1;
                        number = 0;
                    // Accepts . as (thousands) separator
                    } else if c == '.' {
                        continue;
                    } else {
                        let mut is_number = false;
                        for n in 0..10 {
                            if n.to_string() == c.to_string() {
                                number += n * tens;
                                tens *= 10;
                                is_number = true;
                                break;
                            }
                            //dbg!("Found unknown character: {}", c)
                        }
                        if ! is_number {
                            //number = 0;
                            break;
                        }
                    }
                }
            }

            if number == 0 {
                println!("[KG]:\tMessage did not contain a number to convert to EUROs. Returning.");
                return;
            }
            let description = match over_100_000 {
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
                        if over_100_000 {
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
