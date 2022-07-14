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
}
