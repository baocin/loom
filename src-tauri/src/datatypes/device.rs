use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::datatypes::types::DeviceType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub device_id: String,
    pub user_id: String,
    pub device_type: DeviceType,
    pub os_type: String,
    pub os_version: String,
    pub app_version: String,
    pub available_sensors: Vec<String>,
    pub capabilities: DeviceCapabilities,
    pub created_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    pub has_camera: bool,
    pub has_microphone: bool,
    pub has_gps: bool,
    pub has_accelerometer: bool,
    pub has_gyroscope: bool,
    pub has_magnetometer: bool,
    pub has_proximity: bool,
    pub has_light: bool,
    pub has_pressure: bool,
    pub has_temperature: bool,
    pub has_humidity: bool,
    pub has_step_counter: bool,
    pub has_heart_rate: bool,
    pub has_ecg: bool,
    pub has_blood_oxygen: bool,
    pub has_stress: bool,
    pub has_compass: bool,
    pub screen_details: ScreenDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenDetails {
    pub width: i32,
    pub height: i32,
    pub density: f32,
    pub refresh_rate: i32,
}

impl Device {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl DeviceCapabilities {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
} 