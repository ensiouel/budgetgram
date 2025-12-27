use crate::handlers::callback::CancellableMessageBuilder;
use crate::handlers::callback::MessageBuilder;
use crate::handlers::categories::message_builders::{
    cancel_update_category, create_category, show_categories_settings, show_category_settings,
    update_category,
};
use crate::proto::callback::v1::{CancelUpdateCategory, CreateCategory, ShowCategoryList, ShowCategorySettings, UpdateCategory};
use crate::services;
use crate::telegram::{State};
use crate::telegram::{Dialog, HandlerResult};
use std::sync::Arc;
use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::sugar::bot::BotMessagesExt;
use teloxide::types::{InlineKeyboardMarkup, ParseMode};

pub async fn create_category(
    bot: Bot,
    dialog: Dialog,
    callback_query: CallbackQuery,
    callback: CreateCategory,
) -> HandlerResult {
    let builder = create_category::MessageBuilder::new(callback);
    let text = builder.text().await?;
    let keyboard = builder.reply_markup().await?;

    if let Some(message) = callback_query.regular_message() {
        match dialog.get().await? {
            Some(state) => match state {
                State::None => {}
                State::UpdateCategory {
                    answer_message_id,
                    callback,
                } => {}
                State::CreateCategory {
                    answer_message_id,
                    callback,
                } => {
                    bot.send_message(
                        message.chat.id,
                        "Пожалуйста, сначала завершите создание текущей категории.",
                    )
                    .reply_markup(keyboard)
                    .parse_mode(ParseMode::MarkdownV2)
                    .await?;
                    return Ok(());
                }
            },
            None => {}
        }

        let answer_message = bot
            .send_message(message.chat.id, text)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::MarkdownV2)
            .await?;

        dialog
            .update(State::CreateCategory {
                answer_message_id: answer_message.id,
                callback,
            })
            .await?;
    }

    Ok(())
}

pub async fn show_categories_settings(
    bot: Bot,
    _dialog: Dialog,
    callback_query: CallbackQuery,
    query: ShowCategoryList,
    categories_service: Arc<dyn services::categories::Service>,
) -> HandlerResult {
    let builder = show_categories_settings::MessageBuilder::new(
        callback_query.to_owned(),
        categories_service,
        query,
    );
    let text = builder.text().await?;
    let keyboard = builder.reply_markup().await?;

    if let Some(message) = callback_query.regular_message() {
        bot.edit_text(message, text).reply_markup(keyboard).await?;
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
    if let Some(message) = callback_query.regular_message() {
        let builder = show_category_settings::MessageBuilder::new(
            message.to_owned(),
            categories_service,
            query,
        );
        let text = builder.text().await?;
        let keyboard = builder.reply_markup().await?;

        bot.edit_text(message, text)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
    }

    Ok(())
}

pub async fn update_category(
    bot: Bot,
    dialog: Dialog,
    callback_query: CallbackQuery,
    query: UpdateCategory,
    categories_service: Arc<dyn services::categories::Service>,
) -> HandlerResult {
    let builder =
        update_category::MessageBuilder::new(callback_query.to_owned(), categories_service, query);
    let text = builder.text().await?;
    let keyboard = builder.reply_markup().await?;

    if let Some(message) = callback_query.regular_message() {
        let answer_message = bot
            .send_message(message.chat.id, text)
            .reply_markup(keyboard)
            .parse_mode(ParseMode::MarkdownV2)
            .await?;

        dialog
            .update(State::UpdateCategory {
                answer_message_id: answer_message.id,
                callback: query,
            })
            .await?;
    }

    Ok(())
}

pub async fn cancel_update_category(
    bot: Bot,
    dialog: Dialog,
    callback_query: CallbackQuery,
    callback: CancelUpdateCategory,
    categories_service: Arc<dyn services::categories::Service>,
) -> HandlerResult {
    let builder = cancel_update_category::MessageBuilder::new(
        callback_query.to_owned(),
        categories_service,
        callback,
    );
    let text = builder.text().await?;
    let keyboard = builder.reply_markup().await?;

    if let Some(message) = callback_query.regular_message() {
        bot.edit_text(message, "").await?;
        dialog.exit().await?;
    }

    Ok(())
}

pub async fn cancel_create_category(
    bot: Bot,
    dialog: Dialog,
    callback_query: CallbackQuery,
) -> HandlerResult {
    let Some(State::CreateCategory {
        callback: create_category_callback,
        answer_message_id,
    }) = dialog.get().await?
    else {
        return Ok(());
    };

    let builder = create_category::CancellableMessageBuilder::new(create_category_callback);
    let text = builder.text().await?;
    let keyboard = builder.reply_markup().await?;

    if let Some(message) = callback_query.regular_message() {
        bot.send_message(message.chat.id, text)
            .reply_markup(keyboard)
            .await?;

        bot.edit_message_reply_markup(message.chat.id, answer_message_id)
            .reply_markup(InlineKeyboardMarkup::default())
            .await?;

        dialog.exit().await?;
    }

    Ok(())
}
