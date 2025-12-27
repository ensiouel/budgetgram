use crate::handlers::callback::MessageBuilder;
use crate::handlers::transactions::message_builders::create_transaction;
use crate::proto::callback::v1::CreateTransaction;
use crate::services;
use crate::telegram::{Dialog, HandlerResult};
use std::sync::Arc;
use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::sugar::bot::BotMessagesExt;

pub async fn create_transaction(
    bot: Bot,
    dialog: Dialog,
    callback_query: CallbackQuery,
    categories_service: Arc<dyn services::categories::Service>,
    transactions_service: Arc<dyn services::transactions::Service>,
    callback: CreateTransaction,
) -> HandlerResult {
    if let Some(message) = callback_query.regular_message() {
        let builder = create_transaction::MessageBuilder::new(
            message.chat.id,
            categories_service,
            transactions_service,
            callback,
        );
        let text = builder.text().await?;
        let keyboard = builder.reply_markup().await?;

        bot.edit_text(message, text).reply_markup(keyboard).await?;
    }

    Ok(())
}
