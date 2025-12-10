use crate::handlers::callback::MessageBuilder;
use crate::handlers::categories::callback::{
    CreateCategoryMessageBuilder, ShowCategoriesSettingsMessageBuilder,
    ShowCategorySettingsMessageBuilder, UpdateCategoryMessageBuilder,
};
use crate::proto::callback::v1::{
    CreateCategory, ShowCategoriesSettings, ShowCategorySettings, UpdateCategory,
};
use crate::services;
use crate::telegram::{Dialog, HandlerResult};
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::sugar::bot::BotMessagesExt;
use teloxide::types::ParseMode;
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

pub async fn show_category_settings(
    bot: Bot,
    _dialog: Dialog,
    callback_query: CallbackQuery,
    query: ShowCategorySettings,
    categories_service: Arc<dyn services::categories::Service>,
) -> HandlerResult {
    let builder = ShowCategorySettingsMessageBuilder::new(
        callback_query.to_owned(),
        categories_service,
        query,
    );

    if let Some(message) = callback_query.regular_message() {
        bot.edit_text(message, builder.text().await)
            .reply_markup(builder.reply_markup().await)
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
    }

    Ok(())
}

pub async fn update_category(
    bot: Bot,
    _dialog: Dialog,
    callback_query: CallbackQuery,
    query: UpdateCategory,
    categories_service: Arc<dyn services::categories::Service>,
) -> HandlerResult {
    let builder =
        UpdateCategoryMessageBuilder::new(callback_query.to_owned(), categories_service, query);

    if let Some(message) = callback_query.regular_message() {
        bot.send_message(message.chat.id, builder.text().await)
            .reply_markup(builder.reply_markup().await)
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
    }

    Ok(())
}
