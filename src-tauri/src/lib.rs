mod networking;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use chrono::{DateTime, Duration, Utc};
use serde_json::{json, Value};
use crate::db::Database;
use std::path::Path;

#[tauri::command]
fn get_last_24h_events() -> Result<Value, String> {
    // Initialize DB connection
    let db_path = Path::new("data.db"); // You may need to adjust this path
    let db = Database::new(db_path).map_err(|e| e.to_string())?;

    let end = Utc::now();
    let start = end - Duration::hours(24);

    // Get data from each sensor table
    let accelerometer = db.get_accelerometer_data("*", start, end)
        .map_err(|e| e.to_string())?;
    
    // Build response JSON
    let events = json!({
        "timestamp": end.to_rfc3339(),
        "timeRange": {
            "start": start.to_rfc3339(),
            "end": end.to_rfc3339()
        },
        "sensorData": {
            "accelerometer": accelerometer,
            // Add other sensor data here as they are implemented in db.rs
            // "gyroscope": gyroscope_data,
            // "magnetometer": magnetometer_data,
            // etc.
        },
        "metadata": {
            "version": "1.0",
            "dataPoints": accelerometer.len()
        }
    });

    Ok(events)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Start the networking service
    crate::networking::start_networking_service();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub mod datatypes;
pub mod db;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
