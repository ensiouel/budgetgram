use crate::models::category::Category;
use crate::proto::callback::v1::CategoryDirection;
use crate::repositories::categories::Repository;
use chrono::{DateTime, Utc};
use std::sync::Arc;

pub type ServiceError = Box<dyn std::error::Error + Sync + Send>;

pub struct CreateCategoryRequest {
    pub chat_id: i64,
    pub name: String,
    pub label: String,
    pub direction: CategoryDirection,
    pub is_regular: bool,
    pub target_amount: Option<i64>,
}

pub struct UpdateCategoryRequest {
    pub id: i64,
    pub chat_id: i64,
    pub name: Option<String>,
    pub label: Option<String>,
    pub direction: Option<CategoryDirection>,
    pub is_regular: Option<bool>,
    pub target_amount: Option<i64>,
}

#[async_trait::async_trait]
pub trait Service: Send + Sync {
    async fn create_category(
        &self,
        request: CreateCategoryRequest,
    ) -> Result<Category, ServiceError>;
    async fn update_category(
        &self,
        request: UpdateCategoryRequest,
    ) -> Result<Category, ServiceError>;
    async fn get_category(&self, chat_id: i64, id: i64) -> Result<Category, ServiceError>;
    async fn select_categories(
        &self,
        chat_id: i64,
        direction: CategoryDirection,
    ) -> Result<Vec<Category>, ServiceError>;
    async fn delete_category(&self, chat_id: i64, id: i64) -> Result<(), ServiceError>;
}

pub struct Categories {
    repository: Arc<dyn Repository>,
}

impl Categories {
    pub fn new(repository: Arc<dyn Repository>) -> Arc<Self> {
        Arc::new(Self { repository })
    }
}

#[async_trait::async_trait]
impl Service for Categories {
    async fn create_category(
        &self,
        request: CreateCategoryRequest,
    ) -> Result<Category, ServiceError> {
        let now: DateTime<Utc> = Default::default();
        let mut category = Category {
            id: -1,
            chat_id: request.chat_id,
            name: request.name,
            label: request.label,
            direction: request.direction,
            is_regular: request.is_regular,
            target_amount: request.target_amount,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };
        category.id = self.repository.create_category(category.to_owned()).await?;

        Ok(category)
    }

    async fn update_category(
        &self,
        request: UpdateCategoryRequest,
    ) -> Result<Category, ServiceError> {
        let mut category = self.get_category(request.chat_id, request.id).await?;

        if let Some(name) = request.name {
            category.name = name;
        }
        if let Some(label) = request.label {
            category.label = label;
        }
        if let Some(direction) = request.direction {
            category.direction = direction;
        }
        if let Some(is_regular) = request.is_regular {
            category.is_regular = is_regular;
        }
        if let Some(target_amount) = request.target_amount {
            category.target_amount = Some(target_amount);
        }

        self.repository.update_category(category.to_owned()).await?;

        Ok(category)
    }

    async fn get_category(&self, chat_id: i64, id: i64) -> Result<Category, ServiceError> {
        self.repository.get_category(chat_id, id).await
    }

    async fn select_categories(
        &self,
        chat_id: i64,
        direction: CategoryDirection,
    ) -> Result<Vec<Category>, ServiceError> {
        self.repository.select_categories(chat_id, direction).await
    }

    async fn delete_category(&self, chat_id: i64, id: i64) -> Result<(), ServiceError> {
        self.repository.delete_category(chat_id, id).await
    }
}
