use anchor_lang::prelude::*;
use anchor_spl::token_2022::MintTo;
use anchor_spl::{associated_token::AssociatedToken, token_2022::mint_to, token_interface};

use crate::error::ErrorCode;
use crate::{AppState, DISTRIBUTION_DENOMINATOR, FOUNDER_AND_TEAM, INITIAL_SUPPLY};
use std::mem::size_of;

#[derive(Accounts)]
pub struct MintFounder<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        mint::authority = authority,
        token::token_program = token_program
    )]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(
        init,
        payer = signer,
        space = size_of::<AppState>() + 8,
        seeds = [b"app-state", mint.key().as_ref()],
        bump
    )]
    pub app_state: Box<Account<'info, AppState>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", mint.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(
        init,
        payer = signer,
        token::mint = mint,
        token::authority = authority,
        token::token_program = token_program
    )]
    pub founder_token: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintFounder<'info> {
    pub fn mint_founder_ctx(&self) -> CpiContext<'info, 'info, 'info, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.mint.to_account_info(),
                to: self.founder_token.to_account_info(),
                authority: self.authority.to_account_info(),
            },
        )
    }
}

pub fn mint_founder_handler(ctx: Context<MintFounder>, decimals: u8, bump: u8) -> Result<()> {
    if ctx.accounts.app_state.minted_founder {
        return err!(ErrorCode::AlreadyMinted);
    }
    let seeds: &[&[u8]; 3] = &[
        b"authority",
        ctx.accounts.mint.to_account_info().key.as_ref(),
        &[bump],
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    let amount_for_founder: u64 = INITIAL_SUPPLY / DISTRIBUTION_DENOMINATOR as u64
        * FOUNDER_AND_TEAM as u64
        * (10_u64.pow(decimals as u32) );
    mint_to(
        ctx.accounts.mint_founder_ctx().with_signer(signer_seeds),
        amount_for_founder,
    )?;
    let app_state: &mut Box<Account<AppState>> = &mut ctx.accounts.app_state;
    app_state.bump = bump;
    app_state.minted_founder = true;
    app_state.founder_token_account = ctx.accounts.founder_token.key();
    app_state.owner = ctx.accounts.signer.key();
    app_state.founder_amount = amount_for_founder;
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    app_state.minted_time = now;
    Ok(())
}
