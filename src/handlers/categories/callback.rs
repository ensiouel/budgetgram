use crate::handlers::callback::MessageBuilder;
use crate::proto::callback::v1::callback::Query;
use crate::proto::callback::v1::update_category::Field;
use crate::proto::callback::v1::{
    Callback, CategoryDirection, CreateCategory, DeleteCategory, ShowCategoriesSettings,
    ShowCategorySettings, ShowSettings, UpdateCategory,
};
use crate::services;
use std::sync::Arc;
use teloxide::prelude::CallbackQuery;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub struct CreateCategoryMessageBuilder {}

impl CreateCategoryMessageBuilder {
    pub fn new() -> Self {
        CreateCategoryMessageBuilder {}
    }
}

#[async_trait::async_trait]
impl MessageBuilder for CreateCategoryMessageBuilder {
    async fn text(&self) -> String {
        "".to_string()
    }

    async fn reply_markup(&self) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::default()
    }
}

pub struct ShowCategoriesSettingsMessageBuilder {
    callback_query: CallbackQuery,
    service: Arc<dyn services::categories::Service>,
    query: ShowCategoriesSettings,
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

        reply_markup.append_row(vec![
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
        ])
    }
}

pub struct ShowCategorySettingsMessageBuilder {
    callback_query: CallbackQuery,
    service: Arc<dyn services::categories::Service>,
    query: ShowCategorySettings,
}

impl ShowCategorySettingsMessageBuilder {
    pub fn new(
        callback_query: CallbackQuery,
        service: Arc<dyn services::categories::Service>,
        query: ShowCategorySettings,
    ) -> Self {
        Self {
            callback_query,
            service,
            query,
        }
    }
}

#[async_trait::async_trait]
impl MessageBuilder for ShowCategorySettingsMessageBuilder {
    async fn text(&self) -> String {
        let category = self
            .service
            .get_category(
                self.callback_query.regular_message().unwrap().chat.id.0,
                self.query.category_id,
            )
            .await
            .unwrap();

        let name = category.name;
        let label = category.label;
        let (direction, target_limit_title, is_regular_title, target_limit) =
            match category.direction {
                CategoryDirection::Expense => (
                    "Расход 📉",
                    "Лимит",
                    "расход",
                    match category.target_amount {
                        Some(amount) => {
                            format!("{}", amount)
                        }
                        None => "_Без ограничений_".to_owned(),
                    },
                ),
                CategoryDirection::Income => (
                    "Доход 📈",
                    "План",
                    "доход",
                    match category.target_amount {
                        Some(amount) => {
                            format!("{}", amount)
                        }
                        None => "_Не установлен_".to_owned(),
                    },
                ),
                CategoryDirection::Unspecified => {
                    unreachable!()
                }
            };

        let is_regular = if category.is_regular {
            "_Да_"
        } else {
            "_Нет_"
        };

        format!(
            "⚙️ *Настройки категории*
*Название:* {name}
*Ярлык:* {label}
*Тип:* {direction}
*{target_limit_title}:* {target_limit}
*Постоянный {is_regular_title}:* {is_regular}",
        )
    }

    async fn reply_markup(&self) -> InlineKeyboardMarkup {
        let category = self
            .service
            .get_category(
                self.callback_query.regular_message().unwrap().chat.id.0,
                self.query.category_id,
            )
            .await
            .unwrap();

        InlineKeyboardMarkup::default()
            .append_row(vec![
                InlineKeyboardButton::callback(
                    "✏ Изменить название",
                    String::try_from(Callback {
                        query: Option::from(Query::UpdateCategory(UpdateCategory {
                            category_id: self.query.category_id,
                            field: i32::from(Field::Name),
                        })),
                    })
                    .unwrap(),
                ),
                InlineKeyboardButton::callback(
                    "🏷 Изменить ярлык",
                    String::try_from(Callback {
                        query: Option::from(Query::UpdateCategory(UpdateCategory {
                            category_id: self.query.category_id,
                            field: i32::from(Field::Label),
                        })),
                    })
                    .unwrap(),
                ),
            ])
            .append_row(vec![
                InlineKeyboardButton::callback(
                    "🔄 Изменить тип",
                    String::try_from(Callback {
                        query: Option::from(Query::UpdateCategory(UpdateCategory {
                            category_id: self.query.category_id,
                            field: i32::from(Field::Name),
                        })),
                    })
                    .unwrap(),
                ),
                InlineKeyboardButton::callback(
                    format!(
                        "🎯 Изменить {}",
                        match category.direction {
                            CategoryDirection::Expense => {
                                "лимит"
                            }
                            CategoryDirection::Income => {
                                "план"
                            }
                            CategoryDirection::Unspecified => {
                                unreachable!()
                            }
                        }
                    ),
                    String::try_from(Callback {
                        query: Option::from(Query::UpdateCategory(UpdateCategory {
                            category_id: self.query.category_id,
                            field: i32::from(Field::TargetAmount),
                        })),
                    })
                    .unwrap(),
                ),
            ])
            .append_row(vec![InlineKeyboardButton::callback(
                "📅 Изменить регулярность",
                String::try_from(Callback {
                    query: Option::from(Query::UpdateCategory(UpdateCategory {
                        category_id: self.query.category_id,
                        field: i32::from(Field::IsRegular),
                    })),
                })
                .unwrap(),
            )])
            .append_row(vec![
                InlineKeyboardButton::callback(
                    "❌ Удалить",
                    String::try_from(Callback {
                        query: Option::from(Query::DeleteCategory(DeleteCategory {
                            category_id: self.query.category_id,
                        })),
                    })
                    .unwrap(),
                ),
                InlineKeyboardButton::callback(
                    "🔙 Назад",
                    String::try_from(Callback {
                        query: Option::from(Query::ShowCategoriesSettings(
                            self.query.navigation_from.unwrap(),
                        )),
                    })
                    .unwrap(),
                ),
            ])
    }
}
