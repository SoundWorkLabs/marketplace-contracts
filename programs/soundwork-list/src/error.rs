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
    #[msg("The mint address provided does not match seller's provided mint address.")]
    PaymentMintAddressMismatch,
    #[msg("An account required for this operation is missing.")]
    MissingAccount,
}
