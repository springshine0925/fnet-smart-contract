use anchor_lang::prelude::*;

#[constant]
pub const DISTRIBUTION_DENOMINATOR: u16 = 10000;
pub const INITIAL_SUPPLY: u64 = 1000000000;
pub const FOUNDER_AND_TEAM: u64 = 1400;
pub const ONE_YEAR_VEST: u64 = 500;
pub const SIX_YEAR_VEST: u64 = 250;
pub const VEST_PERIOD: i64 = 5184000;
pub const FIRST_ROUND_AMOUNT: u64 = 10000000;
pub const SECOND_ROUND_AMOUNT: u64 = 10000000;
pub const THIRD_ROUND_AMOUNT: u64 = 100000000;

pub const FIRST_ROUND_RATE: u64 = 10; //$o.1 per token
pub const SECOND_ROUND_RATE: u64 = 6; //~$0.15 per token
pub const THIRD_ROUND_RATE: u64 = 4; //$0.25 per token
// pub const DECIMALS: u8 = 6;

pub const FOUNDER_SEED: &[u8] = b"founder";
pub const FOUNDER_AIRDROP_PERCENT: u16 = 250;
pub const SIX_YEAR_VEST_PERIOD: i64 = 5184;
pub const PRESALE_VEST_PERIOD: i64 = 7776;
pub const PRESALE_UNLOCK_PERCENT: u16 = 2500;
pub const SIX_YEAR_SEED: &[u8] = b"six-year";
pub const BUYER_SEED: &[u8] = b"buyer";

pub const FIRST_ROUND_SEED: &[u8] = b"first-round";
pub const SECOND_ROUND_SEED: &[u8] = b"second-round";
pub const THIRD_ROUND_SEED: &[u8] = b"third-round";