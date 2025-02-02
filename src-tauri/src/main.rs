// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    use std::path::Path;
    let db_path = Path::new("loom.db");
    if !db_path.exists() {
        loom_app_lib::db::Database::new(db_path).expect("Failed to create database");
    }
    
    loom_app_lib::run()
}
