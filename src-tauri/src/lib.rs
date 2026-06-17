mod commands;
mod models;

use crate::commands::amortization_command::calculate_monthly_payment;

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::fs::create_dir_all;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub struct AppState {
    pub db: SqlitePool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup_app)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, calculate_monthly_payment])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_app<R: tauri::Runtime>(app: &mut tauri::App<R>) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        let db_path = app_handle
            .path()
            .app_data_dir()
            .expect("failed to resolve app data dir")
            .join("app.db");

        create_dir_all(db_path.parent().unwrap()).unwrap();

        let pool = SqlitePoolOptions::new()
            .max_connections(6)
            .connect(&format!("sqlite://{}?mode=rwc", db_path.display()))
            .await
            .expect("failed to connect to database");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("failed to run migrations");

        app_handle.manage(AppState { db: pool });
    });
    Ok(())
}
