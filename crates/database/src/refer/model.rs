use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Refer {
    pub lower: String,            // Address
    pub upper: String,            // Address
    pub timestamp: DateTime<Utc>, // 2024-10-01T04:50:42.849324741Z
}
