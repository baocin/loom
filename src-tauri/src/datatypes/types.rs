use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceType {
    Unknown,
    Headphone,
    Speaker,
    Car,
    Keyboard,
    Mouse,
    Gamepad,
    Watch,
    Phone,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CameraType {
    Unknown,
    Front,
    BackMain,
    BackWide,
    BackTelephoto,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConnectionType {
    Unknown,
    None,
    Wifi,
    Cellular2g,
    Cellular3g,
    Cellular4g,
    Cellular5g,
    Ethernet,
    Vpn,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntityType {
    Face,
    Object,
    Pose,
    Audio,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotePriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SyncPriority {
    Critical,
    High,
    Medium,
    Low,
    Background,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CompressionAlgorithm {
    None,
    Lz4,
    Zstd,
    Gzip,
}

pub type Metadata = HashMap<String, serde_json::Value>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseEntity {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
} 