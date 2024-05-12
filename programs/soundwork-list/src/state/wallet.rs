use anchor_lang::prelude::*;

#[account]
pub struct Wallet {
    pub owner: Pubkey,
}

impl Wallet {
    pub const LEN: usize = (8 + 32);

    // todo: (Jimii) should size be changed to accommodate paying with tokens" 
    pub fn new(owner: Pubkey) -> Self {
        Self { owner }
    }
}
