use crate::handlers::callback::MessageBuilder;
use crate::handlers::categories::callback::{
    CreateCategoryMessageBuilder, ShowCategoriesSettingsMessageBuilder,
};
use crate::proto::callback::v1::{CreateCategory, ShowCategoriesSettings};
use crate::services;
use crate::telegram::{Dialog, HandlerResult};
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::sugar::bot::BotMessagesExt;
use teloxide::Bot;

pub async fn create_category(
    bot: Bot,
    _dialog: Dialog,
    callback_query: CallbackQuery,
    query: CreateCategory,
) -> HandlerResult {
    let builder = CreateCategoryMessageBuilder::new();
    let text = builder.text();
    let keyboard = builder.reply_markup();

    Ok(())
}

pub async fn show_categories_settings(
    bot: Bot,
    _dialog: Dialog,
    callback_query: CallbackQuery,
    query: ShowCategoriesSettings,
    categories_service: Arc<dyn services::categories::Service>,
) -> HandlerResult {
    let builder = ShowCategoriesSettingsMessageBuilder::new(
        callback_query.to_owned(),
        categories_service,
        query,
    );

    if let Some(message) = callback_query.regular_message() {
        bot.edit_text(message, builder.text().await)
            .reply_markup(builder.reply_markup().await)
            .await?;
    }

    Ok(())
}
