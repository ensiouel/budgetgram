use crate::proto::callback::v1::CategoryDirection;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Category {
    pub id: i64,
    pub chat_id: i64,
    pub name: String,
    pub label: String,
    pub direction: CategoryDirection,
    pub is_regular: bool,
    pub target_amount: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
