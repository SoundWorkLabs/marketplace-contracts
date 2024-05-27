use anchor_lang::prelude::*;

#[error_code]
pub enum BidErrorCode {
    #[msg("Signer address does not math the initializer address")]
    UnrecognizedSigner,
    #[msg("Bid TimeStamp Expired")]
    BidExpired,
    #[msg("Insufficient Funds to make bid for item")]
    InsufficientFunds,
    #[msg("An account required for this operation is missing.")]
    MissingAccount,
    #[msg("The mint address provided does not match seller's provided mint address.")]
    PaymentMintAddressMismatch,
    #[msg("Operations resulted in an overflow.")]
    Overflow,
}
