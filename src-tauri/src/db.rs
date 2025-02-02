use std::path::Path;
use duckdb::{Connection, Result as DuckResult, params};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::PathBuf;

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    /// Create a new database connection
    pub fn new<P: AsRef<Path>>(path: P) -> DuckResult<Self> {
        let conn = Connection::open(path)?;
        
        // Enable JSON extension
        conn.execute_batch("INSTALL json; LOAD json;")?;
        
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Execute a SQL query that returns no rows
    pub fn execute(&self, sql: &str, params: &[&dyn duckdb::ToSql]) -> DuckResult<usize> {
        let conn = self.conn.lock().unwrap();
        conn.execute(sql, params)
    }

    /// Execute a SQL query that returns rows
    pub fn query<T, F>(&self, sql: &str, params: &[&dyn duckdb::ToSql], mut f: F) -> DuckResult<Vec<T>>
    where
        F: FnMut(&duckdb::Row) -> DuckResult<T>,
    {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params, |row| f(row))?;
        rows.collect()
    }

    /// Execute a SQL query and return the results as JSON
    pub fn query_json(&self, sql: &str, params: &[&dyn duckdb::ToSql]) -> DuckResult<Value> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            &format!("SELECT json_group_array(json_object(*)) FROM ({sql})"),
            params,
            |row| row.get(0),
        )
    }

    /// Insert JSON data into a table
    pub fn insert_json(&self, table: &str, json: &str) -> DuckResult<usize> {
        let sql = format!(
            "INSERT INTO {} SELECT * FROM json_to_table(?)",
            table
        );
        self.execute(&sql, &[&json])
    }

    /// Create a table from a JSON schema
    pub fn create_table_from_json(&self, table: &str, json_schema: &str) -> DuckResult<()> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} AS SELECT * FROM json_to_table(?) WHERE 1=0",
            table
        );
        self.execute(&sql, &[&json_schema])?;
        Ok(())
    }

    /// Begin a transaction
    pub fn begin_transaction(&self) -> DuckResult<()> {
        self.execute("BEGIN TRANSACTION", &[])?;
        Ok(())
    }

    /// Commit a transaction
    pub fn commit_transaction(&self) -> DuckResult<()> {
        self.execute("COMMIT", &[])?;
        Ok(())
    }

    /// Rollback a transaction
    pub fn rollback_transaction(&self) -> DuckResult<()> {
        self.execute("ROLLBACK", &[])?;
        Ok(())
    }

    /// Execute multiple SQL statements in a batch
    pub fn execute_batch(&self, sql: &str) -> DuckResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(sql)
    }

    /// Get the last inserted row id
    pub fn last_insert_rowid(&self) -> i64 {
        let conn = self.conn.lock().unwrap();
        conn.last_insert_rowid()
    }

    /// Check if a table exists
    pub fn table_exists(&self, table: &str) -> DuckResult<bool> {
        let sql = "SELECT count(*) FROM information_schema.tables WHERE table_name = ?";
        let count: i64 = self.conn.lock().unwrap()
            .query_row(sql, &[&table], |row| row.get(0))?;
        Ok(count > 0)
    }

    /// Get the schema of a table
    pub fn get_table_schema(&self, table: &str) -> DuckResult<String> {
        let sql = format!("DESCRIBE {}", table);
        let schema = self.query_json(&sql, &[])?;
        Ok(schema.to_string())
    }

    /// Create an index on a table
    pub fn create_index(&self, table: &str, columns: &[&str]) -> DuckResult<()> {
        let index_name = format!("idx_{}__{}", table, columns.join("_"));
        let columns = columns.join(", ");
        let sql = format!(
            "CREATE INDEX IF NOT EXISTS {} ON {} ({})",
            index_name, table, columns
        );
        self.execute(&sql, &[])?;
        Ok(())
    }
}

// Transaction wrapper for RAII-style transactions
pub struct Transaction<'a> {
    db: &'a Database,
    committed: bool,
}

impl<'a> Transaction<'a> {
    pub fn new(db: &'a Database) -> DuckResult<Self> {
        db.begin_transaction()?;
        Ok(Self {
            db,
            committed: false,
        })
    }

    pub fn commit(mut self) -> DuckResult<()> {
        self.committed = true;
        self.db.commit_transaction()
    }
}

