use crate::models::loan_entry::LoanEntryRow;
use crate::AppState;

#[tauri::command]
async fn command_name(state: tauri::State<'_, AppState>) -> Result<(), sqlx::Error> {
    let rows = sqlx::query_as!(LoanEntryRow, "SELECT * from loans")
        .fetch_all(&state.db)
        .await?;

    Ok(())
}
