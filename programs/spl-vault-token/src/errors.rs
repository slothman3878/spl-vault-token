use anchor_lang::prelude::*;

#[error_code]
pub enum VaultTokenError {
  #[msg("arbitrary error")]
  ArbitraryError,
  #[msg("invariant check failed")]
  InvariantError,
  #[msg("liquidity to/from collateral conversion decimal overflow")]
  ConversionError,
}