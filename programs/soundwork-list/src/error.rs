use anchor_lang::prelude::*;

#[error_code]
pub enum ListErrorCode {
    #[msg("Insufficient funds to purchase asset")]
    InsufficientFunds,
}
