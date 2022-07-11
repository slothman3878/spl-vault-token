use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use crate::errors::VaultTokenError;

pub fn collateral_to_liquidity(
  total_collateral: u64,
  total_liquidity: u64,
  collateral_amount: u64,
) -> Result<u64, VaultTokenError> {
  let liquidity_amount = Decimal::from(collateral_amount)
    .checked_div(Decimal::from(total_collateral))
    .ok_or(VaultTokenError::ConversionError)?
    .checked_mul(Decimal::from(total_liquidity))
    .ok_or(VaultTokenError::ConversionError)?
    .floor()
    .to_u64()
    .ok_or(VaultTokenError::ConversionError)?;
  Ok(liquidity_amount)
}

pub fn liquidity_to_collateral(
  total_collateral: u64,
  total_liquidity: u64,
  liquidity_amount: u64,
) -> Result<u64, VaultTokenError> {
  let collateral_amount = Decimal::from(liquidity_amount)
    .checked_div(Decimal::from(total_liquidity))
    .ok_or(VaultTokenError::ConversionError)?
    .checked_mul(Decimal::from(total_collateral))
    .ok_or(VaultTokenError::ConversionError)?
    .floor()
    .to_u64()
    .ok_or(VaultTokenError::ConversionError)?;
  Ok(collateral_amount)
}