
use anchor_lang::prelude::*;

#[account]
pub struct Round {
  pub round_index: u8,
  pub start_time: i64,
  pub end_time: i64,
  pub total_collected: u64,
  pub total_sold: u64,
  // @dev stable coin, usdc or usdt
  pub currency: Currency,
  pub token_account: Pubkey,
  pub burned: bool,
  pub buyer_count: u64,
}


#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Currency {
    // @dev decimals of ido token
    pub decimals: u8,
    // @dev mint address of ido token
    pub mint: Pubkey,
}