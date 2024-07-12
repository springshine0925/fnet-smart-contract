use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token::Mint, token_2022::{mint_to, MintTo}, token_interface
};

use std::mem::size_of;

use crate::{ AppState, Round, FIRST_ROUND_AMOUNT, FIRST_ROUND_SEED };

#[derive(Accounts)]
pub struct CreateFirstRound<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(has_one = owner)]
    pub app_state: Box<Account<'info, AppState>>,

    #[account(mut, mint::authority = authority)]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,
    pub currency_mint: Box<Account<'info, Mint>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", mint.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(
        init, 
        payer = owner, 
        space = size_of::<Round>() + 8,
        seeds = [FIRST_ROUND_SEED, app_state.key().as_ref()],
        bump
    )]
    pub first_round: Box<Account<'info, Round>>,

    #[account(
        init,
        payer = owner,
        rent_exempt = enforce,
        token::mint = mint,
        token::authority = authority,
        token::token_program = token_program,
    )]
    pub first_round_token: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,
  
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateFirstRound<'info> {
    pub fn mint_ctx(&self) -> CpiContext<'info, 'info, 'info, 'info, MintTo<'info>> {
        CpiContext::new(self.token_program.to_account_info(), MintTo {
            mint: self.mint.to_account_info(),
            to: self.first_round_token.to_account_info(),
            authority: self.authority.to_account_info(),
        })
    }
}

pub fn create_first_round_handler(
    ctx: Context<CreateFirstRound>,
    start_time: i64,
    end_time: i64,
) -> Result<()> {
    let seeds: &[&[u8]; 3] = &[
        b"authority",
        ctx.accounts.mint.to_account_info().key.as_ref(),
        &[ctx.accounts.app_state.bump],
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    mint_to(ctx.accounts.mint_ctx().with_signer(signer_seeds), FIRST_ROUND_AMOUNT * (10_u64.pow(ctx.accounts.mint.decimals as u32)))?;
    let first_round: &mut Box<Account<Round>> = &mut ctx.accounts.first_round;
    first_round.start_time = start_time;
    first_round.end_time = end_time;
    first_round.total_collected = 0;
    first_round.total_sold = 0;
    first_round.currency.mint = ctx.accounts.currency_mint.key();
    first_round.token_account = ctx.accounts.first_round_token.key();
    first_round.burned = false;
    first_round.round_index = 1;
    first_round.buyer_count = 0;
    Ok(())
}
