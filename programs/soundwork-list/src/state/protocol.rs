use anchor_lang::prelude::*;

#[account]
pub struct MarketPlaceConfig {
    /// PDA bump
    pub bump: u8,

    /// Taker fee percentage
    pub treasury_address: Pubkey,

    /// Taker fee basis points
    pub taker_fee_bps: u8,

    ///  Unused reserved byte space for additive future changes.
    pub _reserved: [u8; 128],
}

impl MarketPlaceConfig {
    pub const LEN: usize = 8 + // anchor account discriminator
        1 + // PDA bump
        32 + //  treasury address
        1 + // taker fee percentage
        130; // reserved space

    pub fn new(bump: u8, treasury_address: Pubkey, taker_fee_bps: u8) -> Self {
        Self {
            bump,
            treasury_address,
            taker_fee_bps,
            _reserved: [0; 128],
        }
    }
}
