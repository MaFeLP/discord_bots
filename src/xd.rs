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

pub struct XDHandler;

#[async_trait]
impl EventHandler for XDHandler {
    async fn message(&self, _ctx: Context, mut _new_message: Message) {
        if _new_message.author.bot {
            return;
        }

        for (key, value) in replies() {
            if _new_message
                .content
                .to_lowercase()
                .contains(&key.to_lowercase())
            {
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

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("[XD]:\tLogged in as {}", _data_about_bot.user.name);

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
