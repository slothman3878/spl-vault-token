use anchor_lang::prelude::*;

declare_id!("MqUazenmqvR1S2T9aNh9eeKY1yWMy3KG8fQRWTXRb6L");

#[program]
pub mod vault_relayer {
  use super::*;

  pub fn relay_deposit<'info>(ctx: Context<'_, '_, '_, 'info, Initialize<'info>>, amount: u64) -> Result<()> {
    //let owner = &ctx.accounts.owner;
    let remaining_accounts = ctx.remaining_accounts;

    spl_vault_token::cpi::deposit(
      CpiContext::new(
        remaining_accounts[6].to_account_info(), 
        spl_vault_token::cpi::accounts::PoolInteraction {
          owner: ctx.accounts.owner.to_account_info(),
          token_account: remaining_accounts[0].to_account_info(),
          vault_token_account: remaining_accounts[1].to_account_info(),
          vault_token_mint: remaining_accounts[2].to_account_info(),
          vault_info: remaining_accounts[3].to_account_info(),
          pool: remaining_accounts[4].to_account_info(),
          token_program: remaining_accounts[5].to_account_info(),
        }
      ), amount)?;
    Ok(())
  }
}

#[derive(Accounts)]
pub struct Initialize<'info>{
  pub owner: Signer<'info>,
}
