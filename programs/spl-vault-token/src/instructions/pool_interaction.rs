use anchor_lang::prelude::*;
use anchor_spl::{
  token::{
    Mint, 
    Token, 
    TokenAccount,
  },
};

use crate::state::{VaultInfo};

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
    mut,
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
  let depositor_token_account = &ctx.accounts.token_account;
  let depositor_vault_token_account = &ctx.accounts.vault_token_account;
  
  let pool = &ctx.accounts.pool;
  let vault_token_mint = &ctx.accounts.vault_token_mint;
  
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

  Ok(())
}

pub fn withdraw_handler(ctx: Context<PoolInteraction>, amount: u64) -> Result<()> {
  let vault_info = &ctx.accounts.vault_info;
  
  let withdrawer = &ctx.accounts.owner;
  let withdrawer_token_account = &ctx.accounts.token_account;
  let withdrawer_vault_token_account = &ctx.accounts.vault_token_account;
  
  let pool = &ctx.accounts.pool;
  let vault_token_mint = &ctx.accounts.vault_token_mint;

  anchor_spl::token::burn(
    CpiContext::new(
      ctx.accounts.token_program.to_account_info(),
      anchor_spl::token::Burn{
        authority: withdrawer.to_account_info(),
        mint: vault_token_mint.to_account_info(),
        from: withdrawer_vault_token_account.to_account_info(),
      },
    ), amount / 2
  )?;

  anchor_spl::token::transfer(
    CpiContext::new_with_signer(
      ctx.accounts.token_program.to_account_info(),
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

  Ok(())
}