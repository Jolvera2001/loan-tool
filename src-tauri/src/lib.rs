pub mod commands;
pub mod models;
pub mod entity;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{fs::create_dir_all, time::Duration};
use tauri::Manager;

use crate::commands::{db_commands, loan_commands};

pub struct AppState {
    pub db: DatabaseConnection,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup_app)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // db_commands::get_all_loans,
            // db_commands::get_single_loan,
            // db_commands::create_loan,
            // db_commands::update_loan,
            // db_commands::delete_loan,
            loan_commands::calculate_monthly_payment,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// fn setup_app<R: tauri::Runtime>(app: &mut tauri::App<R>) -> Result<(), Box<dyn std::error::Error>> {
//     let app_handle = app.handle().clone();
//     tauri::async_runtime::spawn(async move {
//         let db_path = app_handle
//             .path()
//             .app_data_dir()
//             .expect("failed to resolve app data dir")
//             .join("app.db");

//         create_dir_all(db_path.parent().unwrap()).unwrap();

//         let pool = SqlitePoolOptions::new()
//             .max_connections(6)
//             .connect(&format!("sqlite://{}?mode=rwc", db_path.display()))
//             .await
//             .expect("failed to connect to database");

//         sqlx::migrate!("./migrations")
//             .run(&pool)
//             .await
//             .expect("failed to run migrations");

//         app_handle.manage(AppState { db: pool });
//     });
//     Ok(())
// }

fn setup_app<R: tauri::Runtime>(app: &mut tauri::App<R>) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        let db_path = app_handle
            .path()
            .app_data_dir()
            .expect("failed to resolve app data dir")
            .join("app.db");

        create_dir_all(db_path.parent().unwrap()).unwrap();

        let mut opt = ConnectOptions::new(&format!(
            "sqlite:///{}?mode=rwc",
            db_path.to_string_lossy().replace('\\', "/")
        ));
        opt.max_connections(5).sqlx_logging(false);
        let db = Database::connect(opt)
            .await
            .map_err(|e| e.to_string())
            .unwrap();

        Migrator::up(&db, None)
            .await
            .map_err(|e| e.to_string())
            .unwrap();

        app_handle.manage(AppState { db });
    });
    Ok(())
}
