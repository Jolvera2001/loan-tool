use uuid::Uuid;

use crate::models::loan_entry::{LoanEntry, LoanEntryRow};
use crate::AppState;

#[tauri::command]
pub async fn get_all_loans(state: tauri::State<'_, AppState>) -> Result<Vec<LoanEntry>, String> {
    let rows = sqlx::query_as!(LoanEntryRow, "SELECT * from loans")
        .fetch_all(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    rows.into_iter()
        .map(LoanEntry::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_single_loan(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<LoanEntry, String> {
    let uuid_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let row = sqlx::query_as!(LoanEntryRow, "SELECT * FROM loans WHERE id = ?", &uuid_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    let real_row = LoanEntry::try_from(row).map_err(|e| e.to_string())?;

    Ok(real_row)
}

#[tauri::command]
async fn create_loan(
    state: tauri::State<'_, AppState>,
    new_entry: LoanEntry,
) -> Result<Uuid, String> {
    Ok(Uuid::new_v4()) // TODO
}

#[tauri::command]
async fn update_loan(
    state: tauri::State<'_, AppState>,
    id: String,
    update: LoanEntry,
) -> Result<bool, String> {
    Ok(false) // TODO
}

#[tauri::command]
async fn delete_loan(state: tauri::State<'_, AppState>, id: String) -> Result<bool, String> {
    Ok(false) // TODO
}
