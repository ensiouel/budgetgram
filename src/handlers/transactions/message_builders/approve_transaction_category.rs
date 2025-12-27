use crate::handlers::callback;
use crate::proto::callback::v1::ApproveTransactionCategory;
use crate::services;
use std::sync::Arc;
use teloxide::prelude::CallbackQuery;
use teloxide::types::InlineKeyboardMarkup;

pub struct MessageBuilder {
    callback_query: CallbackQuery,
    categories_service: Arc<dyn services::categories::Service>,
    transactions_service: Arc<dyn services::transactions::Service>,
    callback: ApproveTransactionCategory,
}

impl MessageBuilder {
    pub fn new(
        callback_query: CallbackQuery,
        categories_service: Arc<dyn services::categories::Service>,
        transactions_service: Arc<dyn services::transactions::Service>,
        callback: ApproveTransactionCategory,
    ) -> Self {
        Self {
            callback_query,
            categories_service,
            transactions_service,
            callback,
        }
    }
}

#[async_trait::async_trait]
impl callback::MessageBuilder for MessageBuilder {
    async fn text(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok("".to_string())
    }

    async fn reply_markup(
        &self,
    ) -> Result<InlineKeyboardMarkup, Box<dyn std::error::Error + Send + Sync>> {
        Ok(InlineKeyboardMarkup::default())
    }
}
