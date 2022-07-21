use anchor_lang::prelude::*;
use anchor_spl::{
  token::{
    Token, 
    TokenAccount,
  },
};
use solana_program::{
  instruction::*,
  program::{
    invoke
  },
};

#[derive(Accounts)]
pub struct Deposit<'info> {
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
#[derive(Accounts)]
pub struct Withdraw<'info> {
  pub owner: Signer<'info>,
  #[account(
    mut,
    has_one = owner,
  )]
  pub source_collateral_account: Box<Account<'info, TokenAccount>>,
  #[account(
    mut,
    has_one = owner,
  )]
  pub destination_liquidity_account: Box<Account<'info, TokenAccount>>,

  pub token_program: Program<'info, Token>,
}

pub fn deposit_handler<'a, 'b, 'c, 'info,>(
  ctx: Context<'_, '_, '_, 'info, Deposit<'info>>, 
  amount: u64
) -> Result<()> {
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
        token_program: ctx.accounts.token_program.to_account_info(),
      }, 
    ), amount
  )?;
  Ok(())
}
pub fn withdraw_handler<'a, 'b, 'c, 'info,>(
  ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>, 
  amount: u64
) -> Result<()> {
  let remaining_accounts = ctx.remaining_accounts;

  spl_vault_token::cpi::withdraw(
    CpiContext::new(
      remaining_accounts[3].to_account_info(),
      spl_vault_token::cpi::accounts::PoolInteraction {
        owner: ctx.accounts.owner.to_account_info(),
        token_account: ctx.accounts.destination_liquidity_account.to_account_info(),
        vault_token_account: ctx.accounts.source_collateral_account.to_account_info(),
        vault_token_mint: remaining_accounts[0].to_account_info(),
        vault_info: remaining_accounts[1].to_account_info(),
        pool: remaining_accounts[2].to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
      }, 
    ), amount
  )?;
  Ok(())
}
