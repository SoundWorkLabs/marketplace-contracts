pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GfK5B7Njeagu5GCeBGdVgpGzLcD8BpMDkcLeQjoXJBmY");

#[program]
pub mod soundwork_bid {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
