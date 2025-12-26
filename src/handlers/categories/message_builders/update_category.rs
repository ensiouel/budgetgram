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
    query: UpdateCategory,
}

impl MessageBuilder {
    pub fn new(
        callback_query: CallbackQuery,
        service: Arc<dyn services::categories::Service>,
        query: UpdateCategory,
    ) -> Self {
        Self {
            callback_query,
            service,
            query,
        }
    }
}

#[async_trait::async_trait]
impl callback::MessageBuilder for MessageBuilder {
    async fn text(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let category = self
            .service
            .get_category(
                self.callback_query.regular_message().unwrap().chat.id.0,
                self.query.category_id,
            )
            .await?;

        Ok(match Field::try_from(self.query.field).ok() {
            Some(Field::Name) => format!(
                "✏️ *Изменение названия категории: {} {}*

*Текущее название:* {}

_Введите новое название категории\\._",
                category.label, category.name, category.name
            ),
            Some(Field::Label) => format!(
                "🏷 *Изменение ярлыка категории: {} {}*

*Текущий ярлык:* {}

Отправьте новый символ \\(например, 🛒, 🍎, 🏷️\\)\\.",
                category.label, category.name, category.label
            ),
            Some(Field::Direction) => "todo".to_string(),
            Some(Field::IsRegular) => "todo".to_string(),
            Some(Field::TargetAmount) => match category.direction {
                CategoryDirection::Expense => format!(
                    "✏️ *Изменение лимита категории: {} {}*

*Текущий лимит:* {:?}

_Введите новую сумму \\(или «0» для удаления лимита\\)\\._",
                    category.label,
                    category.name,
                    category.target_amount.unwrap_or(0),
                ),
                CategoryDirection::Income => format!(
                    "✏️ *Изменение плана*

*Категория:* «{} {}»
*Текущий план:* {:?}

_Введите новую целевую сумму \\(или «0» для удаления плана\\)\\._",
                    category.label,
                    category.name,
                    category.target_amount.unwrap_or(0),
                ),
                CategoryDirection::Unspecified => {
                    unreachable!()
                }
            },
            _ => {
                unreachable!()
            }
        })
    }

    async fn reply_markup(
        &self,
    ) -> Result<InlineKeyboardMarkup, Box<dyn std::error::Error + Send + Sync>> {
        Ok(
            InlineKeyboardMarkup::default().append_row(vec![InlineKeyboardButton::callback(
                "❌ Отменить",
                String::try_from(Callback {
                    query: Option::from(Query::CancelUpdateCategory(CancelUpdateCategory {})),
                })
                .unwrap(),
            )]),
        )
    }
}
