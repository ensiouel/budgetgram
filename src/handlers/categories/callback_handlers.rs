use crate::handlers::callback::MessageBuilder;
use crate::handlers::categories::callback::{
    CreateCategoryMessageBuilder, ShowCategoriesSettingsMessageBuilder,
};
use crate::proto::callback::v1::{CreateCategory, ShowCategoriesSettings};
use crate::telegram::{Dialog, HandlerResult};
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
) -> HandlerResult {
    let builder = ShowCategoriesSettingsMessageBuilder::new(None, query);

    if let Some(message) = callback_query.regular_message() {
        bot.edit_text(message, builder.text())
            .reply_markup(builder.reply_markup())
            .await?;
    }

    Ok(())
}
