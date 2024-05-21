use anchor_lang::prelude::*;

#[error_code]
pub enum ListErrorCode {
    #[msg("Insufficient funds to purchase asset")]
    InsufficientFunds,
    #[msg("Invalid operation!")]
    InvalidOperation,
    #[msg("You do no have authority to perform the requested operation!")]
    InvalidAuthority,
    #[msg("The value provided should not be zero.")]
    ZeroValueNotAllowed,
}
