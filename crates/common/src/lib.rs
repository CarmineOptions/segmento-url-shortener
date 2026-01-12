use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod validation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub id: i64,
    pub short_code: String,
    pub target_url: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLink<'a> {
    pub target_url: &'a str,
}
