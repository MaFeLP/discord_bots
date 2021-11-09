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

        if _new_message.content.to_lowercase().contains("€") ||
            _new_message.content.to_lowercase().contains("eur") {
            match _new_message
                .channel_id
                .send_message(&_ctx.http, |m| {
                    m.embed(|e| {
                        //e.author();
                        e.description("So viele Euros???");

                        e
                    });

                    m
                }).await {
                Ok(msg) => println!("[KG]:\tSending money Message to {}", msg.channel_id),
                Err(why) => println!("[KG]:\tError sending message: {:?}", why),
            };
        }
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("[KG]:\tLogged in as {}", _data_about_bot.user.name);

        let permissions = Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES | Permissions::EMBED_LINKS;
        match _data_about_bot.user.invite_url(&_ctx, permissions).await {
            Ok(url) => println!("[KG]:\tBot invitation url is: {}", url),
            Err(why) => println!("[KG]:\tError getting invite url: {}", why),
        };
    }
}
