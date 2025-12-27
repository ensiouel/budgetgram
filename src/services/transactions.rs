use crate::models::transaction::{CurrencyCode, Transaction};
use crate::repositories::transactions::Repository;
use chrono::{DateTime, Utc};
use std::sync::Arc;

pub type ServiceError = Box<dyn std::error::Error + Sync + Send>;

pub struct CreateTransactionRequest {
    pub chat_id: i64,
    pub category_id: Option<i64>,
    pub amount: i64,
    pub description: String,
    pub currency_code: CurrencyCode,
}

pub struct UpdateTransactionRequest {
    pub id: i64,
    pub chat_id: i64,
    pub category_id: Option<i64>,
    pub amount: Option<i64>,
    pub description: Option<String>,
    pub currency_code: Option<CurrencyCode>,
}

#[async_trait::async_trait]
pub trait Service: Send + Sync {
    async fn create_transaction(
        &self,
        request: CreateTransactionRequest,
    ) -> Result<Transaction, ServiceError>;
    async fn update_transaction(
        &self,
        request: UpdateTransactionRequest,
    ) -> Result<Transaction, ServiceError>;
    async fn get_transaction(&self, chat_id: i64, id: i64) -> Result<Transaction, ServiceError>;
    async fn delete_transaction(&self, chat_id: i64, id: i64) -> Result<(), ServiceError>;
}

pub struct Transactions {
    repository: Arc<dyn Repository>,
}

impl Transactions {
    pub fn new(repository: Arc<dyn Repository>) -> Arc<Self> {
        Arc::new(Self { repository })
    }
}

#[async_trait::async_trait]
impl Service for Transactions {
    async fn create_transaction(
        &self,
        request: CreateTransactionRequest,
    ) -> Result<Transaction, ServiceError> {
        let now: DateTime<Utc> = Default::default();
        let mut transaction = Transaction {
            id: -1,
            chat_id: request.chat_id,
            category_id: request.category_id,
            amount: request.amount,
            amount_modified: false,
            description: request.description,
            currency_code: request.currency_code,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };
        transaction.id = self
            .repository
            .create_transaction(transaction.to_owned())
            .await?;

        Ok(transaction)
    }

    async fn update_transaction(
        &self,
        request: UpdateTransactionRequest,
    ) -> Result<Transaction, ServiceError> {
        let mut transaction = self.get_transaction(request.chat_id, request.id).await?;

        if let Some(category_id) = request.category_id {
            transaction.category_id = Some(category_id);
        }
        if let Some(amount) = request.amount {
            transaction.amount = amount;
            transaction.amount_modified = true;
        }
        if let Some(description) = request.description {
            transaction.description = description;
        }
        if let Some(currency_code) = request.currency_code {
            transaction.currency_code = currency_code;
        }

        transaction.updated_at = Utc::now();
        self.repository
            .update_transaction(transaction.to_owned())
            .await?;

        Ok(transaction)
    }

    async fn get_transaction(&self, chat_id: i64, id: i64) -> Result<Transaction, ServiceError> {
        self.repository.get_transaction(chat_id, id).await
    }

    async fn delete_transaction(&self, chat_id: i64, id: i64) -> Result<(), ServiceError> {
        self.repository.delete_transaction(chat_id, id).await
    }
}
