use crate::handlers::callback;
use crate::proto::callback::v1::callback::Query;
use crate::proto::callback::v1::update_category::Field;
use crate::proto::callback::v1::{
    Callback, CategoryDirection, DeleteCategory, ShowCategorySettings, UpdateCategory,
};
use crate::services;
use std::sync::Arc;
use teloxide::prelude::{CallbackQuery, Message};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub struct MessageBuilder {
    message: Message,
    service: Arc<dyn services::categories::Service>,
    callback: ShowCategorySettings,
}

impl MessageBuilder {
    pub fn new(
        message: Message,
        service: Arc<dyn services::categories::Service>,
        callback: ShowCategorySettings,
    ) -> Self {
        Self {
            message,
            service,
            callback,
        }
    }
}

#[async_trait::async_trait]
impl callback::MessageBuilder for MessageBuilder {
    async fn text(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let category = self
            .service
            .get_category(self.message.chat.id.0, self.callback.category_id)
            .await?;

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

        Ok(format!(
            "⚙️ *Настройки категории*
*Название:* {name}
*Ярлык:* {label}
*Тип:* {direction}
*{target_limit_title}:* {target_limit}
*Постоянный {is_regular_title}:* {is_regular}",
        ))
    }

    async fn reply_markup(
        &self,
    ) -> Result<InlineKeyboardMarkup, Box<dyn std::error::Error + Send + Sync>> {
        let category = self
            .service
            .get_category(self.message.chat.id.0, self.callback.category_id)
            .await
            .unwrap();

        Ok(InlineKeyboardMarkup::default()
            .append_row(vec![
                InlineKeyboardButton::callback(
                    "✏ Изменить название",
                    String::try_from(Callback {
                        query: Option::from(Query::UpdateCategory(UpdateCategory {
                            category_id: self.callback.category_id,
                            field: i32::from(Field::Name),
                        })),
                    })
                    .unwrap(),
                ),
                InlineKeyboardButton::callback(
                    "🏷 Изменить ярлык",
                    String::try_from(Callback {
                        query: Option::from(Query::UpdateCategory(UpdateCategory {
                            category_id: self.callback.category_id,
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
                            category_id: self.callback.category_id,
                            field: i32::from(Field::Direction),
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
                            category_id: self.callback.category_id,
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
                        category_id: self.callback.category_id,
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
                            category_id: self.callback.category_id,
                        })),
                    })
                    .unwrap(),
                ),
                InlineKeyboardButton::callback(
                    "🔙 Назад",
                    String::try_from(Callback {
                        query: Option::from(Query::ShowCategoriesSettings(
                            self.callback.navigated_from.unwrap(),
                        )),
                    })
                    .unwrap(),
                ),
            ]))
    }
}
