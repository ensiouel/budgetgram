use crate::handlers::callback::MessageBuilder;
use crate::handlers::settings::message_builders::show_settings;
use crate::telegram::{Dialog, HandlerResult};
use teloxide::Bot;
use teloxide::prelude::*;

pub async fn show_settings(bot: Bot, _dialog: Dialog, message: Message) -> HandlerResult {
    let builder = show_settings::MessageBuilder::new();

    bot.send_message(message.chat.id, builder.text().await?)
        .reply_markup(builder.reply_markup().await?)
        .await?;

    Ok(())
}
