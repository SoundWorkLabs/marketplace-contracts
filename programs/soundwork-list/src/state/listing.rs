use anchor_lang::prelude::*;

#[account]
pub struct ListingData {
    /// PDA bump
    pub bump: u8,

    /// amount in lamports asset is being listed for
    pub amount: u64,

    /// asset owner
    pub owner: Pubkey,

    /// asset address
    pub asset_address: Pubkey,

    /// unix timestamp listing is being made
    pub created_ts: i64,

    ///  Unused reserved byte space for additive future changes.
    pub _reserved: [u8; 128],
}

impl ListingData {
    pub const LEN: usize = 8 + (1 + 8 + 32 + 32 + 8 + 128);

    /// instantiate the listing data account with provided args
    pub fn new(bump: u8, amount: u64, owner: Pubkey, asset_address: Pubkey) -> Self {
        let created_ts = Clock::get().unwrap().unix_timestamp;

        Self {
            bump,
            amount,
            owner,
            created_ts,
            asset_address,
            _reserved: [0; 128],
        }
    }

    // update listing data account
    pub fn update_amount(amount: u64) -> u64 {
        amount
    }
}
