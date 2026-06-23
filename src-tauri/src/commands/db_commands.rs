use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::entity::loans::{self, Entity as Loan};
use crate::models::loan_entry::LoanEntryDto;
use crate::AppState;

#[tauri::command]
pub async fn get_all_loans(state: tauri::State<'_, AppState>) -> Result<Vec<loans::Model>, String> {
    Loan::find().all(&state.db).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_single_loan(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<loans::Model, String> {
    let real_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    Loan::find_by_id(real_id)
        .one(&state.db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Loan was not found".to_string())
}

#[tauri::command]
pub async fn create_loan(
    state: tauri::State<'_, AppState>,
    dto: LoanEntryDto,
) -> Result<Uuid, String> {
    let id = Uuid::new_v4();
    let date_now = Utc::now().naive_utc();

    let new_loan = loans::ActiveModel {
        id: Set(id),
        date_created: Set(date_now),
        date_updated: Set(date_now),
        principal: Set(dto.principal),
        rate: Set(dto.rate),
        number_of_months: Set(dto.number_of_months),
        monthly_payment: Set(dto.monthly_payment),
    };

    let _ = loans::Entity::insert(new_loan)
        .exec(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    Ok(id)
}

#[tauri::command]
pub async fn update_loan(
    state: tauri::State<'_, AppState>,
    id: String,
    update: LoanEntryDto,
) -> Result<bool, String> {
    Ok(false) // TODO
}

// #[tauri::command]
// pub async fn delete_loan(state: tauri::State<'_, AppState>, id: String) -> Result<bool, String> {
//     Ok(false) // TODO
// }
