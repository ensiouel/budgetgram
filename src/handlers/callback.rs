use crate::handlers::{categories, settings};
use crate::proto::callback::v1::callback::Query;
use crate::proto::callback::v1::Callback;
use crate::telegram::{Dialog, HandlerResult};
use teloxide::prelude::*;
use teloxide::types::InlineKeyboardMarkup;
use teloxide::Bot;

pub async fn match_callback_query(
    bot: Bot,
    dialog: Dialog,
    callback_query: CallbackQuery,
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
        _ => {}
    }

    bot.answer_callback_query(callback_query.id.to_owned())
        .await?;

    Ok(())
}

pub trait MessageBuilder {
    fn text(&self) -> String;
    fn reply_markup(&self) -> InlineKeyboardMarkup;
}
