use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{mint_to, MintTo},
    token_interface,
};

use crate::{error::ErrorCode, SixYearState, SIX_YEAR_SEED};
use crate::{AppState, DISTRIBUTION_DENOMINATOR, INITIAL_SUPPLY, SIX_YEAR_VEST};
use std::mem::size_of;

#[derive(Accounts)]
pub struct MintSixYear<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut,mint::authority = authority)]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(mut)]
    pub app_state: Box<Account<'info, AppState>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", mint.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(
        init,
        payer = owner,
        rent_exempt = enforce,
        token::mint = mint,
        token::authority = authority
    )]
    pub six_year_token: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(
        init,
        payer = owner,
        space = size_of::<SixYearState>() + 8,
        seeds = [SIX_YEAR_SEED, mint.key().as_ref()],
        bump,
    )]
    pub six_year_state: Box<Account<'info, SixYearState>>,

    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintSixYear<'info> {
    pub fn mint_founder_ctx(&self) -> CpiContext<'info, 'info, 'info, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.mint.to_account_info(),
                to: self.six_year_token.to_account_info(),
                authority: self.authority.to_account_info(),
            },
        )
    }
}

pub fn mint_six_year_handler(ctx: Context<MintSixYear>) -> Result<()> {
    if ctx.accounts.app_state.minted_six_year {
        return err!(ErrorCode::AlreadyMinted);
    }
    let amount: u64 = INITIAL_SUPPLY / DISTRIBUTION_DENOMINATOR as u64
        * SIX_YEAR_VEST as u64
        * (10_u64.pow(ctx.accounts.mint.decimals as u32));
    let seeds: &[&[u8]; 3] = &[
        b"authority",
        ctx.accounts.mint.to_account_info().key.as_ref(),
        &[ctx.accounts.app_state.bump],
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    mint_to(
        ctx.accounts.mint_founder_ctx().with_signer(signer_seeds),
        amount,
    )?;
    let app_state: &mut Box<Account<AppState>> = &mut ctx.accounts.app_state;
    app_state.minted_six_year = true;
    app_state.six_year_token_account = ctx.accounts.six_year_token.key();
    let six_year_state: &mut Box<Account<SixYearState>> = &mut ctx.accounts.six_year_state;
    six_year_state.total_amount = amount;
    six_year_state.claimed_amount = 0;
    six_year_state.last_claim = Clock::get().unwrap().unix_timestamp;
    Ok(())
}
