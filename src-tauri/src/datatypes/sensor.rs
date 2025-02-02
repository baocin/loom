use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::datatypes::types::{CameraType, ConnectionType, Metadata};

// Motion sensors
#[derive(Debug, Serialize, Deserialize)]
pub struct AccelerometerData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GyroscopeData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MagnetometerData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

// Location sensors
#[derive(Debug, Serialize, Deserialize)]
pub struct GPSData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearing: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satellites: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

// Health sensors
#[derive(Debug, Serialize, Deserialize)]
pub struct HeartRateData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub bpm: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rr_intervals: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ECGData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub voltage: Vec<f32>,
    pub time: Vec<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rhythm_classification: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heart_rate: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BloodOxygenData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub spo2: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_values: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StressData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub stress_score: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stress_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrv: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

// Environmental sensors
#[derive(Debug, Serialize, Deserialize)]
pub struct ProximityData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub distance: f32,
    pub near: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LightData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub lux: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PressureData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub hectopascals: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemperatureData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub celsius: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HumidityData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub percentage: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

// Activity sensors
#[derive(Debug, Serialize, Deserialize)]
pub struct StepCountData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub steps: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

// Audio sensors
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioLevelData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub db: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_db: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

// System sensors
#[derive(Debug, Serialize, Deserialize)]
pub struct BatteryData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub percentage: i32,
    pub charging: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voltage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub connection_type: ConnectionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roaming: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular_technology: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_metered: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_servers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenStateData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub screen_on: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

// TodosData
#[derive(Debug, Serialize, Deserialize)]
pub struct TodosData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub todo_id: String,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub priority: Option<i32>,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

// Camera and vision sensors
#[derive(Debug, Serialize, Deserialize)]
pub struct CameraData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub camera_type: CameraType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_detection: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_distance: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom_level: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppUsageData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub package_name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub activity_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WifiData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub ssid: String,
    pub bssid: String,
    pub strength: i32,
    pub frequency: i32,
    pub ip_address: String,
    pub link_speed: i32,
    pub security_type: String,
    pub is_5ghz: bool,
    pub is_6ghz: bool,
    pub is_passpoint: bool,
    pub is_restricted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nearby_networks: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallLogData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub call_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    pub duration_seconds: i32,
    pub is_missed: bool,
    pub is_blocked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_slot: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GpsData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearing: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satellites: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationData {
    pub timestamp: DateTime<Utc>,
    pub device_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

// Implement JSON conversion for all sensor types
macro_rules! impl_json_conversion {
    ($($t:ty),*) => {
        $(
            impl $t {
                pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
                    serde_json::from_str(json)
                }

                pub fn to_json(&self) -> Result<String, serde_json::Error> {
                    serde_json::to_string(self)
                }
            }
        )*
    };
}

impl_json_conversion!(
    AccelerometerData, GyroscopeData, MagnetometerData,
    GPSData, HeartRateData, ECGData, BloodOxygenData,
    StressData, ProximityData, LightData, PressureData,
    TemperatureData, HumidityData, StepCountData,
    AudioLevelData, BatteryData, NetworkData,
    ScreenStateData, CameraData, AppUsageData, WifiData
); 