impl<'a> Drop for Transaction<'a> {
    fn drop(&mut self) {
        if !self.committed {
            let _ = self.db.rollback_transaction();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::datatypes::*;
    use serde_json::json;

    fn get_init_sql() -> String {
        let possible_paths = [
            "src-tauri/db-setup/init.sql",
            "db-setup/init.sql",
            "../db-setup/init.sql",
        ];

        for path in possible_paths {
            if let Ok(content) = fs::read_to_string(path) {
                return content;
            }
        }

        panic!("Could not find init.sql in any of the expected locations");
    }

    fn init_test_db() -> (tempfile::TempDir, Database) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::new(&db_path).unwrap();
        
        // Read and execute initialization SQL
        let init_sql = get_init_sql();
        db.execute_batch(&init_sql).unwrap();
        
        (dir, db)
    }

    #[test]
    fn test_user_crud() -> DuckResult<()> {
        let (_dir, db) = init_test_db();

        // Create
        let user = User {
            id: "user1".to_string(),
            email: "test@example.com".to_string(),
            name: Some("Test User".to_string()),
            encrypted_password: "hashed_password".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let user_json = serde_json::to_string(&user)?;
        db.insert_json("users", &user_json)?;

        // Read
        let result = db.query_json(
            "SELECT * FROM users WHERE id = ?",
            &[&"user1"]
        )?;
        let users: Vec<User> = serde_json::from_value(result)?;
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].email, "test@example.com");

        // Update
        db.execute(
            "UPDATE users SET name = ? WHERE id = ?",
            &[&"Updated User", &"user1"]
        )?;

        // Delete
        db.execute("DELETE FROM users WHERE id = ?", &[&"user1"])?;
        
        let count: i64 = db.query_row(
            "SELECT COUNT(*) FROM users WHERE id = ?",
            &[&"user1"],
            |row| row.get(0)
        )?;
        assert_eq!(count, 0);

        Ok(())
    }

