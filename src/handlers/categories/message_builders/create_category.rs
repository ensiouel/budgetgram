use crate::handlers::callback;
use crate::proto::callback::v1::callback::Query;
use crate::proto::callback::v1::{
    Callback, CancelCreateCategory, CategoryDirection, CreateCategory, ShowCategoriesSettings,
    ShowCategorySettings,
};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub struct MessageBuilder {
    callback: CreateCategory,
}

impl MessageBuilder {
    pub fn new(callback: CreateCategory) -> Self {
        MessageBuilder { callback }
    }
}

#[async_trait::async_trait]
impl callback::MessageBuilder for MessageBuilder {
    async fn text(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match CategoryDirection::try_from(self.callback.category_direction)? {
            CategoryDirection::Expense => {
                Ok("📉 Введите название новой категории расходов и ежемесячный лимит по ней, используя следующий формат:

💼 Зарплата \\- 500 000".to_string())
            }
            CategoryDirection::Income => {
                Ok("📈 Введите название новой категории доходов и ежемесячную плановую сумму для неё, используя следующий формат:

🍎 Фрукты \\- 300".to_string())
            }
            CategoryDirection::Unspecified => {
                unreachable!()
            }
        }
    }

    async fn reply_markup(
        &self,
    ) -> Result<InlineKeyboardMarkup, Box<dyn std::error::Error + Send + Sync>> {
        Ok(
            InlineKeyboardMarkup::default().append_row(vec![InlineKeyboardButton::callback(
                "❌ Отменить",
                String::try_from(Callback {
                    query: Option::from(Query::CancelCreateCategory(CancelCreateCategory {})),
                })
                .unwrap(),
            )]),
        )
    }
}

pub struct CancellableMessageBuilder {
    callback: CreateCategory,
}

impl CancellableMessageBuilder {
    pub fn new(callback: CreateCategory) -> Self {
        CancellableMessageBuilder { callback }
    }
}

#[async_trait::async_trait]
impl callback::CancellableMessageBuilder for CancellableMessageBuilder {
    async fn text(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match CategoryDirection::try_from(self.callback.category_direction)? {
            CategoryDirection::Expense => {
                Ok("⚠️ Создание категории расходов отменено.".to_string())
            }
            CategoryDirection::Income => Ok("⚠️ Создание категории доходов отменено.".to_string()),
            CategoryDirection::Unspecified => unreachable!(),
        }
    }

    async fn reply_markup(
        &self,
    ) -> Result<InlineKeyboardMarkup, Box<dyn std::error::Error + Send + Sync>> {
        Ok(
            InlineKeyboardMarkup::default().append_row(vec![InlineKeyboardButton::callback(
                "🔙 К настройкам категорий",
                String::try_from(Callback {
                    query: Option::from(Query::ShowCategoriesSettings(ShowCategoriesSettings {
                        category_direction: self.callback.category_direction,
                    })),
                })
                .unwrap(),
            )]),
        )
    }
}
