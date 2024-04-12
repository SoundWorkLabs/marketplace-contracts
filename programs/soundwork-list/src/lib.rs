pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Bh1Wa72RL4GeCPG3hKzT8W7rmvdp2sf5cbNGUsbbEMoc");

#[program]
pub mod soundwork_list {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
