use crate::handlers::callback;
use crate::proto::callback::v1::callback::Query;
use crate::proto::callback::v1::{Callback, CategoryDirection, ShowCategoriesSettings};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub struct MessageBuilder {}

impl MessageBuilder {
    pub fn new() -> Self {
        MessageBuilder {}
    }
}

#[async_trait::async_trait]
impl callback::MessageBuilder for MessageBuilder {
    async fn text(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok("⚙️ Выберите раздел настроек".to_string())
    }

    async fn reply_markup(
        &self,
    ) -> Result<InlineKeyboardMarkup, Box<dyn std::error::Error + Send + Sync>> {
        Ok(InlineKeyboardMarkup::default().append_row(vec![
            InlineKeyboardButton::callback(
                "📉 Расходы",
                String::try_from(Callback {
                    query: Option::from(Query::ShowCategoriesSettings(ShowCategoriesSettings {
                        category_direction: i32::from(CategoryDirection::Expense),
                    })),
                })
                .unwrap(),
            ),
            InlineKeyboardButton::callback(
                "📈 Доходы",
                String::try_from(Callback {
                    query: Option::from(Query::ShowCategoriesSettings(ShowCategoriesSettings {
                        category_direction: i32::from(CategoryDirection::Income),
                    })),
                })
                .unwrap(),
            ),
        ]))
    }
}
