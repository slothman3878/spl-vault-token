use anchor_lang::prelude::*;

#[error_code]
pub enum VaultTokenError {
  #[msg("arbitrary error")]
  ArbitraryError,
  #[msg("liquidity to/from collateral conversion decimal overflow")]
  ConversionError,
}