use crate::handlers::callback;
use crate::models::callback::{Declinable, GrammaticalNumber, Labeled, NameCase};
use crate::proto::callback::v1::callback::Query;
use crate::proto::callback::v1::{
    Callback, CategoryDirection, CreateCategory, ShowCategoriesSettings, ShowCategorySettings,
    ShowSettings,
};
use crate::services;
use std::sync::Arc;
use teloxide::prelude::CallbackQuery;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub struct MessageBuilder {
    callback_query: CallbackQuery,
    service: Arc<dyn services::categories::Service>,
    query: ShowCategoriesSettings,
}

impl MessageBuilder {
    pub fn new(
        callback_query: CallbackQuery,
        service: Arc<dyn services::categories::Service>,
        query: ShowCategoriesSettings,
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
        let direction = CategoryDirection::try_from(self.query.category_direction)?;
        Ok(format!(
            "{} Выберите категорию {}",
            direction.label(),
            direction.decline(NameCase::Genitive, GrammaticalNumber::Plural)
        ))
    }

    async fn reply_markup(
        &self,
    ) -> Result<InlineKeyboardMarkup, Box<dyn std::error::Error + Send + Sync>> {
        let list = self
            .service
            .select_categories(
                self.callback_query.regular_message().unwrap().chat.id.0,
                CategoryDirection::try_from(self.query.category_direction).unwrap(),
            )
            .await?;

        let mut reply_markup = InlineKeyboardMarkup::default();

        for chunk in list.chunks(2) {
            let row: Vec<InlineKeyboardButton> = chunk
                .iter()
                .map(|category| {
                    InlineKeyboardButton::callback(
                        format!("{} {}", category.label, category.name),
                        String::try_from(Callback {
                            query: Option::from(Query::ShowCategorySettings(
                                ShowCategorySettings {
                                    category_id: category.id,
                                    navigated_from: Some(self.query),
                                },
                            )),
                        })
                        .unwrap(),
                    )
                })
                .collect();

            reply_markup = reply_markup.append_row(row);
        }

        Ok(reply_markup.append_row(vec![]).append_row(vec![
            InlineKeyboardButton::callback(
                "➕ Добавить",
                String::try_from(Callback {
                    query: Option::from(Query::CreateCategory(CreateCategory {
                        category_direction: self.query.category_direction,
                    })),
                })
                .unwrap(),
            ),
            InlineKeyboardButton::callback(
                "🔙 Назад",
                String::try_from(Callback {
                    query: Option::from(Query::ShowSettings(ShowSettings {})),
                })
                .unwrap(),
            ),
        ]))
    }
}
