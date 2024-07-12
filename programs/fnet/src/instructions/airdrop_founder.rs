use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_2022::{transfer_checked, TransferChecked}, token_interface
};

use crate::{AppState, Founder, DISTRIBUTION_DENOMINATOR, FOUNDER_AIRDROP_PERCENT};

#[derive(Accounts)]
pub struct AirdropFounder<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut,mint::authority = authority)]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(mut)]
    pub user_token_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    #[account(mut)]
    pub app_state: Box<Account<'info, AppState>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", mint.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(mut, has_one = user)]
    pub founder: Box<Account<'info, Founder>>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = authority
    )]
    pub founder_token: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> AirdropFounder<'info> {
    pub fn transfer_checked_ctx(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_program: AccountInfo = self.token_program.to_account_info();
        let cpi_accounts: TransferChecked = TransferChecked {
            from: self.founder_token.to_account_info(),
            to: self.user_token_account.to_account_info(),
            authority: self.authority.to_account_info(),
            mint: self.mint.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn airdrop_founder_handler(ctx: Context<AirdropFounder>) -> Result<()> {
    let amount: u64 = ctx.accounts.app_state.founder_amount * FOUNDER_AIRDROP_PERCENT as u64
        / DISTRIBUTION_DENOMINATOR as u64
        / ctx.accounts.app_state.founder_count;
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
    let founder: &mut Box<Account<Founder>> = &mut ctx.accounts.founder;
    founder.withdrawn += amount;
    Ok(())
}
