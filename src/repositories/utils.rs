use chrono::{DateTime, Utc};
use sqlx::types::time::OffsetDateTime;

pub fn convert_offset_to_chrono(offset: OffsetDateTime) -> DateTime<Utc> {
    DateTime::from_timestamp(offset.unix_timestamp(), 0).unwrap_or(DateTime::default())
}