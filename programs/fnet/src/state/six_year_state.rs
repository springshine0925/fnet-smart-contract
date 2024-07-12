use anchor_lang::prelude::*;

#[account]
pub struct SixYearState {
  pub total_amount: u64,
  pub claimed_amount: u64,
  pub last_claim: i64,
}