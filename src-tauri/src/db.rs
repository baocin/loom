use std::path::Path;
use duckdb::{Connection, Result};
use chrono::{DateTime, Utc};
use crate::datatypes::{
    sensor::{AccelerometerData, GyroscopeData, MagnetometerData, GpsData, HeartRateData, ProximityData, LightData, PressureData, TemperatureData, HumidityData, StepCountData, AudioLevelData, BatteryData, NetworkData, ScreenStateData, NotificationData, AppUsageData, WifiData, CallLogData, TodosData},
    device::*,
    user::*,
    types::ConnectionType,
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

    // User methods
    pub fn insert_user(&self, user: &User) -> Result<()> {
        self.conn.execute(
            "INSERT INTO users (
                id, email, name, encrypted_password, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?)",
            &[
                &user.id,
                &user.email,
                &user.name.as_deref().unwrap_or_default().to_string(),
                &user.encrypted_password,
                &user.created_at.to_string(),
                &user.updated_at.to_string(),
            ],
        )?;
        Ok(())
    }

    pub fn get_user(&self, id: &str) -> Result<User> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM users WHERE id = ?"
        )?;
        
        let user = stmt.query_row([id], |row| {
            Ok(User {
                id: row.get(0)?,
                email: row.get(1)?,
                name: row.get(2)?,
                encrypted_password: row.get(3)?,
                created_at: row.get::<_,String>(4)?.parse::<DateTime<Utc>>().unwrap(),
                updated_at: row.get::<_,String>(5)?.parse::<DateTime<Utc>>().unwrap(),
            })
        })?;

        Ok(user)
    }

    // Device methods
    pub fn insert_device(&self, device: &Device) -> Result<()> {
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

    pub fn insert_gyroscope_data(&self, data: &GyroscopeData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO gyroscope_data (
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

    pub fn get_gyroscope_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<GyroscopeData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM gyroscope_data 
             WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(GyroscopeData {
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

    pub fn insert_magnetometer_data(&self, data: &MagnetometerData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO magnetometer_data (
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

    pub fn get_magnetometer_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<MagnetometerData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM magnetometer_data 
             WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(MagnetometerData {
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

    pub fn insert_gps_data(&self, data: &GpsData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO gps_data (
                timestamp, device_id, latitude, longitude, altitude, 
                accuracy, speed, bearing, satellites, provider, metadata
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.latitude.to_string(),
                &data.longitude.to_string(),
                &data.altitude.map(|a| a.to_string()).unwrap_or_default(),
                &data.accuracy.map(|a| a.to_string()).unwrap_or_default(),
                &data.speed.map(|s| s.to_string()).unwrap_or_default(),
                &data.bearing.map(|b| b.to_string()).unwrap_or_default(),
                &data.satellites.map(|s| s.to_string()).unwrap_or_default(),
                &data.provider.as_deref().unwrap_or_default().to_string(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_gps_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<GpsData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM gps_data 
             WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(GpsData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                latitude: row.get(2)?,
                longitude: row.get(3)?,
                altitude: row.get(4)?,
                accuracy: row.get(5)?,
                speed: row.get(6)?,
                bearing: row.get(7)?,
                satellites: row.get(8)?,
                provider: row.get(9)?,
                metadata: row.get::<_,Option<String>>(10)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        let mut data = Vec::new();
        for row in rows {
            data.push(row?);
        }
        Ok(data)
    }

    /* Heart Rate Data */
    pub fn insert_heart_rate_data(&self, data: &HeartRateData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO heart_rate_data (
                timestamp, device_id, bpm, confidence, rr_intervals, metadata
            ) VALUES (?, ?, ?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.bpm.to_string(),
                &data.confidence.map(|c| c.to_string()).unwrap_or_default(),
                &serde_json::to_string(&data.rr_intervals).unwrap(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_heart_rate_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<HeartRateData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM heart_rate_data 
             WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(HeartRateData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                bpm: row.get(2)?,
                confidence: row.get(3)?,
                rr_intervals: serde_json::from_str(&row.get::<_,String>(4)?).unwrap(),
                metadata: row.get::<_,Option<String>>(5)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        let mut data = Vec::new();
        for row in rows {
            data.push(row?);
        }
        Ok(data)
    }

    /* Proximity Data */
    // pub fn insert_proximity_data(&self, data: &ProximityData) -> Result<()> {
    //     self.conn.execute(
    //         "INSERT INTO proximity_data (
    //             timestamp, device_id, distance, near, metadata
    //         ) VALUES (?, ?, ?, ?, ?)",
    //         &[
    //             &data.timestamp.to_string(),
    //             &data.device_id,
    //             &data.distance.map(|d| d.to_string()).unwrap_or_default(),
    //             &data.near.map(|n| n.to_string()).unwrap_or_default(),
    //             &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
    //         ],
    //     )?;
    //     Ok(())
    // }

    pub fn get_proximity_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<ProximityData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM proximity_data 
             WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(ProximityData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                distance: row.get(2)?,
                near: row.get(3)?,
                metadata: row.get::<_,Option<String>>(4)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        let mut data = Vec::new();
        for row in rows {
            data.push(row?);
        }
        Ok(data)
    }

    pub fn insert_light_data(&self, data: &LightData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO light_data (timestamp, device_id, lux, metadata) VALUES (?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.lux.to_string(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_light_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<LightData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM light_data WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(LightData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                lux: row.get(2)?,
                metadata: row.get::<_,Option<String>>(3)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        Ok(rows.collect::<Result<_, _>>()?)
    }

    pub fn insert_pressure_data(&self, data: &PressureData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO pressure_data (timestamp, device_id, hectopascals, metadata) VALUES (?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.hectopascals.to_string(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_pressure_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<PressureData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM pressure_data WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(PressureData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                hectopascals: row.get(2)?,
                metadata: row.get::<_,Option<String>>(3)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        Ok(rows.collect::<Result<_, _>>()?)
    }

    /* Temperature Data */
    pub fn insert_temperature_data(&self, data: &TemperatureData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO temperature_data (timestamp, device_id, celsius, metadata) VALUES (?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.celsius.to_string(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_temperature_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<TemperatureData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM temperature_data WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(TemperatureData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                celsius: row.get(2)?,
                metadata: row.get::<_,Option<String>>(3)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        Ok(rows.collect::<Result<_, _>>()?)
    }

    /* Humidity Data */
    pub fn insert_humidity_data(&self, data: &HumidityData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO humidity_data (timestamp, device_id, percentage, metadata) VALUES (?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.percentage.to_string(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_humidity_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<HumidityData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM humidity_data WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(HumidityData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                percentage: row.get(2)?,
                metadata: row.get::<_,Option<String>>(3)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        Ok(rows.collect::<Result<_, _>>()?)
    }

    /* Step Count Data */
    pub fn insert_step_count_data(&self, data: &StepCountData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO step_count_data (timestamp, device_id, steps, activity_type, confidence, metadata) VALUES (?, ?, ?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.steps.to_string(),
                &data.activity_type.as_deref().unwrap_or_default().to_string(),
                &data.confidence.map(|c| c.to_string()).unwrap_or_default(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_step_count_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<StepCountData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM step_count_data WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(StepCountData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                steps: row.get(2)?,
                activity_type: row.get(3)?,
                confidence: row.get(4)?,
                metadata: row.get::<_,Option<String>>(5)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        Ok(rows.collect::<Result<_, _>>()?)
    }

    /* Call Log Data */
    pub fn insert_call_log_data(&self, data: &CallLogData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO call_log_data (
                timestamp, device_id, call_type, phone_number, contact_name, 
                duration_seconds, is_missed, is_blocked, sim_slot, metadata
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.call_type,
                &data.phone_number.as_deref().unwrap_or_default().to_string(),
                &data.contact_name.as_deref().unwrap_or_default().to_string(),
                &data.duration_seconds.to_string(),
                &data.is_missed.to_string(),
                &data.is_blocked.to_string(),
                &data.sim_slot.map(|s| s.to_string()).unwrap_or_default(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_call_log_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<CallLogData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM call_log_data WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(CallLogData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                call_type: row.get(2)?,
                phone_number: row.get(3)?,
                contact_name: row.get(4)?,
                duration_seconds: row.get(5)?,
                is_missed: row.get(6)?,
                is_blocked: row.get(7)?,
                sim_slot: row.get(8)?,
                metadata: row.get::<_,Option<String>>(9)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        Ok(rows.collect::<Result<_, _>>()?)
    }

    /* Todos Data */
    pub fn insert_todos_data(&self, data: &TodosData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO todos_data (
                timestamp, device_id, todo_id, title, description, 
                due_date, completed, completed_at, priority, tags, metadata
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.todo_id,
                &data.title,
                &data.description.as_deref().unwrap_or_default().to_string(),
                &data.due_date.map(|d| d.to_string()).unwrap_or_default(),
                &data.completed.to_string(),
                &data.completed_at.map(|c| c.to_string()).unwrap_or_default(),
                &data.priority.map(|p| p.to_string()).unwrap_or_default(),
                &serde_json::to_string(&data.tags).unwrap(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_todos_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<TodosData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM todos_data WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(TodosData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                todo_id: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
                due_date: row.get::<_,Option<String>>(5)?.map(|s| s.parse::<DateTime<Utc>>().unwrap()),
                completed: row.get(6)?,
                completed_at: row.get::<_,Option<String>>(7)?.map(|s| s.parse::<DateTime<Utc>>().unwrap()),
                priority: row.get(8)?,
                tags: serde_json::from_str(&row.get::<_,String>(9)?).unwrap(),
                metadata: row.get::<_,Option<String>>(10)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        Ok(rows.collect::<Result<_, _>>()?)
    }

     /* Audio Level Data */
     pub fn insert_audio_level_data(&self, data: &AudioLevelData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO audio_level_data (timestamp, device_id, db, peak_db, volume, metadata) VALUES (?, ?, ?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.db.to_string(),
                &data.peak_db.map(|p| p.to_string()).unwrap_or_default(),
                &data.volume.map(|v| v.to_string()).unwrap_or_default(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_audio_level_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<AudioLevelData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM audio_level_data WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(AudioLevelData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                db: row.get(2)?,
                peak_db: row.get(3)?,
                volume: row.get(4)?,
                metadata: row.get::<_,Option<String>>(5)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        Ok(rows.collect::<Result<_, _>>()?)
    }

    /* Battery Data */
    pub fn insert_battery_data(&self, data: &BatteryData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO battery_data (
                timestamp, device_id, percentage, charging, power_source, 
                temperature, voltage, current, metadata
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.percentage.to_string(),
                &data.charging.to_string(),
                &data.power_source.as_deref().unwrap_or_default().to_string(),
                &data.temperature.map(|t| t.to_string()).unwrap_or_default(),
                &data.voltage.map(|v| v.to_string()).unwrap_or_default(),
                &data.current.map(|c| c.to_string()).unwrap_or_default(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_battery_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<BatteryData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM battery_data WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(BatteryData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                percentage: row.get(2)?,
                charging: row.get(3)?,
                power_source: row.get(4)?,
                temperature: row.get(5)?,
                voltage: row.get(6)?,
                current: row.get(7)?,
                metadata: row.get::<_,Option<String>>(8)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        Ok(rows.collect::<Result<_, _>>()?)
    }

    /* Network Data */
    pub fn insert_network_data(&self, data: &NetworkData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO network_data (
                timestamp, device_id, type, state, strength, carrier, 
                roaming, cellular_technology, is_metered, dns_servers, 
                gateway, metadata
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.connection_type.to_string(),
                &data.state.as_deref().unwrap_or_default().to_string(),
                &data.strength.map(|s| s.to_string()).unwrap_or_default(),
                &data.carrier.as_deref().unwrap_or_default().to_string(),
                &data.roaming.map(|r| r.to_string()).unwrap_or_default(),
                &data.cellular_technology.as_deref().unwrap_or_default().to_string(),
                &data.is_metered.map(|m| m.to_string()).unwrap_or_default(),
                &serde_json::to_string(&data.dns_servers).unwrap(),
                &data.gateway.as_deref().unwrap_or_default().to_string(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    // pub fn get_network_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<NetworkData>> {
    //     let mut stmt = self.conn.prepare(
    //         "SELECT * FROM network_data WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
    //     )?;

    //     let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
    //         Ok(NetworkData {
    //             timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
    //             device_id: row.get(1)?,
    //             connection_type: row.get::<_,String>(2)?.parse().unwrap(),
    //             state: row.get(3)?,
    //             strength: row.get(4)?,
    //             carrier: row.get(5)?,
    //             roaming: row.get(6)?,
    //             cellular_technology: row.get(7)?,
    //             is_metered: row.get(8)?,
    //             dns_servers: serde_json::from_str(&row.get::<_,String>(9)?).unwrap(),
    //             gateway: row.get(10)?,
    //             metadata: row.get::<_,Option<String>>(11)?.map(|s| serde_json::from_str(&s).unwrap()),
    //         })
    //     })?;

    //     Ok(rows.collect::<Result<_, _>>()?)
    // }

    /* Screen State Data */
    pub fn insert_screen_state_data(&self, data: &ScreenStateData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO screen_state_data (timestamp, device_id, screen_on, brightness, orientation, metadata) VALUES (?, ?, ?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.screen_on.to_string(),
                &data.brightness.map(|b| b.to_string()).unwrap_or_default(),
                &data.orientation.as_deref().unwrap_or_default().to_string(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_screen_state_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<ScreenStateData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM screen_state_data WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(ScreenStateData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                screen_on: row.get(2)?,
                brightness: row.get(3)?,
                orientation: row.get(4)?,
                metadata: row.get::<_,Option<String>>(5)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        Ok(rows.collect::<Result<_, _>>()?)
    }

    /* Notification Data */
    pub fn insert_notification_data(&self, data: &NotificationData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO notification_data (
                timestamp, device_id, package_name, title, priority, 
                category, posted_at, removed_at, metadata
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.package_name.as_deref().unwrap_or_default().to_string(),
                &data.package_name.as_deref().unwrap_or_default().to_string(),
                &data.title.as_deref().unwrap_or_default().to_string(),
                &data.priority.map(|p| p.to_string()).unwrap_or_default(),
                &data.category.as_deref().unwrap_or_default().to_string(),
                &data.posted_at.map(|p| p.to_string()).unwrap_or_default(),
                &data.removed_at.map(|r| r.to_string()).unwrap_or_default(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_notification_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<NotificationData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM notification_data WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(NotificationData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                package_name: row.get(2)?,
                title: row.get(3)?,
                priority: row.get(4)?,
                category: row.get(5)?,
                posted_at: row.get::<_,Option<String>>(6)?.map(|s| s.parse::<DateTime<Utc>>().unwrap()),
                removed_at: row.get::<_,Option<String>>(7)?.map(|s| s.parse::<DateTime<Utc>>().unwrap()),
                metadata: row.get::<_,Option<String>>(8)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        Ok(rows.collect::<Result<_, _>>()?)
    }

    /* App Usage Data */
    pub fn insert_app_usage_data(&self, data: &AppUsageData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO app_usage_data (
                timestamp, device_id, package_name, start_time, end_time, activity_type, metadata
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.package_name,
                &data.start_time.to_string(),
                &data.end_time.to_string(),
                &data.activity_type,
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_app_usage_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<AppUsageData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM app_usage_data 
             WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(AppUsageData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                package_name: row.get(2)?,
                start_time: row.get::<_,String>(3)?.parse::<DateTime<Utc>>().unwrap(),
                end_time: row.get::<_,String>(4)?.parse::<DateTime<Utc>>().unwrap(),
                activity_type: row.get(5)?,
                metadata: row.get::<_,Option<String>>(6)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        let mut data = Vec::new();
        for row in rows {
            data.push(row?);
        }
        Ok(data)
    }

    /* Wifi Data */
    pub fn insert_wifi_data(&self, data: &WifiData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO wifi_data (
                timestamp, device_id, ssid, bssid, strength, frequency, ip_address, link_speed, security_type, 
                is_5ghz, is_6ghz, is_passpoint, is_restricted, nearby_networks, metadata
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            &[
                &data.timestamp.to_string(),
                &data.device_id,
                &data.ssid,
                &data.bssid,
                &data.strength.to_string(),
                &data.frequency.to_string(),
                &data.ip_address,
                &data.link_speed.to_string(),
                &data.security_type,
                &data.is_5ghz.to_string(),
                &data.is_6ghz.to_string(),
                &data.is_passpoint.to_string(),
                &data.is_restricted.to_string(),
                &data.nearby_networks.as_ref().map(|n| serde_json::to_string(n).unwrap()).unwrap_or_default(),
                &data.metadata.as_ref().map(|m| serde_json::to_string(m).unwrap()).unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn get_wifi_data(&self, device_id: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<WifiData>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM wifi_data 
             WHERE device_id = ? AND timestamp BETWEEN ? AND ?"
        )?;

        let rows = stmt.query_map([device_id, &start.to_string(), &end.to_string()], |row| {
            Ok(WifiData {
                timestamp: row.get::<_,String>(0)?.parse::<DateTime<Utc>>().unwrap(),
                device_id: row.get(1)?,
                ssid: row.get(2)?,
                bssid: row.get(3)?,
                strength: row.get(4)?,
                frequency: row.get(5)?,
                ip_address: row.get(6)?,
                link_speed: row.get(7)?,
                security_type: row.get(8)?,
                is_5ghz: row.get(9)?,
                is_6ghz: row.get(10)?,
                is_passpoint: row.get(11)?,
                is_restricted: row.get(12)?,
                nearby_networks: row.get::<_,Option<String>>(13)?.map(|s| serde_json::from_str(&s).unwrap()),
                metadata: row.get::<_,Option<String>>(14)?.map(|s| serde_json::from_str(&s).unwrap()),
            })
        })?;

        let mut data = Vec::new();
        for row in rows {
            data.push(row?);
        }
        Ok(data)
    }
}

impl std::fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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

    #[test]
    fn test_get_app_usage_data() -> Result<()> {
        let dir = tempdir()?;
        let db_path = dir.path().join("test.db");
        let db = Database::new(&db_path)?;

        let data = AppUsageData {
            timestamp: Utc::now(),
            device_id: "test_device".to_string(),
            package_name: "com.example.app".to_string(),
            start_time: Utc::now() - chrono::Duration::minutes(30),
            end_time: Utc::now(),
            activity_type: "foreground".to_string(),
            metadata: None,
        };

        db.insert_app_usage_data(&data)?;

        let start = Utc::now() - chrono::Duration::hours(1);
        let end = Utc::now() + chrono::Duration::hours(1);

        let retrieved = db.get_app_usage_data("test_device", start, end)?;
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].package_name, data.package_name);
        assert_eq!(retrieved[0].activity_type, data.activity_type);

        Ok(())
    }
}
