use crate::handlers::callback::MessageBuilder;
use crate::proto::callback::v1::callback::Query;
use crate::proto::callback::v1::{
    Callback, CategoryDirection, ShowCategoriesSettings, ShowCategorySettings, ShowSettings,
};
use crate::services;
use std::sync::Arc;
use teloxide::prelude::CallbackQuery;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub struct CreateCategoryMessageBuilder {}

#[async_trait::async_trait]
impl MessageBuilder for CreateCategoryMessageBuilder {
    async fn text(&self) -> String {
        "".to_string()
    }

    async fn reply_markup(&self) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::default()
    }
}

impl CreateCategoryMessageBuilder {
    pub fn new() -> Self {
        CreateCategoryMessageBuilder {}
    }
}

pub struct ShowCategoriesSettingsMessageBuilder {
    callback_query: CallbackQuery,
    service: Arc<dyn services::categories::Service>,
    query: ShowCategoriesSettings,
}

#[async_trait::async_trait]
impl MessageBuilder for ShowCategoriesSettingsMessageBuilder {
    async fn text(&self) -> String {
        let (icon, text) = match CategoryDirection::try_from(self.query.category_direction).ok() {
            Some(CategoryDirection::Expense) => ("📉", "расходов"),
            Some(CategoryDirection::Income) => ("📈", "доходов"),
            _ => ("", ""),
        };
        format!("{icon} Выберите категорию {text}",)
    }

    async fn reply_markup(&self) -> InlineKeyboardMarkup {
        let list = self
            .service
            .select_categories(
                self.callback_query.regular_message().unwrap().chat.id.0,
                CategoryDirection::try_from(self.query.category_direction).unwrap(),
            )
            .await
            .unwrap();

        let mut reply_markup = InlineKeyboardMarkup::default();

        const PAGE_SIZE: usize = 6;

        for cats in list.chunks(if list.len() > PAGE_SIZE { 2 } else { 1 }) {
            let row: Vec<InlineKeyboardButton> = cats
                .iter()
                .map(|category| {
                    InlineKeyboardButton::callback(
                        format!("{} {}", category.label, category.name),
                        String::try_from(Callback {
                            query: Option::from(Query::ShowCategorySettings(
                                ShowCategorySettings {
                                    category_id: category.id,
                                    navigation_from: Option::from(self.query),
                                },
                            )),
                        })
                        .unwrap(),
                    )
                })
                .collect();

            reply_markup = reply_markup.append_row(row);
        }

        reply_markup.append_row(vec![InlineKeyboardButton::callback(
            "Назад",
            String::try_from(Callback {
                query: Option::from(Query::ShowSettings(ShowSettings {})),
            })
            .unwrap(),
        )])
    }
}

impl ShowCategoriesSettingsMessageBuilder {
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
