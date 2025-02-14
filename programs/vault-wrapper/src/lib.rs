use anchor_lang::prelude::*;
pub mod instructions;
use instructions::*;

declare_id!("xNjpLELAS4hhPNgoFWjuVzRg83cZNnippovXHuNXLAo");

#[program]
pub mod vault_wrapper {
  use super::*;

  pub fn deposit<'info>(
    ctx: Context<'_, '_, '_, 'info, Deposit<'info>>, 
    amount: u64
  ) -> Result<()> {
    instructions::deposit_handler(ctx, amount)
  }

  pub fn mint_to<'info>(
    ctx: Context<'_, '_, '_, 'info, Deposit<'info>>,
    amount: u64,
  ) -> Result<()> {
    instructions::mint_to_handler(ctx, amount)
  }

  pub fn withdraw<'info>(
    ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>,
    amount: u64,
  ) -> Result<()> {
    instructions::withdraw_handler(ctx, amount)
  }

  pub fn redeem<'info>(
    ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>,
    amount: u64,
  ) -> Result<()> {
    instructions::redeem_handler(ctx, amount)
  }
}
