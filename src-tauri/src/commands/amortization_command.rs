use rust_decimal::{dec, Decimal, MathematicalOps};

#[tauri::command]
pub async fn calculate_monthly_payment(
    principal: Decimal,
    rate: Decimal,
    number_of_months: Decimal,
) -> Result<Decimal, String> {
    calc_amortization(principal, rate, number_of_months)
}

fn calc_amortization(
    principal: Decimal,
    rate: Decimal,
    number_of_months: Decimal,
) -> Result<Decimal, String> {
    // M = Pr / 1 - (1 + r)^-n
    let r = rate / dec!(100) / dec!(12);
    let base = Decimal::ONE + r;
    let factor = base.powd(-number_of_months);
    let den = Decimal::ONE - factor;

    let payment = (principal * r / den).round_dp(2);

    return Ok(payment);
}
