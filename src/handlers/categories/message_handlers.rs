use crate::proto::callback::v1::CategoryDirection;
use crate::services;
use crate::services::categories::CreateCategoryRequest;
use crate::telegram::{Dialog, HandlerResult, State};
use regex::Regex;
use std::sync::Arc;
use teloxide::Bot;
use teloxide::payloads::EditMessageReplyMarkupSetters;
use teloxide::prelude::{Message, Requester};
use teloxide::types::InlineKeyboardMarkup;

static RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
    Regex::new(r"^(?:(?P<label>[^\p{L}\p{N}\s])\s+)?(?P<name>[^-]+?)\s*(?:-\s*(?P<amount>.+))?$")
        .expect("invalid regex")
});

pub async fn create_category(
    bot: Bot,
    message: Message,
    dialog: Dialog,
    categories_service: Arc<dyn services::categories::Service>,
) -> HandlerResult {
    let text = match message.text() {
        Some(text) => text,
        None => {
            bot.send_message(
                message.chat.id,
                "Пожалуйста, отправьте текстовое сообщение.",
            )
            .await?;
            return Ok(());
        }
    };

    let captures = match RE.captures(text) {
        Some(captures) => captures,
        None => {
            bot.send_message(
                message.chat.id,
                "Неверный формат. Используйте формат: 💼 Зарплата - 500 000",
            )
            .await?;
            return Ok(());
        }
    };

    let label = captures
        .name("label")
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_default();

    let name = captures
        .name("name")
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_default();

    let amount = match captures.name("amount") {
        Some(m) => match m.as_str().trim().replace(' ', "").parse::<f64>() {
            Ok(amount) => amount,
            Err(_) => {
                bot.send_message(
                    message.chat.id,
                    "Неверный формат суммы. Используйте число (например: 1000 или 1 000)",
                )
                .await?;
                return Ok(());
            }
        },
        None => 0.0,
    };

    let Some(State::CreateCategory {
        answer_message_id,
        callback,
    }) = dialog.get().await?
    else {
        bot.send_message(
            message.chat.id,
            "Сессия устарела. Пожалуйста, начните создание категории заново.",
        )
        .await?;

        return Ok(());
    };

    categories_service
        .create_category(CreateCategoryRequest {
            chat_id: message.chat.id.0,
            name,
            label,
            direction: CategoryDirection::try_from(callback.category_direction)
                .unwrap_or(CategoryDirection::Expense),
            is_regular: true,
            target_amount: Some((amount * 100.0) as i64),
        })
        .await?;

    dialog.reset().await?;

    bot.edit_message_reply_markup(message.chat.id, answer_message_id)
        .reply_markup(InlineKeyboardMarkup::default())
        .await?;

    Ok(())
}
