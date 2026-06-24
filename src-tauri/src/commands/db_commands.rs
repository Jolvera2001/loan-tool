use chrono::Utc;
use sea_orm::entity::ModelTrait;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::dtos::loan_dto::LoanDto;
use crate::entity::loans::{self, Entity as Loan};
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
pub async fn create_loan(state: tauri::State<'_, AppState>, dto: LoanDto) -> Result<Uuid, String> {
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
    id: Uuid,
    update: LoanDto,
) -> Result<Uuid, String> {
    let date_now = Utc::now().naive_utc();

    let mut loan: loans::ActiveModel = Loan::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Loan not found".to_string())?
        .into();

    loan.date_updated = Set(date_now);
    loan.principal = Set(update.principal);
    loan.rate = Set(update.rate);
    loan.number_of_months = Set(update.number_of_months);
    loan.monthly_payment = Set(update.monthly_payment);

    let updated = loan.update(&state.db).await.map_err(|e| e.to_string())?;

    Ok(updated.id)
}

#[tauri::command]
pub async fn delete_loan(state: tauri::State<'_, AppState>, id: String) -> Result<bool, String> {
    let real_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let loan = Loan::find_by_id(real_id)
        .one(&state.db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Loan not found".to_string())?;

    let res = loan.delete(&state.db).await.map_err(|e| e.to_string())?;
    let check = res.rows_affected > 0;

    Ok(check)
}
