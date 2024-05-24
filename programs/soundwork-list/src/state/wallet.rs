use anchor_lang::prelude::*;

#[account]
pub struct Wallet {
    pub authority: Pubkey,
    pub bump: u8,
    pub _reserved: [u8; 128],
}

impl Wallet {
    pub const LEN: usize = 8 // discriminator
    + 32  // owner address
    + 1  // wallet bump
    + 130; // reserved space

    // todo: (Jimii) should size be changed to accommodate paying with tokens"
    pub fn new(authority: &Pubkey, bump: u8) -> Self {
        Self {
            authority: authority.to_owned(),
            bump,
            _reserved: [0; 128],
        }
    }
}
