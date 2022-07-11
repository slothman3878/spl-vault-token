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
pub struct Initialize<'info> {
  #[account(mut)]
  pub vault_creator: Signer<'info>,

  #[account(
    init,
    payer = vault_creator,
    seeds = [b"vault_info", mint.key().as_ref(),],
    bump,
    space = VaultInfo::size(),
  )]
  pub vault_info: Box<Account<'info, VaultInfo>>,

  /**
   * liquidity pool and liquidity token mint
   */
  #[account(
    init,
    payer = vault_creator,
    seeds = [b"pool", vault_info.key().as_ref()],
    bump,
    token::mint = mint,
    token::authority = pool,
  )]
  pub pool: Box<Account<'info, TokenAccount>>,

  pub mint: Box<Account<'info, Mint>>,

  /**
   * vault share token mint
   */
  #[account(
    init,
    payer = vault_creator,
    seeds = [b"vault_token_mint", vault_info.key().as_ref()],
    bump,
    mint::decimals = mint.decimals,
    mint::authority = vault_token_mint,
  )]
  pub vault_token_mint: Box<Account<'info, Mint>>,

  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, Token>,
  // pub associated_token_program: Program<'info, AssociatedToken>,
  pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
  let vault_creator = &ctx.accounts.vault_creator;
  let vault_info = &mut ctx.accounts.vault_info;
  let mint = &ctx.accounts.mint;
  let vault_token_mint = &ctx.accounts.vault_token_mint;
  let pool = &ctx.accounts.pool;

  /* do some checks */

  let vault_info_bump = *ctx.bumps.get("vault_info").unwrap();
  let pool_bump = *ctx.bumps.get("pool").unwrap();
  let vault_token_mint_bump = *ctx.bumps.get("vault_token_mint").unwrap();

  vault_info.vault_creator = vault_creator.key();
  vault_info.mint = mint.key();
  vault_info.vault_token_mint = vault_token_mint.key();
  vault_info.pool = pool.key();
  vault_info.pool_bump = pool_bump;
  vault_info.vault_token_mint_bump = vault_token_mint_bump;
  vault_info.vault_info_bump = vault_info_bump;

  Ok(())
}