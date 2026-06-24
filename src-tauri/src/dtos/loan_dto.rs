use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanDto {
    pub principal: Decimal,
    pub rate: Decimal,
    pub number_of_months: Decimal,
    pub monthly_payment: Decimal,
}
