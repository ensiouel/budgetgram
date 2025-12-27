use crate::handlers::callback;
use crate::proto::callback::v1::callback::Query;
use crate::proto::callback::v1::{
    ApproveTransactionCategory, Callback, CategoryDirection, CreateTransaction,
};
use crate::services;
use std::sync::Arc;
use teloxide::prelude::{ChatId, Message};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub struct MessageBuilder {
    chat_id: ChatId,
    categories_service: Arc<dyn services::categories::Service>,
    transactions_service: Arc<dyn services::transactions::Service>,
    callback: CreateTransaction,
}

impl MessageBuilder {
    pub fn new(
        chat_id: ChatId,
        categories_service: Arc<dyn services::categories::Service>,
        transactions_service: Arc<dyn services::transactions::Service>,
        callback: CreateTransaction,
    ) -> Self {
        Self {
            chat_id,
            categories_service,
            transactions_service,
            callback,
        }
    }
}

#[async_trait::async_trait]
impl callback::MessageBuilder for MessageBuilder {
    async fn text(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let transaction = self
            .transactions_service
            .get_transaction(self.chat_id.0, self.callback.transaction_id)
            .await?;

        Ok(format!(
            "Выберите, в какую категорию добавить {} {}",
            transaction.amount,
            transaction.currency_code.as_symbol_str()
        ))
    }

    async fn reply_markup(
        &self,
    ) -> Result<InlineKeyboardMarkup, Box<dyn std::error::Error + Send + Sync>> {
        let categories = self
            .categories_service
            .select_categories(
                self.chat_id.0,
                CategoryDirection::try_from(self.callback.category_direction).unwrap(),
            )
            .await?;

        let mut reply_markup = InlineKeyboardMarkup::default();
        for chunk in categories.chunks(if self.callback.is_short_mode { 5 } else { 3 }) {
            let row: Vec<InlineKeyboardButton> = chunk
                .iter()
                .map(|category| {
                    InlineKeyboardButton::callback(
                        if self.callback.is_short_mode {
                            format!("{}", category.label)
                        } else {
                            format!("{} {}", category.label, category.name)
                        },
                        String::try_from(Callback {
                            query: Option::from(Query::ApproveTransactionCategory(
                                ApproveTransactionCategory {
                                    transaction_id: self.callback.transaction_id,
                                    category_id: category.id,
                                },
                            )),
                        })
                        .unwrap(),
                    )
                })
                .collect();

            reply_markup = reply_markup.append_row(row);
        }

        reply_markup = reply_markup.append_row(vec![
            InlineKeyboardButton::callback(
                match CategoryDirection::try_from(self.callback.category_direction)? {
                    CategoryDirection::Expense => "📈 Доходы",
                    CategoryDirection::Income => "📉 Расходы",
                    CategoryDirection::Unspecified => unreachable!(),
                },
                String::try_from(Callback {
                    query: Option::from(Query::CreateTransaction(CreateTransaction {
                        transaction_id: self.callback.transaction_id,
                        category_direction: match CategoryDirection::try_from(
                            self.callback.category_direction,
                        )? {
                            CategoryDirection::Expense => CategoryDirection::Income.into(),
                            CategoryDirection::Income => CategoryDirection::Expense.into(),
                            CategoryDirection::Unspecified => unreachable!(),
                        },
                        is_short_mode: self.callback.is_short_mode,
                    })),
                })
                .unwrap(),
            ),
            InlineKeyboardButton::callback(
                match self.callback.is_short_mode {
                    true => "📔 Подробный вид",
                    false => "📔 Компактный вид",
                },
                String::try_from(Callback {
                    query: Option::from(Query::CreateTransaction(CreateTransaction {
                        transaction_id: self.callback.transaction_id,
                        category_direction: self.callback.category_direction,
                        is_short_mode: match self.callback.is_short_mode {
                            true => false,
                            false => true,
                        },
                    })),
                })
                .unwrap(),
            ),
        ]);

        Ok(reply_markup)
    }
}
