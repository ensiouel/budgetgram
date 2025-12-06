use crate::handlers::callback::MessageBuilder;
use crate::proto::callback::v1::callback::Query;
use crate::proto::callback::v1::{Callback, CategoryDirection, ShowCategoriesSettings};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub struct ShowSettingsMessageBuilder {}

#[async_trait::async_trait]
impl MessageBuilder for ShowSettingsMessageBuilder {
    async fn text(&self) -> String {
        "⚙️ Выберите раздел настроек".to_string().clone()
    }

    async fn reply_markup(&self) -> InlineKeyboardMarkup {
        const PAGE_SIZE: i64 = 8;
        InlineKeyboardMarkup::default().append_row(vec![
            InlineKeyboardButton::callback(
                "📉 Расходы",
                String::try_from(Callback {
                    query: Option::from(Query::ShowCategoriesSettings(ShowCategoriesSettings {
                        category_direction: i32::from(CategoryDirection::Expense),
                        page: 0,
                        page_size: PAGE_SIZE,
                    })),
                })
                .unwrap(),
            ),
            InlineKeyboardButton::callback(
                "📈 Доходы",
                String::try_from(Callback {
                    query: Option::from(Query::ShowCategoriesSettings(ShowCategoriesSettings {
                        category_direction: i32::from(CategoryDirection::Income),
                        page: 0,
                        page_size: PAGE_SIZE,
                    })),
                })
                .unwrap(),
            ),
        ])
    }
}

impl ShowSettingsMessageBuilder {
    pub fn new() -> Self {
        ShowSettingsMessageBuilder {}
    }
}
