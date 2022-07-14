use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;
pub mod utils;
use instructions::*;

declare_id!("DtSYEGH3LiHBebux3Lo6LuXoNMM32xBuVvE3nmZv9BF1");

#[program]
pub mod spl_vault_token {
  use super::*;

  pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    initialize::handler(ctx)
  }

  pub fn deposit(ctx: Context<PoolInteraction>, amount: u64) -> Result<()> {
    pool_interaction::deposit_handler(ctx, amount)
  }

  pub fn withdraw(ctx: Context<PoolInteraction>, amount: u64) -> Result<()> {
    pool_interaction::withdraw_handler(ctx, amount)
  }
}