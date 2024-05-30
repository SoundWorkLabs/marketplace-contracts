use anchor_lang::prelude::*;

#[account]
pub struct BidData {
    /// PDA bump
    pub bump: u8,

    /// amount in lamports asset is being listed for
    pub amount: u64,

    /// asset owner
    pub authority: Pubkey,

    /// unix timestamp listing expires
    pub expiry_ts: i64,

    ///  Unused reserved byte space for additive future changes.
    pub _reserved: [u8; 128],
}

impl BidData {
    pub const LEN: usize = 8 // anchor discriminator 
    + 1 // bump 
    + 8 // amount 
    + 32 // authority address
    + 8 // expiry timestamp 
    + 128; // reserved

    /// instantiate the bid data account with provided args
    pub fn new(bump: u8, amount: u64, authority: Pubkey, expiry_ts: i64) -> Self {
        Self {
            bump,
            amount,
            authority,
            expiry_ts,
            _reserved: [0; 128],
        }
    }

    /// update bid data account
    pub fn update(
        &mut self,
        updated_amount: Option<u64>,
        updated_expiry_ts: Option<i64>,
    ) -> (u64, i64) {
        let amount = updated_amount.unwrap_or(self.amount);

        let expiry_ts = updated_expiry_ts.unwrap_or(self.expiry_ts);

        (amount, expiry_ts)
    }
}
