use crate::handlers::callback::MessageBuilder;
use crate::proto::callback::v1::callback::Query;
use crate::proto::callback::v1::{
    Callback, CategoryDirection, ShowCategoriesSettings, ShowSettings,
};
use crate::services;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub struct CreateCategoryMessageBuilder {}

impl MessageBuilder for CreateCategoryMessageBuilder {
    fn text(&self) -> String {
        "".to_string()
    }

    fn reply_markup(&self) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::default()
    }
}

impl CreateCategoryMessageBuilder {
    pub fn new() -> Self {
        CreateCategoryMessageBuilder {}
    }
}

pub struct ShowCategoriesSettingsMessageBuilder {
    service: Box<dyn services::categories::Service>,
    query: ShowCategoriesSettings,
}

impl MessageBuilder for ShowCategoriesSettingsMessageBuilder {
    fn text(&self) -> String {
        let (icon, text) = match CategoryDirection::try_from(self.query.category_direction).ok() {
            Some(CategoryDirection::Expense) => ("📉", "расходов"),
            Some(CategoryDirection::Income) => ("📈", "доходов"),
            _ => ("", ""),
        };
        format!("{icon} Выберите категорию {text}",)
    }

    fn reply_markup(&self) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::default()
            .append_row(vec![])
            .append_row(vec![InlineKeyboardButton::callback(
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
        service: Box<dyn services::categories::Service>,
        query: ShowCategoriesSettings,
    ) -> Self {
        ShowCategoriesSettingsMessageBuilder { service, query }
    }
}
