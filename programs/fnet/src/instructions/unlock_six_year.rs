use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{transfer_checked, TransferChecked},
    token_interface,
};

use crate::{error::ErrorCode, AppState, SixYearState, DISTRIBUTION_DENOMINATOR, INITIAL_SUPPLY, SIX_YEAR_SEED, SIX_YEAR_VEST, SIX_YEAR_VEST_PERIOD};

#[derive(Accounts)]
pub struct UnlockSixYear<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut,mint::authority = authority)]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(mut)]
    pub token_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(
        mut,
        seeds = [SIX_YEAR_SEED, mint.key().as_ref()],
        bump,
    )]
    pub six_year_state: Box<Account<'info, SixYearState>>,

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
    pub six_year_token: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> UnlockSixYear<'info> {
    pub fn transfer_checked_ctx(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_program: AccountInfo = self.token_program.to_account_info();
        let cpi_accounts: TransferChecked = TransferChecked {
            from: self.six_year_token.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.authority.to_account_info(),
            mint: self.mint.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn unlock_six_year_handler(ctx: Context<UnlockSixYear>,) -> Result<()> {
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    if ctx.accounts.six_year_state.last_claim + SIX_YEAR_VEST_PERIOD > now {
        return err!(ErrorCode::InvalidClaimTime);
    }
    let seeds: &[&[u8]; 3] = &[
        b"authority",
        ctx.accounts.mint.to_account_info().key.as_ref(),
        &[ctx.accounts.app_state.bump],
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    let amount: u64 = INITIAL_SUPPLY / DISTRIBUTION_DENOMINATOR as u64
    * SIX_YEAR_VEST as u64
    * (10 ^ ctx.accounts.mint.decimals as u64) / DISTRIBUTION_DENOMINATOR as u64 * 250;
    transfer_checked(
        ctx.accounts
            .transfer_checked_ctx()
            .with_signer(signer_seeds),
        amount,
        ctx.accounts.mint.decimals,
    )?;
    let six_year_state: &mut Box<Account<SixYearState>> = &mut ctx.accounts.six_year_state;
    six_year_state.claimed_amount += amount;
    six_year_state.last_claim = now;
    Ok(())
}
