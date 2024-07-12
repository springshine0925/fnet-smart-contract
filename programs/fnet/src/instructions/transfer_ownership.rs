use anchor_lang::prelude::*;
use anchor_spl::token_interface;

use crate::AppState;

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK:
    pub new_owner: UncheckedAccount<'info>,

    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(
      mut,
      seeds = [b"app-state", mint.key().as_ref()],
      bump,
      has_one = owner
    )]
    pub app_state: Box<Account<'info, AppState>>,
}

pub fn transfer_ownership_handler(ctx: Context<TransferOwnership>) -> Result<()> {
    ctx.accounts.app_state.owner = ctx.accounts.new_owner.key();
    Ok(())
}
