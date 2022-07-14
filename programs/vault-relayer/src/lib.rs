use anchor_lang::prelude::*;
use anchor_spl::token::{
  TokenAccount,
  Token,
};

declare_id!("MqUazenmqvR1S2T9aNh9eeKY1yWMy3KG8fQRWTXRb6L");

#[program]
pub mod vault_relayer {
  use super::*;

  pub fn deposit<'info>(ctx: Context<'_, '_, '_, 'info, Deposit<'info>>, amount: u64) -> Result<()> {
    //let owner = &ctx.accounts.owner;
    let remaining_accounts = ctx.remaining_accounts;

    spl_vault_token::cpi::deposit(
      CpiContext::new(
        remaining_accounts[3].to_account_info(), 
        spl_vault_token::cpi::accounts::PoolInteraction {
          owner: ctx.accounts.owner.to_account_info(),
          token_account: ctx.accounts.source_liquidity_account.to_account_info(),
          vault_token_account: ctx.accounts.destination_collateral_account.to_account_info(),
          vault_token_mint: remaining_accounts[0].to_account_info(),
          vault_info: remaining_accounts[1].to_account_info(),
          pool: remaining_accounts[2].to_account_info(),
          token_program: ctx.accounts.token_program.to_account_info(), // remaining_accounts[3].to_account_info(),
        }
      ), amount)?;
    Ok(())
  }
}

#[derive(Accounts)]
pub struct Deposit<'info>{
  pub owner: Signer<'info>,
  #[account(
    mut,
    has_one = owner,
  )]
  pub source_liquidity_account: Box<Account<'info, TokenAccount>>,
  #[account(
    mut,
    has_one = owner,
  )]
  pub destination_collateral_account: Box<Account<'info, TokenAccount>>,

  pub token_program: Program<'info, Token>,
}
