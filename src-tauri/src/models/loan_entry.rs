use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanEntry {
    pub id: Uuid,
    pub date_created: DateTime<Utc>,
    pub date_updated: DateTime<Utc>,
    pub principal: Decimal,
    pub rate: Decimal,
    pub number_of_months: Decimal,
    pub monthly_payment: Decimal,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanEntryDto {
    pub principal: Decimal,
    pub rate: Decimal,
    pub number_of_months: Decimal,
    pub monthly_payment: Decimal,
}

impl TryFrom<LoanEntryRow> for LoanEntry {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: LoanEntryRow) -> Result<Self, Self::Error> {
        Ok(LoanEntry {
            id: Uuid::from_slice(&value.id)?,
            date_created: value.date_created.parse()?,
            date_updated: value.date_updated.parse()?,
            principal: value.principal.parse()?,
            rate: value.rate.parse()?,
            number_of_months: value.number_of_months.parse()?,
            monthly_payment: value.monthly_payment.parse()?,
        })
    }
}

#[derive(sqlx::FromRow)]
pub struct LoanEntryRow {
    pub id: Vec<u8>,
    pub date_created: String,
    pub date_updated: String,
    pub principal: String,
    pub rate: String,
    pub number_of_months: String,
    pub monthly_payment: String,
}
