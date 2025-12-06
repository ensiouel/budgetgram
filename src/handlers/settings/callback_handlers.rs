use crate::handlers::callback::MessageBuilder;
use crate::handlers::settings::callback::ShowSettingsMessageBuilder;
use crate::proto::callback::v1::ShowSettings;
use crate::telegram::{Dialog, HandlerResult};
use teloxide::payloads::EditMessageTextSetters;
use teloxide::prelude::CallbackQuery;
use teloxide::sugar::bot::BotMessagesExt;
use teloxide::Bot;

pub async fn show_settings(
    bot: Bot,
    _dialog: Dialog,
    callback_query: CallbackQuery,
    query: ShowSettings,
) -> HandlerResult {
    let builder = ShowSettingsMessageBuilder::new();

    if let Some(message) = callback_query.regular_message() {
        bot.edit_text(message, builder.text())
            .reply_markup(builder.reply_markup())
            .await?;
    }

    Ok(())
}
