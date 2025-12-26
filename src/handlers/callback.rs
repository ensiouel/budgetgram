use crate::handlers::{categories, settings};
use crate::proto::callback::v1::Callback;
use crate::proto::callback::v1::callback::Query;
use crate::services;
use crate::telegram::{Dialog, HandlerResult};
use std::sync::Arc;
use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::InlineKeyboardMarkup;

pub async fn match_callback_query(
    bot: Bot,
    dialog: Dialog,
    callback_query: CallbackQuery,
    categories_service: Arc<dyn services::categories::Service>,
) -> HandlerResult {
    let Some(data) = callback_query.to_owned().data else {
        return Ok(());
    };
    let Ok(callback) = Callback::try_from(data) else {
        return Ok(());
    };
    let Some(query) = callback.query else {
        return Ok(());
    };

    match query {
        Query::ShowSettings(show_setting) => {
            settings::callback_handlers::show_settings(
                bot.to_owned(),
                dialog.to_owned(),
                callback_query.to_owned(),
                show_setting.to_owned(),
            )
            .await?;
        }
        Query::ShowCategoriesSettings(show_categories_settings) => {
            categories::callback_handlers::show_categories_settings(
                bot.to_owned(),
                dialog.to_owned(),
                callback_query.to_owned(),
                show_categories_settings.to_owned(),
                categories_service,
            )
            .await?;
        }
        Query::ShowCategorySettings(show_category_settings) => {
            categories::callback_handlers::show_category_settings(
                bot.to_owned(),
                dialog.to_owned(),
                callback_query.to_owned(),
                show_category_settings.to_owned(),
                categories_service,
            )
            .await?;
        }
        Query::CreateCategory(create_category) => {
            categories::callback_handlers::create_category(
                bot.to_owned(),
                dialog.to_owned(),
                callback_query.to_owned(),
                create_category.to_owned(),
            )
            .await?;
        }
        Query::CancelCreateCategory(_) => {
            categories::callback_handlers::cancel_create_category(
                bot.to_owned(),
                dialog.to_owned(),
                callback_query.to_owned(),
            )
            .await?;
        }
        Query::UpdateCategory(update_category) => {
            categories::callback_handlers::update_category(
                bot.to_owned(),
                dialog.to_owned(),
                callback_query.to_owned(),
                update_category.to_owned(),
                categories_service,
            )
            .await?;
        }
        _ => {}
    }

    bot.answer_callback_query(callback_query.id.to_owned())
        .await?;

    Ok(())
}

#[async_trait::async_trait]
pub trait MessageBuilder {
    async fn text(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    async fn reply_markup(
        &self,
    ) -> Result<InlineKeyboardMarkup, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait::async_trait]
pub trait CancellableMessageBuilder {
    async fn text(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    async fn reply_markup(
        &self,
    ) -> Result<InlineKeyboardMarkup, Box<dyn std::error::Error + Send + Sync>>;
}
