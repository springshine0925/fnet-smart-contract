use anchor_lang::prelude::*;
use anchor_spl::{token_2022::{mint_to, MintTo}, token_interface};

use crate::AppState;

#[derive(Accounts)]
pub struct MintByOwner<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(has_one = owner)]
    pub app_state: Box<Account<'info, AppState>>,

    #[account(seeds = [b"authority", mint.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(mut,mint::authority = authority)]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,

    #[account(
      mut,
      token::mint = mint,
    )]
    pub owner_token_account: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    pub token_program: Interface<'info, token_interface::TokenInterface>,
}

impl <'info> MintByOwner <'info> {
  pub fn mint_ctx(&self) -> CpiContext<'info, 'info, 'info, 'info, MintTo<'info>> {
    CpiContext::new(
        self.token_program.to_account_info(),
        MintTo {
            mint: self.mint.to_account_info(),
            to: self.owner_token_account.to_account_info(),
            authority: self.authority.to_account_info(),
        },
    )
}
}


pub fn mint_by_owner_handler(
  ctx: Context<MintByOwner>,
  amount: u64,
) -> Result<()> {
  let seeds: &[&[u8]; 3] = &[
        b"authority",
        ctx.accounts.mint.to_account_info().key.as_ref(),
        &[ctx.accounts.app_state.bump],
    ];
    let signer_seeds: &[&[&[u8]]; 1] = &[&seeds[..]];
  mint_to(ctx.accounts.mint_ctx().with_signer(signer_seeds), amount)?;
  Ok(())
}