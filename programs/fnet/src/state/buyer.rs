use anchor_lang::prelude::*;

#[account]
pub struct Buyer {
  pub total_paid: u64,
  pub total_bought1: u64,
  pub total_bought2: u64,
  pub total_bought3: u64,
  pub claimed_amount1: u64,
  pub claimed_amount2: u64,
  pub claimed_amount3: u64,
  pub last_claim1: i64,
  pub last_claim2: i64,
  pub last_claim3: i64,
  pub user: Pubkey,
}
