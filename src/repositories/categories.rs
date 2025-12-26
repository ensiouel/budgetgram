use crate::models::category::Category;
use crate::proto::callback::v1::CategoryDirection;
use sqlx::{FromRow, PgPool, types::time::OffsetDateTime};
use std::sync::Arc;
use crate::repositories::utils;

#[derive(Debug, Clone, FromRow)]
pub struct RawCategory {
    pub id: i64,
    pub chat_id: i64,
    pub name: String,
    pub label: String,
    pub direction: i32,
    pub is_regular: bool,
    pub target_amount: Option<i64>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

impl TryFrom<RawCategory> for Category {
    type Error = String;

    fn try_from(raw: RawCategory) -> Result<Self, Self::Error> {
        Ok(Category {
            id: raw.id,
            chat_id: raw.chat_id,
            name: raw.name,
            label: raw.label,
            direction: CategoryDirection::try_from(raw.direction)
                .unwrap_or(CategoryDirection::Unspecified),
            is_regular: raw.is_regular,
            target_amount: raw.target_amount,
            created_at: utils::convert_offset_to_chrono(raw.created_at),
            updated_at: utils::convert_offset_to_chrono(raw.updated_at),
            deleted_at: raw.deleted_at.map(utils::convert_offset_to_chrono),
        })
    }
}

pub type RepositoryError = Box<dyn std::error::Error + Sync + Send>;

#[async_trait::async_trait]
pub trait Repository: Send + Sync {
    async fn create_category(&self, category: Category) -> Result<i64, RepositoryError>;
    async fn update_category(&self, category: Category) -> Result<(), RepositoryError>;
    async fn get_category(&self, chat_id: i64, id: i64) -> Result<Category, RepositoryError>;
    async fn select_categories(
        &self,
        chat_id: i64,
        direction: CategoryDirection,
    ) -> Result<Vec<Category>, RepositoryError>;
    async fn delete_category(&self, chat_id: i64, id: i64) -> Result<(), RepositoryError>;
}

pub struct Categories {
    db: PgPool,
}

impl Categories {
    pub fn new(db: PgPool) -> Arc<Self> {
        Arc::new(Self { db })
    }
}

#[async_trait::async_trait]
impl Repository for Categories {
    async fn create_category(&self, category: Category) -> Result<i64, RepositoryError> {
        let raw = sqlx::query_file!(
            "src/repositories/queries/create_category.sql",
            category.chat_id,
            category.name,
            category.label,
            i32::try_from(category.direction).unwrap(),
            category.is_regular,
            category.target_amount,
            OffsetDateTime::from_unix_timestamp(category.created_at.timestamp()).unwrap(),
            OffsetDateTime::from_unix_timestamp(category.updated_at.timestamp()).unwrap()
        )
        .fetch_one(&self.db)
        .await?;

        Ok(raw.id)
    }

    async fn update_category(&self, category: Category) -> Result<(), RepositoryError> {
        sqlx::query_file!(
            "src/repositories/queries/update_category.sql",
            category.id,
            category.chat_id,
            category.name,
            category.label,
            i32::try_from(category.direction).unwrap(),
            category.is_regular,
            category.target_amount,
            OffsetDateTime::from_unix_timestamp(category.updated_at.timestamp()).unwrap()
        )
        .fetch_one(&self.db)
        .await?;

        Ok(())
    }

    async fn get_category(&self, chat_id: i64, id: i64) -> Result<Category, RepositoryError> {
        let raw = sqlx::query_file_as!(
            RawCategory,
            "src/repositories/queries/get_category.sql",
            id,
            chat_id,
        )
        .fetch_one(&self.db)
        .await?;

        Category::try_from(raw).map_err(|e| e.into())
    }

    async fn select_categories(
        &self,
        chat_id: i64,
        direction: CategoryDirection,
    ) -> Result<Vec<Category>, RepositoryError> {
        let raws = sqlx::query_file_as!(
            RawCategory,
            "src/repositories/queries/select_categories.sql",
            chat_id,
            i32::try_from(direction).unwrap()
        )
        .fetch_all(&self.db)
        .await?;

        let categories: Result<Vec<Category>, _> = raws
            .into_iter()
            .map(|raw| Category::try_from(raw.to_owned()))
            .collect();

        categories.map_err(|e| e.into())
    }

    async fn delete_category(&self, chat_id: i64, id: i64) -> Result<(), RepositoryError> {
        sqlx::query_file!(
            "src/repositories/queries/delete_category.sql",
            id,
            chat_id,
            OffsetDateTime::now_utc(),
        )
        .fetch_one(&self.db)
        .await?;

        Ok(())
    }
}
