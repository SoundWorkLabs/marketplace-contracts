use anchor_lang::prelude::*;

/// When listed, how does the user want to receive funds
///
#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub enum PaymentOption {
    #[default]
    Native,
    Token {
        mint: Pubkey,
    },
    // todo(both) a combination of tokens and sol
}

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

    /// type of way user wants to get paid when listing is bought / bid made for asset
    pub payment_option: PaymentOption,

    ///  Unused reserved byte space for additive future changes.
    pub _reserved: [u8; 128],
}

impl ListingData {
    pub const LEN: usize = 8 + (1 + 8 + 32 + 32 + 8 + 33 + 128);

    /// instantiate the listing data account with provided args
    pub fn new(
        bump: u8,
        amount: u64,
        owner: Pubkey,
        asset_address: Pubkey,
        payment_option: PaymentOption,
    ) -> Self {
        let created_ts = Clock::get().unwrap().unix_timestamp;

        Self {
            bump,
            amount,
            owner,
            created_ts,
            asset_address,
            payment_option,
            _reserved: [0; 128],
        }
    }

    // update listing data account
    pub fn update_amount(amount: u64) -> u64 {
        amount
    }
}
