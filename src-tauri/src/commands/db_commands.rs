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
