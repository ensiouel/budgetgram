use crate::models::transaction::{CurrencyCode, Transaction};
use crate::repositories::utils;
use sqlx::{FromRow, PgPool, types::time::OffsetDateTime};
use std::sync::Arc;

#[derive(Debug, Clone, FromRow)]
pub struct RawTransaction {
    pub id: i64,
    pub chat_id: i64,
    pub category_id: i64,
    pub amount: i64,
    pub amount_modified: bool,
    pub description: String,
    pub currency_code: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

impl TryFrom<RawTransaction> for Transaction {
    type Error = String;

    fn try_from(raw: RawTransaction) -> Result<Self, Self::Error> {
        Ok(Transaction {
            id: raw.id,
            chat_id: raw.chat_id,
            category_id: raw.category_id,
            amount: raw.amount,
            amount_modified: raw.amount_modified,
            description: raw.description,
            currency_code: CurrencyCode::from_code_str(&raw.currency_code)
                .unwrap_or(CurrencyCode::RUB),
            created_at: utils::convert_offset_to_chrono(raw.created_at),
            updated_at: utils::convert_offset_to_chrono(raw.created_at),
            deleted_at: raw.deleted_at.map(utils::convert_offset_to_chrono),
        })
    }
}

pub type RepositoryError = Box<dyn std::error::Error + Sync + Send>;

#[async_trait::async_trait]
pub trait Repository: Send + Sync {
    async fn create_transaction(&self, transaction: Transaction) -> Result<i64, RepositoryError>;
    async fn get_transaction(&self, chat_id: i64, id: i64) -> Result<Transaction, RepositoryError>;
    async fn update_transaction(&self, transaction: Transaction) -> Result<(), RepositoryError>;
    async fn delete_transaction(&self, chat_id: i64, id: i64) -> Result<(), RepositoryError>;
}

pub struct Transactions {
    db: PgPool,
}

impl Transactions {
    pub fn new(db: PgPool) -> Arc<Self> {
        Arc::new(Self { db })
    }
}

#[async_trait::async_trait]
impl Repository for Transactions {
    async fn create_transaction(&self, transaction: Transaction) -> Result<i64, RepositoryError> {
        let raw = sqlx::query_file!(
            "src/repositories/queries/create_transaction.sql",
            transaction.chat_id,
            transaction.amount,
            transaction.description,
            transaction.currency_code.as_code_str(),
            OffsetDateTime::from_unix_timestamp(transaction.created_at.timestamp()).unwrap(),
            OffsetDateTime::from_unix_timestamp(transaction.updated_at.timestamp()).unwrap()
        )
        .fetch_one(&self.db)
        .await?;

        Ok(raw.id)
    }

    async fn get_transaction(&self, chat_id: i64, id: i64) -> Result<Transaction, RepositoryError> {
        let raw = sqlx::query_file_as!(
            RawTransaction,
            "src/repositories/queries/get_transaction.sql",
            id,
            chat_id,
        )
        .fetch_one(&self.db)
        .await?;

        Transaction::try_from(raw).map_err(|e| e.into())
    }

    async fn update_transaction(&self, transaction: Transaction) -> Result<(), RepositoryError> {
        sqlx::query_file!(
            "src/repositories/queries/update_transaction.sql",
            transaction.id,
            transaction.chat_id,
            transaction.category_id,
            transaction.amount,
            transaction.amount_modified,
            transaction.description,
            transaction.currency_code.as_code_str(),
            OffsetDateTime::from_unix_timestamp(transaction.updated_at.timestamp()).unwrap()
        )
        .fetch_one(&self.db)
        .await?;

        Ok(())
    }

    async fn delete_transaction(&self, chat_id: i64, id: i64) -> Result<(), RepositoryError> {
        sqlx::query_file!(
            "src/repositories/queries/delete_transaction.sql",
            id,
            chat_id,
            OffsetDateTime::now_utc(),
        )
        .fetch_one(&self.db)
        .await?;

        Ok(())
    }
}
