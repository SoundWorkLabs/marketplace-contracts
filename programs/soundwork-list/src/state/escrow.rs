use anchor_lang::prelude::*;

#[account]
pub struct AssetManager {
    /// PDA bump
    pub bump: u8,

    ///  Unused reserved byte space for additive future changes.
    pub _reserved: [u8; 128],
}

impl AssetManager {
    pub const LEN: usize = 
      8 +  // anchor account discriminator 
      1 +  // PDA bump
      130  // reserved space
     ;
}