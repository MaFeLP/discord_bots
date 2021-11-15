use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, Permissions},
    prelude::*,
};

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
            {
                let mut tens: u64 = 1;
                let mut in_euro = false;
                let mut first_space = true;
                for c in _new_message.content.to_lowercase().chars().rev() {
                    if c == '€' {
                        in_euro = true;
                    }
                    if in_euro {
                        if c == ' ' {
                            if first_space {
                                first_space = false;
                            } else {
                                in_euro = false;
                            }
                            continue;
                        } else {
                            for n in 0..9 {
                                if n.to_string() == c.to_string() {
                                    number += n * tens;
                                    tens *= 10;
                                    continue;
                                }
                                //dbg!("Found unknown character: {}", c)
                            }
                        }
                    }
                }
            }

            let description = format!(
                "{} Euro? Das, das sind ja {} Mark! {} Ostmark! {} Ostmark aufm Schwarzmarkt!",
                number,
                number * 2,
                number * 4,
                number * 8
            );

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

                        e
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
