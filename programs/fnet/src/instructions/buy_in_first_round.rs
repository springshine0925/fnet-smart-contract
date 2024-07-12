use anchor_lang::prelude::*;
use anchor_spl::{token::{ self, Mint, Token, TokenAccount, Transfer }, token_interface};

use std::mem::size_of;

use crate::{ AppState, Buyer, Round, BUYER_SEED, FIRST_ROUND_RATE };

#[derive(Accounts)]
pub struct BuyInFirstRound<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, constraint = first_round.round_index == 1)]
    pub first_round: Box<Account<'info, Round>>,

    pub app_state: Box<Account<'info, AppState>>,

    #[account(
        init_if_needed, 
        payer = signer, 
        space = size_of::<Buyer>() + 8,
        seeds = [BUYER_SEED, signer.key().as_ref()],
        bump,
    )]
    pub buyer: Box<Account<'info, Buyer>>,

    #[account(
        token::token_program = token_program_mint,
    )]
    pub mint: Box<InterfaceAccount<'info, token_interface::Mint>>,
    pub currency_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        token::token_program = token_program,
    )]
    pub user_currency_account: Box<Account<'info, TokenAccount>>,
   
    #[account(mut,
        token::mint = mint,
        token::token_program = token_program_mint,
    )]
    pub first_round_token: Box<InterfaceAccount<'info, token_interface::TokenAccount>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(seeds = [b"authority", mint.key().as_ref()], bump)]
    pub authority: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [b"currency-pot", currency_mint.key().as_ref()],
        bump,
        owner = token_program.key(),
        rent_exempt = enforce,
        token::mint = currency_mint,
        token::authority = authority
    )]
    pub currency_pot: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub token_program_mint: Interface<'info, token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> BuyInFirstRound<'info> {
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(self.token_program.to_account_info(), Transfer {
            from: self.user_currency_account.to_account_info(),
            to: self.currency_pot.to_account_info(),
            authority: self.signer.to_account_info(),
        })
    }
}

pub fn buy_in_first_round_handler(ctx: Context<BuyInFirstRound>, amount: u64) -> Result<()> {
    token::transfer(ctx.accounts.transfer_ctx(), amount)?;
    let buyer: &mut Box<Account<Buyer>> = &mut ctx.accounts.buyer;
    let round: &mut Box<Account<Round>> = &mut ctx.accounts.first_round;
    if buyer.total_bought1 == 0 {
        round.buyer_count += 1;
    }
    buyer.total_paid += amount;
    let offer_amount: u64 = amount / 10_u64.pow(ctx.accounts.currency_mint.decimals as u32) * FIRST_ROUND_RATE * 10_u64.pow(ctx.accounts.mint.decimals as u32);
        
    buyer.total_bought1 += offer_amount;
    round.total_sold += offer_amount;
    round.total_collected += amount;
    Ok(())
}
