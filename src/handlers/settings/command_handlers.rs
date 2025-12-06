use crate::handlers::callback::MessageBuilder;
use crate::handlers::settings::callback::ShowSettingsMessageBuilder;
use crate::telegram::{Dialog, HandlerResult};
use teloxide::prelude::*;
use teloxide::Bot;

pub async fn show_settings(bot: Bot, _dialog: Dialog, message: Message) -> HandlerResult {
    let builder = ShowSettingsMessageBuilder::new();
    bot.send_message(message.chat.id.to_owned(), builder.text())
        .reply_markup(builder.reply_markup())
        .await?;
    Ok(())
}
