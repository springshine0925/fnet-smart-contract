use anchor_lang::prelude::*;
use anchor_spl::token_2022::{mint_to, MintTo};
use anchor_spl::{associated_token::AssociatedToken, token_interface};

use crate::error::ErrorCode;
use crate::{AppState, DISTRIBUTION_DENOMINATOR, INITIAL_SUPPLY, ONE_YEAR_VEST};

#[derive(Accounts)]
pub struct MintOneYear<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut,mint::authority = authority)]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(mut, has_one = owner)]
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
    pub one_year_token: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintOneYear<'info> {
    pub fn mint_one_year_ctx(&self) -> CpiContext<'info, 'info, 'info, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.mint.to_account_info(),
                to: self.one_year_token.to_account_info(),
                authority: self.authority.to_account_info(),
            },
        )
    }
}

pub fn mint_one_year_handler(ctx: Context<MintOneYear>) -> Result<()> {
    if ctx.accounts.app_state.minted_one_year {
        return err!(ErrorCode::AlreadyMinted);
    }
    let amount: u64 = INITIAL_SUPPLY / DISTRIBUTION_DENOMINATOR as u64
        * ONE_YEAR_VEST as u64
        * (10_u64.pow(ctx.accounts.mint.decimals as u32));
    let seeds: &[&[u8]; 3] = &[
        b"authority",
        ctx.accounts.mint.to_account_info().key.as_ref(),
        &[ctx.accounts.app_state.bump],
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    mint_to(
        ctx.accounts.mint_one_year_ctx().with_signer(signer_seeds),
        amount,
    )?;
    let app_state: &mut Box<Account<AppState>> = &mut ctx.accounts.app_state;
    app_state.minted_one_year = true;
    app_state.one_year_token_account = ctx.accounts.one_year_token.key();
    Ok(())
}
