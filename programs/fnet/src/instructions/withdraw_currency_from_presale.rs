use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Mint, Token, TokenAccount, Transfer };

use crate::AppState;

#[derive(Accounts)]
pub struct WithdrawCurrencyPresale<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub currency_mint: Box<Account<'info, Mint>>,
    pub mint: Box<Account<'info, Mint>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", mint.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub owner_currency_token: Box<Account<'info, TokenAccount>>,

    #[account(has_one = owner)]
    pub app_state: Box<Account<'info, AppState>>,

    #[account(
      mut,
      seeds = [b"currency-pot", currency_mint.key().as_ref()],
      bump,
      token::mint = currency_mint,
      token::authority = authority
  )]
    pub currency_pot: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawCurrencyPresale<'info> {
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(self.token_program.to_account_info(), Transfer {
            from: self.currency_pot.to_account_info(),
            to: self.owner_currency_token.to_account_info(),
            authority: self.authority.to_account_info(),
        })
    }
}

pub fn withdraw_currency_from_presale_handler(
    ctx: Context<WithdrawCurrencyPresale>,
    amount: u64
) -> Result<()> {
    let seeds: &[&[u8]; 3] = &[
        b"authority",
        ctx.accounts.mint.to_account_info().key.as_ref(),
        &[ctx.accounts.app_state.bump],
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
    token::transfer(ctx.accounts.transfer_ctx().with_signer(signer_seeds), amount)?;
    Ok(())
}
