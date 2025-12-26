use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum CurrencyCode {
    USD,
    EUR,
    JPY,
    RUB,
}

impl CurrencyCode {
    pub fn as_code_str(&self) -> &'static str {
        match self {
            CurrencyCode::USD => "USD",
            CurrencyCode::EUR => "EUR",
            CurrencyCode::JPY => "JPY",
            CurrencyCode::RUB => "RUB",
        }
    }

    pub fn from_code_str(value: &str) -> Option<CurrencyCode> {
        match value {
            "USD" => Some(CurrencyCode::USD),
            "EUR" => Some(CurrencyCode::EUR),
            "JPY" => Some(CurrencyCode::JPY),
            "RUB" => Some(CurrencyCode::RUB),
            _ => None,
        }
    }

    pub fn as_symbol_str(&self) -> &'static str {
        match self {
            CurrencyCode::USD => "$",
            CurrencyCode::EUR => "€",
            CurrencyCode::JPY => "¥",
            CurrencyCode::RUB => "₽",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: i64,
    pub chat_id: i64,
    pub category_id: i64,
    pub amount: i64,
    pub amount_modified: bool,
    pub description: String,
    pub currency_code: CurrencyCode,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
