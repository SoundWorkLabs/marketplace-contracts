pub mod constants;
pub mod error;
pub mod helpers;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use helpers::*;
pub use instructions::*;
pub use state::*;

declare_id!("Cdn2CtPiYR9Lar4JnzhQbY3Gy4s6xYVjQLy3NBvZAN6k");

#[program]
pub mod soundwork_list {

    use super::*;

    /// Initialize asset manager escrow account.
    ///
    /// Note: Only admin address can call this function
    ///
    pub fn init_escrow_account(ctx: Context<InitEscrow>) -> Result<()> {
        InitEscrow::init_escrow(ctx)
    }

    /// Initialize marketplace config account.
    ///
    /// Note: Only admin address can call this function
    ///
    pub fn init_marketplace_config_account(
        ctx: Context<InitMarketplaceConfig>,
        params: InitMarketPlaceConfigParams,
    ) -> Result<()> {
        InitMarketplaceConfig::init_marketplace_config(ctx, params)
    }

    /// List an MPL Core asset on Soundwork
    ///
    /// Expect
    /// 1. amount - listing amount/price in lamports
    pub fn list_asset(ctx: Context<ListAsset>, params: ListTokenParams) -> Result<()> {
        ListAsset::list_asset(ctx, params)
    }

    /// Remove MPL Core asset listed on our marketplace
    ///
    pub fn update_listing_amount(
        ctx: Context<UpdateListing>,
        params: UpdateListingParams,
    ) -> Result<()> {
        UpdateListing::update_listing(ctx, params)
    }
    /// Remove MPL Core asset listed on our marketplace
    ///
    pub fn unlist_asset(ctx: Context<UnListAsset>) -> Result<()> {
        UnListAsset::unlist(ctx)
    }
}
