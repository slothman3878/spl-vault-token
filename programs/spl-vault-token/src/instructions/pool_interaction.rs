use anchor_lang::prelude::*;
use anchor_spl::{
  // associated_token::ID,
  token::{
    Mint, 
    Token, 
    TokenAccount,
  },
};

use crate::state::{VaultInfo};
use crate::errors::VaultTokenError;

#[derive(Accounts)]
#[instruction(
  amount: u8,
)]
pub struct PoolInteraction<'info> {
  pub owner: Signer<'info>,

  #[account(
    mut,
    has_one = owner,
  )]
  pub token_account: Box<Account<'info, TokenAccount>>,
  #[account(
    mut,
    has_one = owner,
  )]
  pub vault_token_account: Box<Account<'info, TokenAccount>>,

  #[account(
    mut,
    seeds = [b"vault_token_mint", vault_info.key().as_ref(),],
    bump,
  )]
  pub vault_token_mint: Box<Account<'info,Mint>>,
  
  #[account(
    // mut,
    has_one = vault_token_mint,
    seeds = [b"vault_info", vault_info.mint.key().as_ref(),],
    bump,
  )]
  pub vault_info: Box<Account<'info, VaultInfo>>,

  #[account(
    mut,
    seeds = [b"pool", vault_info.key().as_ref(),],
    bump,
  )]
  pub pool: Box<Account<'info, TokenAccount>>,

  pub token_program: Program<'info, Token>,
}

pub fn deposit_handler(ctx: Context<PoolInteraction>, amount: u64) -> Result<()> {
  let vault_info = &ctx.accounts.vault_info;
  
  let depositor = &ctx.accounts.owner;
  let depositor_token_account = &mut ctx.accounts.token_account;
  let depositor_vault_token_account = &mut ctx.accounts.vault_token_account;
  
  let pool = &mut ctx.accounts.pool;
  let vault_token_mint = &mut ctx.accounts.vault_token_mint;
  
  // it might be best if we calculate this elsewhere...
  let vault_token_supply: u64 = vault_token_mint.supply;
  let pool_balance: u64 = pool.amount;
  let collateral_to_liquidity = vault_token_supply / pool_balance;

  // cross program calls
  anchor_spl::token::mint_to(
    CpiContext::new_with_signer(
      ctx.accounts.token_program.to_account_info(), 
      anchor_spl::token::MintTo{
        authority: vault_token_mint.to_account_info(),
        mint: vault_token_mint.to_account_info(),
        to: depositor_vault_token_account.to_account_info(),
      }, 
      &[&[
        b"vault_token_mint", vault_info.key().as_ref(),
        &[vault_info.vault_token_mint_bump],
      ]],
    ), amount / 2,
  )?;

  anchor_spl::token::transfer(
    CpiContext::new(
      ctx.accounts.token_program.to_account_info(),
      anchor_spl::token::Transfer{
        authority: depositor.to_account_info(),
        from: depositor_token_account.to_account_info(),
        to: pool.to_account_info(),
      },
    ), amount,
  )?;

  // reload updated accounts
  vault_token_mint.reload()?;
  depositor_vault_token_account.reload()?;
  depositor_token_account.reload()?;
  pool.reload()?;

  // validate invariants

  Ok(())
}

pub fn withdraw_handler(ctx: Context<PoolInteraction>, amount: u64) -> Result<()> {
  let vault_info = &ctx.accounts.vault_info;
  
  let withdrawer = &ctx.accounts.owner;
  let withdrawer_token_account = &mut ctx.accounts.token_account;
  let withdrawer_vault_token_account = &mut ctx.accounts.vault_token_account;
  
  let pool = &mut ctx.accounts.pool;
  let vault_token_mint = &mut ctx.accounts.vault_token_mint;

  let token_program = &ctx.accounts.token_program;

  // checks
  // require_keys_eq!(token_program.key(), ID, VaultTokenError::UnknownError); // unnecessary check

  // cross program calls
  anchor_spl::token::burn(
    CpiContext::new(
      token_program.to_account_info(),
      anchor_spl::token::Burn{
        authority: withdrawer.to_account_info(),
        mint: vault_token_mint.to_account_info(),
        from: withdrawer_vault_token_account.to_account_info(),
      },
    ), amount / 2
  )?;

  anchor_spl::token::transfer(
    CpiContext::new_with_signer(
      token_program.to_account_info(),
      anchor_spl::token::Transfer{
        authority: pool.to_account_info(),
        from: pool.to_account_info(),
        to: withdrawer_token_account.to_account_info(),
      },
      &[&[
        b"pool", vault_info.key().as_ref(),
        &[vault_info.pool_bump],
      ]],
    ), amount,
  )?;

  // reload updated accounts
  vault_token_mint.reload()?;
  withdrawer_vault_token_account.reload()?;
  withdrawer_token_account.reload()?;
  pool.reload()?;

  // validate invariants

  Ok(())
}