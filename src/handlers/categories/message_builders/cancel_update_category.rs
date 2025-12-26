use crate::handlers::callback;
use crate::proto::callback::v1::callback::Query;
use crate::proto::callback::v1::update_category::Field;
use crate::proto::callback::v1::{
    Callback, CancelUpdateCategory, CategoryDirection, UpdateCategory,
};
use crate::services;
use std::sync::Arc;
use teloxide::prelude::CallbackQuery;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub struct MessageBuilder {
    callback_query: CallbackQuery,
    service: Arc<dyn services::categories::Service>,
    callback: CancelUpdateCategory,
}

impl MessageBuilder {
    pub fn new(
        callback_query: CallbackQuery,
        service: Arc<dyn services::categories::Service>,
        callback: CancelUpdateCategory,
    ) -> Self {
        Self {
            callback_query,
            service,
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
