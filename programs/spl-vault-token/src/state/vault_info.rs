use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct VaultInfo{
  pub vault_creator: Pubkey,
  pub mint: Pubkey,
  pub vault_token_mint: Pubkey,
  pub vault_token_mint_bump: u8,
  pub vault_info_bump: u8,
  pub pool_bump: u8,
  pub pool: Pubkey,
}

impl VaultInfo {
  pub fn size() -> usize {
    32 * 4
    + 8 * 3
    + 8
  }
}

