pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("4mFDYND4AVREYEJXCPhjq1LnbjELHHebJqG3NZechA7X");

#[program]
pub mod soundwork_bid {
    use super::*;

    /// Place a bid for a listed MPL Core asset on Soundwork
    ///
    pub fn make_bid(ctx: Context<MakeBid>, params: MakeBidParams) -> Result<()> {
        MakeBid::make_bid(ctx, params)
    }

    /// Edit a placed bid on Soundwork
    ///
    pub fn edit_bid(ctx: Context<EditBid>, params: EditBidParams) -> Result<()> {
        EditBid::edit_bid(ctx, params)
    }

    /// Revoke placed bid  
    ///
    pub fn revoke_bid(ctx: Context<RevokeBid>) -> Result<()> {
        RevokeBid::revoke_bid(ctx)
    }

    /// Accept placed bid  
    ///
    pub fn accept_bid(ctx: Context<AcceptBid>) -> Result<()> {
        AcceptBid::accept_bid(ctx)
    }

    /// Reject placed bid  
    ///
    pub fn reject_bid(ctx: Context<RejectBid>) -> Result<()> {
        RejectBid::reject_bid(ctx)
    }
}
