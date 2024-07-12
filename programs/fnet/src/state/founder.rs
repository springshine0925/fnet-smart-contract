use anchor_lang::prelude::*;

#[account]
pub struct Founder {
  pub user: Pubkey,
  pub withdrawn: u64,
}