use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Already minted")]
    AlreadyMinted,
    #[msg("Invalid claim time")]
    InvalidClaimTime,
}
