use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{transfer_checked, TransferChecked},
    token_interface,
};

use crate::{
    error::ErrorCode, AppState, Buyer, Round, DISTRIBUTION_DENOMINATOR, PRESALE_UNLOCK_PERCENT,
    PRESALE_VEST_PERIOD,
};

#[derive(Accounts)]
pub struct UnlockSecondRound<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut,mint::authority = authority)]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(mut, has_one = user)]
    pub buyer: Box<Account<'info, Buyer>>,

    #[account(mut)]
    pub token_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(mut, constraint = second_round.round_index == 2)]
    pub second_round: Box<Account<'info, Round>>,

    #[account(mut)]
    pub app_state: Box<Account<'info, AppState>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", mint.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = authority
    )]
    pub second_round_token: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    pub token_program: Interface<'info, token_interface::TokenInterface>,
}

impl<'info> UnlockSecondRound<'info> {
    pub fn transfer_checked_ctx(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_program: AccountInfo = self.token_program.to_account_info();
        let cpi_accounts: TransferChecked = TransferChecked {
            from: self.second_round_token.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.authority.to_account_info(),
            mint: self.mint.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn unlock_second_round_handler(ctx: Context<UnlockSecondRound>) -> Result<()> {
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    if ctx.accounts.buyer.claimed_amount2 == 0
        && now < ctx.accounts.second_round.end_time + PRESALE_VEST_PERIOD
    {
        return err!(ErrorCode::InvalidClaimTime);
    }
    if ctx.accounts.buyer.claimed_amount2 > 0
        && ctx.accounts.buyer.last_claim2 + PRESALE_VEST_PERIOD < now
    {
        return err!(ErrorCode::InvalidClaimTime);
    }
    let mut amount: u64 = ctx.accounts.buyer.total_bought2
        / DISTRIBUTION_DENOMINATOR as u64
        / PRESALE_UNLOCK_PERCENT as u64;
    let left: u64 = ctx.accounts.buyer.total_bought2 - ctx.accounts.buyer.claimed_amount2;
    if amount > left {
        amount = left;
    }
    let seeds: &[&[u8]; 3] = &[
        b"authority",
        ctx.accounts.mint.to_account_info().key.as_ref(),
        &[ctx.accounts.app_state.bump],
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    transfer_checked(
        ctx.accounts
            .transfer_checked_ctx()
            .with_signer(signer_seeds),
        amount,
        ctx.accounts.mint.decimals,
    )?;
    let buyer: &mut Box<Account<Buyer>> = &mut ctx.accounts.buyer;
    buyer.claimed_amount2 += amount;
    buyer.last_claim2 = now;
    Ok(())
}