    #[test]
    fn test_device_crud() -> DuckResult<()> {
        let (_dir, db) = init_test_db();

        // Create
        let device = Device {
            device_id: "device1".to_string(),
            user_id: "user1".to_string(),
            device_type: DeviceType::Phone,
            os_type: "iOS".to_string(),
            os_version: "15.0".to_string(),
            app_version: "1.0.0".to_string(),
            available_sensors: vec!["accelerometer".to_string(), "gps".to_string()],
            capabilities: DeviceCapabilities {
                has_camera: true,
                has_microphone: true,
                has_gps: true,
                has_accelerometer: true,
                has_gyroscope: true,
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
                has_compass: true,
                screen_details: ScreenDetails {
                    width: 1170,
                    height: 2532,
                    density: 3.0,
                    refresh_rate: 120,
                },
            },
            created_at: Utc::now(),
            last_seen: Utc::now(),
            updated_at: Utc::now(),
        };
        let device_json = serde_json::to_string(&device)?;
        db.insert_json("devices", &device_json)?;

        // Read
        let result = db.query_json(
            "SELECT * FROM devices WHERE device_id = ?",
            &[&"device1"]
        )?;
        let devices: Vec<Device> = serde_json::from_value(result)?;
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].os_type, "iOS");

        // Update
        db.execute(
            "UPDATE devices SET os_version = ? WHERE device_id = ?",
            &[&"15.1", &"device1"]
        )?;

        // Delete
        db.execute("DELETE FROM devices WHERE device_id = ?", &[&"device1"])?;

        Ok(())
    }

    #[test]
    fn test_sensor_data_crud() -> DuckResult<()> {
        let (_dir, db) = init_test_db();

        // Test accelerometer data
        let accel_data = AccelerometerData {
            timestamp: Utc::now(),
            device_id: "device1".to_string(),
            x: 0.5,
            y: -0.3,
            z: 9.8,
            accuracy: Some(0.01),
            metadata: Some(json!({
                "sample_rate": 100
            }).as_object().unwrap().clone()),
        };
        let accel_json = serde_json::to_string(&accel_data)?;
        db.insert_json("accelerometer_data", &accel_json)?;

        // Read and verify
        let result = db.query_json(
            "SELECT * FROM accelerometer_data WHERE device_id = ?",
            &[&"device1"]
        )?;
        let data: Vec<AccelerometerData> = serde_json::from_value(result)?;
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].x, 0.5);

        // Test GPS data
        let gps_data = GPSData {
            timestamp: Utc::now(),
            device_id: "device1".to_string(),
            latitude: 37.7749,
            longitude: -122.4194,
            altitude: Some(10.0),
            accuracy: Some(5.0),
            speed: Some(0.0),
            bearing: Some(90.0),
            satellites: Some(8),
            provider: Some("gps".to_string()),
            metadata: None,
        };
        let gps_json = serde_json::to_string(&gps_data)?;
        db.insert_json("gps_data", &gps_json)?;

        // Read and verify
        let result = db.query_json(
            "SELECT * FROM gps_data WHERE device_id = ?",
            &[&"device1"]
        )?;
        let data: Vec<GPSData> = serde_json::from_value(result)?;
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].latitude, 37.7749);

        Ok(())
    }

    #[test]
    fn test_note_crud() -> DuckResult<()> {
        let (_dir, db) = init_test_db();

        // Create note
        let note = Note {
            id: "note1".to_string(),
            user_id: "user1".to_string(),
            timestamp: Utc::now(),
            content: "Test note content".to_string(),
            priority: NotePriority::Medium,
            parent_id: None,
            tags: Some(vec!["test".to_string(), "example".to_string()]),
            embedding: Some(vec![0.1, 0.2, 0.3]),
            metadata: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let note_json = serde_json::to_string(&note)?;
        db.insert_json("notes", &note_json)?;

        // Read
        let result = db.query_json(
            "SELECT * FROM notes WHERE id = ?",
            &[&"note1"]
        )?;
        let notes: Vec<Note> = serde_json::from_value(result)?;
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].content, "Test note content");

        // Update
        db.execute(
            "UPDATE notes SET content = ? WHERE id = ?",
            &[&"Updated content", &"note1"]
        )?;

        // Delete
        db.execute("DELETE FROM notes WHERE id = ?", &[&"note1"])?;

        Ok(())
    }

    #[test]
    fn test_known_entity_crud() -> DuckResult<()> {
        let (_dir, db) = init_test_db();

        // Create entity
        let entity = KnownEntity {
            entity_id: "entity1".to_string(),
            entity_type: EntityType::Face,
            label: "John Doe".to_string(),
            embedding: Some(vec![0.1, 0.2, 0.3]),
            metadata: Some(json!({
                "confidence": 0.95
            }).as_object().unwrap().clone()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let entity_json = serde_json::to_string(&entity)?;
        db.insert_json("known_entities", &entity_json)?;

        // Read
        let result = db.query_json(
            "SELECT * FROM known_entities WHERE entity_id = ?",
            &[&"entity1"]
        )?;
        let entities: Vec<KnownEntity> = serde_json::from_value(result)?;
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].label, "John Doe");

        // Update
        db.execute(
            "UPDATE known_entities SET label = ? WHERE entity_id = ?",
            &[&"Jane Doe", &"entity1"]
        )?;

        // Delete
        db.execute("DELETE FROM known_entities WHERE entity_id = ?", &[&"entity1"])?;

        Ok(())
    }

    #[test]
    fn test_config_crud() -> DuckResult<()> {
        let (_dir, db) = init_test_db();

        // Create retention config
        let config = RetentionConfig {
            table_name: "sensor_data".to_string(),
            compression_enabled: true,
            compression_algorithm: CompressionAlgorithm::Lz4,
            retention_days: Some(30),
            downsample_after_days: Some(7),
            downsample_ratio: Some(10),
            convert_to_text: false,
            min_required_space_mb: Some(1000),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: None,
        };
        let config_json = serde_json::to_string(&config)?;
        db.insert_json("retention_config", &config_json)?;

        // Read
        let result = db.query_json(
            "SELECT * FROM retention_config WHERE table_name = ?",
            &[&"sensor_data"]
        )?;
        let configs: Vec<RetentionConfig> = serde_json::from_value(result)?;
        assert_eq!(configs.len(), 1);
        assert_eq!(configs[0].retention_days, Some(30));

        // Update
        let new_days: i32 = 60;
        db.execute(
            "UPDATE retention_config SET retention_days = ? WHERE table_name = ?",
            &[&new_days, &"sensor_data"]
        )?;

        // Delete
        db.execute("DELETE FROM retention_config WHERE table_name = ?", &[&"sensor_data"])?;

        Ok(())
    }
}
