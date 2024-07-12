use anchor_lang::prelude::*;
use anchor_spl::{token_2022::{self, Burn}, token_interface};

use crate::{ AppState, Round, FIRST_ROUND_AMOUNT };

#[derive(Accounts)]
pub struct FinalizeFirstRound<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(has_one = owner)]
    pub app_state: Box<Account<'info, AppState>>,

    #[account(
        mut,
        mint::authority = authority,
        token::token_program = token_program,
    )]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", mint.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = authority,
        token::token_program = token_program,
    )]
    pub first_round_token: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(mut)]
    pub first_round: Box<Account<'info, Round>>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
}

impl<'info> FinalizeFirstRound<'info> {
    fn to_burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_accounts: Burn = Burn {
            mint: self.mint.to_account_info().clone(),
            from: self.first_round_token.to_account_info().clone(),
            authority: self.authority.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

pub fn finalize_first_round_handler(ctx: Context<FinalizeFirstRound>) -> Result<()> {
    let seeds: &[&[u8]; 3] = &[
        b"authority",
        ctx.accounts.mint.to_account_info().key.as_ref(),
        &[ctx.accounts.app_state.bump],
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    let amount: u64 = FIRST_ROUND_AMOUNT * 10_u64.pow(ctx.accounts.mint.decimals as u32) - ctx.accounts.first_round.total_sold;
    token_2022::burn(ctx.accounts.to_burn_context().with_signer(signer_seeds), amount)?;
    let first_round: &mut Box<Account<Round>> = &mut ctx.accounts.first_round;
    first_round.burned = true;
    Ok(())
}
