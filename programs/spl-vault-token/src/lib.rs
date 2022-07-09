use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;

pub mod state;
pub mod instructions;
use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

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