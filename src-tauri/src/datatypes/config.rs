use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::datatypes::types::{CompressionAlgorithm, SyncPriority, Metadata};

#[derive(Debug, Serialize, Deserialize)]
pub struct RetentionConfig {
    pub table_name: String,
    #[serde(default)]
    pub compression_enabled: bool,
    #[serde(default)]
    pub compression_algorithm: CompressionAlgorithm,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downsample_after_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downsample_ratio: Option<i32>,
    #[serde(default)]
    pub convert_to_text: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_required_space_mb: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncPriorityConfig {
    pub table_name: String,
    pub priority: SyncPriority,
    #[serde(default = "default_batch_size")]
    pub batch_size: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_delay_seconds: Option<i32>,
    #[serde(default = "default_retry_count")]
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

fn default_batch_size() -> i32 {
    1000
}

fn default_retry_count() -> i32 {
    3
}

impl RetentionConfig {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl SyncPriorityConfig {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
} 