use crate::handlers::callback::MessageBuilder;
use crate::handlers::transactions::message_builders;
use crate::models::transaction::CurrencyCode;
use crate::proto::callback::v1::{CategoryDirection, CreateTransaction};
use crate::services;
use crate::services::transactions::CreateTransactionRequest;
use crate::telegram::{Dialog, HandlerResult};
use meval::Expr;
use regex::Regex;
use std::sync::Arc;
use teloxide::Bot;
use teloxide::dispatching::dialogue::GetChatId;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester};

static RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
    Regex::new(r"^(?P<expression>\d+(?:\s*[+\-*/().\s^\d]+)?)\s+(?P<description>.+)$")
        .expect("invalid regex")
});

pub async fn create_transaction(
    bot: Bot,
    message: Message,
    dialog: Dialog,
    categories_service: Arc<dyn services::categories::Service>,
    transactions_service: Arc<dyn services::transactions::Service>,
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
                "Неверный формат. Используйте формат: 200 шоколад или 20 * 2 хлеб",
            )
            .await?;
            return Ok(());
        }
    };

    let expression_caps = match captures.name("expression") {
        Some(expression_caps) => expression_caps.as_str().to_string(),
        None => {
            bot.send_message(
                message.chat.id,
                "Неверный формат. Не найдена сумма или выражение",
            )
            .await?;
            return Ok(());
        }
    };
    let expr = match expression_caps.parse::<Expr>().ok() {
        Some(expr) => expr,
        None => {
            bot.send_message(
                message.chat.id,
                "Неверный формат. Не удалось распознать сумму или выражение",
            )
            .await?;
            return Ok(());
        }
    };
    let amount = match expr.eval() {
        Ok(amount) => amount,
        Err(_) => {
            bot.send_message(
                message.chat.id,
                "Неверный формат. Не удалось вычислить сумму или выражение",
            )
            .await?;
            return Ok(());
        }
    };

    let description = captures
        .name("description")
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_default();

    let transaction = transactions_service
        .create_transaction(CreateTransactionRequest {
            chat_id: message.chat.id.0,
            category_id: None,
            amount: (amount * 100.0) as i64,
            description,
            currency_code: CurrencyCode::RUB,
        })
        .await?;

    let builder = message_builders::create_transaction::MessageBuilder::new(
        message.chat_id().unwrap(),
        categories_service,
        transactions_service,
        CreateTransaction {
            transaction_id: transaction.id,
            category_direction: i32::from(CategoryDirection::Expense),
            is_short_mode: true,
        },
    );

    bot.send_message(message.chat.id, builder.text().await?)
        .reply_markup(builder.reply_markup().await?)
        .await?;

    Ok(())
}
