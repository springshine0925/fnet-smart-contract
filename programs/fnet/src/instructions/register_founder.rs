use anchor_lang::prelude::*;

use crate::{AppState, Founder, FOUNDER_SEED};
use std::mem::size_of;

#[derive(Accounts)]
pub struct RegigsterFounder<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut, has_one = owner)]
    pub app_state: Box<Account<'info, AppState>>,

    /// CHECK:
    pub new_founder: UncheckedAccount<'info>,

    #[account(
      init,
      payer = owner,
      space = size_of::<Founder>() + 8,
      seeds = [FOUNDER_SEED, new_founder.key().as_ref()],
      bump,
    )]
    pub founder: Box<Account<'info, Founder>>,

    pub system_program: Program<'info, System>,
}


pub fn register_founder_handler(
  ctx: Context<RegigsterFounder>,
) -> Result<()> {
  let founder: &mut Box<Account<Founder>> =  &mut ctx.accounts.founder;
  founder.user = ctx.accounts.new_founder.key();
  founder.withdrawn = 0;
  let app_state: &mut Box<Account<AppState>> = &mut ctx.accounts.app_state;
  app_state.founder_count += 1;
  Ok(())
}