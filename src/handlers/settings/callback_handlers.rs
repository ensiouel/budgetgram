use crate::handlers::callback::MessageBuilder;
use crate::handlers::settings::message_builders::show_settings;
use crate::proto::callback::v1::ShowSettings;
use crate::telegram::{Dialog, HandlerResult};
use teloxide::Bot;
use teloxide::payloads::EditMessageTextSetters;
use teloxide::prelude::CallbackQuery;
use teloxide::sugar::bot::BotMessagesExt;

pub async fn show_settings(
    bot: Bot,
    _dialog: Dialog,
    callback_query: CallbackQuery,
    _query: ShowSettings,
) -> HandlerResult {
    let builder = show_settings::MessageBuilder::new();

    if let Some(message) = callback_query.regular_message() {
        bot.edit_text(message, builder.text().await?)
            .reply_markup(builder.reply_markup().await?)
            .await?;
    }

    Ok(())
}
