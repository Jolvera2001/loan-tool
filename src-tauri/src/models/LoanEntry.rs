use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct LoanEntry {
    id: Uuid,
    date_created: DateTime<Utc>,
    principal: Decimal,
    rate: Decimal,
    number_of_months: Decimal,
    monthly_payment: Decimal,
}

impl TryFrom<LoanEntryRow> for LoanEntry {
    type Error = rust_decimal::Error;

    fn try_from(value: LoanEntryRow) -> Result<Self, Self::Error> {
        Ok(LoanEntry {
            id: value.id,
            date_created: value.date_created,
            principal: value.principal.parse()?,
            rate: value.rate.parse()?,
            number_of_months: value.number_of_months.parse()?,
            monthly_payment: value.monthly_payment.parse()?,
        })
    }
}

#[derive(sqlx::FromRow)]
pub struct LoanEntryRow {
    id: Uuid,
    date_created: DateTime<Utc>,
    principal: String,
    rate: String,
    number_of_months: String,
    monthly_payment: String,
}
