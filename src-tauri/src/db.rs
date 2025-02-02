use std::path::Path;
use duckdb::{Connection, Result};
use chrono::{DateTime, Utc};
use crate::datatypes::{
    sensor::*,
    device::*,
};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // Enable extensions via SQL
        conn.execute_batch("
            INSTALL vss;
            LOAD vss;
            INSTALL json;
            LOAD json;
        ")?;

        // Initialize schema from init.sql
        let init_sql = include_str!("../db-setup/init.sql");
        conn.execute_batch(init_sql)?;

        Ok(Self { conn })
    }

    // Device methods
    pub fn insert_device(&self, device: &Device) -> Result<()> {
        // let capabilities_json = serde_json::to_string(&device.capabilities)
        //     .map_err(|e| duckdb::Error::from(std::io::Error::new(std::io::ErrorKind::Other, format!("JSON serialization error: {}", e))))?;
        self.conn.execute(
            "INSERT INTO devices (
                device_id, user_id, device_type, os_type, os_version, 
                app_version, available_sensors, created_at, last_seen
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            &[
                &device.device_id,
                &device.user_id, 
                &device.device_type.to_string(),
                &device.os_type,
                &device.os_version,
                &device.app_version,
                &serde_json::to_string(&device.available_sensors).unwrap(),
                &device.created_at.to_string(),
                &device.last_seen.to_string(),
            ],
        )?;
        Ok(())
    }

    pub fn get_device(&self, device_id: &str) -> Result<Device> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM devices WHERE device_id = ?"
        )?;
        
        let device = stmt.query_row([device_id], |row| {
            Ok(Device {
                device_id: row.get(0)?,
                user_id: row.get(1)?,
                device_type: serde_json::from_str(&row.get::<_,String>(2)?).unwrap(),
                os_type: row.get(3)?,
                os_version: row.get(4)?,
                app_version: row.get(5)?,
                available_sensors: serde_json::from_str(&row.get::<_,String>(6)?).unwrap(),
                capabilities: serde_json::from_str(&row.get::<_,String>(7)?).unwrap(),
                created_at: row.get::<_,String>(8)?.parse::<DateTime<Utc>>().unwrap(),
                last_seen: row.get::<_,String>(9)?.parse::<DateTime<Utc>>().unwrap(),
                updated_at: row.get::<_,String>(10)?.parse::<DateTime<Utc>>().unwrap(),
            })
        })?;

        Ok(device)
    }

    // Sensor data methods
    pub fn insert_accelerometer_data(&self, data: &AccelerometerData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO accelerometer_data (
                timestamp, device_id, x, y, z, accuracy, metadata
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.x.to_string(),
                &data.y.to_string(), 
                &data.z.to_string(),
                &data.accuracy.map(|f| f.to_string()).unwrap_or_default(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_accelerometer_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<AccelerometerData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM accelerometer_data 
             WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(AccelerometerData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                x: row.get(2)?,
                y: row.get(3)?,
                z: row.get(4)?,
                accuracy: row.get(5)?,
                metadata: row.get::<_,Option<String>>(6)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        let mut data = Vec::new();
        for row in rows {
            data.push(row?);
        }
        
        Ok(data)
    }

    // Add similar methods for other sensor types...
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use chrono::Utc;

    #[test]
    fn test_device_crud() -> Result<()> {
        let dir = tempdir()?;
        let db_path = dir.path().join("test.db");
        let db = Database::new(&db_path)?;

        let device = Device {
            device_id: "test_device".to_string(),
            user_id: "test_user".to_string(),
            device_type: crate::datatypes::types::DeviceType::Smartphone,
            os_type: "Android".to_string(),
            os_version: "11".to_string(),
            app_version: "1.0".to_string(),
            available_sensors: vec!["accelerometer".to_string()],
            capabilities: DeviceCapabilities {
                has_camera: true,
                has_microphone: true,
                has_gps: true,
                has_accelerometer: true,
                has_gyroscope: false,
                has_magnetometer: false,
                has_proximity: true,
                has_light: true,
                has_pressure: false,
                has_temperature: false,
                has_humidity: false,
                has_step_counter: true,
                has_heart_rate: false,
                has_ecg: false,
                has_blood_oxygen: false,
                has_stress: false,
                has_compass: false,
                screen_details: ScreenDetails {
                    width: 1080,
                    height: 2400,
                    density: 2.75,
                    refresh_rate: 60,
                },
            },
            created_at: Utc::now(),
            last_seen: Utc::now(),
            updated_at: Utc::now(),
        };

        db.insert_device(&device)?;
        let retrieved = db.get_device("test_device")?;
        assert_eq!(device.device_id, retrieved.device_id);

        Ok(())
    }

    #[test]
    fn test_sensor_data() -> Result<()> {
        let dir = tempdir()?;
        let db_path = dir.path().join("test.db");
        let db = Database::new(&db_path)?;

        let data = AccelerometerData {
            timestamp: Utc::now(),
            device_id: "test_device".to_string(),
            x: 1.0,
            y: 2.0,
            z: 3.0,
            accuracy: Some(0.1),
            metadata: None,
        };

        db.insert_accelerometer_data(&data)?;
        
        let start = Utc::now() - chrono::Duration::hours(1);
        let end = Utc::now() + chrono::Duration::hours(1);
        
        let retrieved = db.get_accelerometer_data("test_device", start, end)?;
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].x, data.x);
        assert_eq!(retrieved[0].y, data.y);
        assert_eq!(retrieved[0].z, data.z);

        Ok(())
    }
}

