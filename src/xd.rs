use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, Permissions},
    prelude::*,
};

pub struct XDHandler;

#[async_trait]
impl EventHandler for XDHandler {
    async fn message(&self, _ctx: Context, mut _new_message: Message) {
        if _new_message.author.bot {
            return;
        }

        if _new_message.content.to_lowercase().contains("xd") {
            match _new_message.reply(_ctx, "XDDDDDDD").await {
                Ok(msg) => println!("[XD]:\tSend message \"XDDDDDDD\" to channel {}", msg.channel_id),
                Err(why) => println!("[XD]:\tError sending message: {:?}", why)
            };
        }
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("[XD]:\tLogged in as {}", _data_about_bot.user.name);

        let permissions = Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES | Permissions::EMBED_LINKS;
        match _data_about_bot.user.invite_url(&_ctx, permissions).await {
            Ok(url) => {
                println!("[XD]:\tBot invitation url is: {}", url);
                return;
            }
            Err(why) => {
                println!("[XD:]\tError getting invite url: {}", why);
                return;
            }
        };
    }
}
