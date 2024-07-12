pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CyRCQokSXZ9iuZemxjJVDnF4FDdbTRoQ313EyzxDCqKz");

#[program]
pub mod fnet {
    use super::*;

    pub fn mint_founder(ctx: Context<MintFounder>, decimals: u8, bump: u8) -> Result<()> {
        mint_founder_handler(ctx, decimals, bump)
    }

    pub fn mint_one_year(ctx: Context<MintOneYear>) -> Result<()> {
        mint_one_year_handler(ctx)
    }

    pub fn mint_six_year(ctx: Context<MintSixYear>) -> Result<()> {
        mint_six_year_handler(ctx)
    }

    pub fn create_first_round(
        ctx: Context<CreateFirstRound>,
        start_time: i64,
        end_time: i64,
    ) -> Result<()> {
        create_first_round_handler(ctx, start_time, end_time)
    }
    pub fn create_second_round(
        ctx: Context<CreateSecondRound>,
        start_time: i64,
        end_time: i64,
    ) -> Result<()> {
        create_second_round_handler(ctx, start_time, end_time)
    }

    pub fn create_third_round(
        ctx: Context<CreateThirdRound>,
        start_time: i64,
        end_time: i64,
    ) -> Result<()> {
        create_third_round_handler(ctx, start_time, end_time)
    }

    pub fn buy_in_first_round(ctx: Context<BuyInFirstRound>, amount: u64) -> Result<()> {
        buy_in_first_round_handler(ctx, amount)
    }

    pub fn buy_in_second_round(ctx: Context<BuyInSecondRound>, amount: u64) -> Result<()> {
        buy_in_second_round_handler(ctx, amount)
    }

    pub fn buy_in_third_round(ctx: Context<BuyInThirdRound>, amount: u64) -> Result<()> {
        buy_in_third_round_handler(ctx, amount)
    }

    pub fn finalize_first_round(ctx: Context<FinalizeFirstRound>) -> Result<()> {
        finalize_first_round_handler(ctx)
    }

    pub fn finalize_second_round(ctx: Context<FinalizeSecondRound>) -> Result<()> {
        finalize_second_round_handler(ctx)
    }

    pub fn finalize_third_round(ctx: Context<FinalizeThirdRound>) -> Result<()> {
        finalize_third_round_handler(ctx)
    }

    pub fn withdraw_currency_from_presale(
        ctx: Context<WithdrawCurrencyPresale>,
        amount: u64,
    ) -> Result<()> {
        withdraw_currency_from_presale_handler(ctx, amount)
    }

    pub fn unlock_six_year(ctx: Context<UnlockSixYear>) -> Result<()> {
        unlock_six_year_handler(ctx)
    }

    pub fn unlock_first_round(ctx: Context<UnlockFirstRound>) -> Result<()> {
        unlock_first_round_handler(ctx)
    }

    pub fn unlock_second_round(ctx: Context<UnlockSecondRound>) -> Result<()> {
        unlock_second_round_handler(ctx)
    }

    pub fn unlock_third_round(ctx: Context<UnlockThirdRound>) -> Result<()> {
        unlock_third_round_handler(ctx)
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
        transfer_ownership_handler(ctx)
    }

    pub fn mint_by_owner(ctx: Context<MintByOwner>, amount: u64) -> Result<()> {
        mint_by_owner_handler(ctx, amount)
    }
}
