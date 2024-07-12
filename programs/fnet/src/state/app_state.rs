use anchor_lang::prelude::*;

#[account]
pub struct AppState {
  pub bump: u8,
  pub minted_founder: bool,
  pub minted_one_year: bool,
  pub minted_six_year: bool,
  pub founder_token_account: Pubkey,
  pub one_year_token_account: Pubkey,
  pub six_year_token_account: Pubkey,
  pub minted_time: i64,
  pub owner: Pubkey,
  pub founder_count: u64,
  pub founder_amount: u64,
}
