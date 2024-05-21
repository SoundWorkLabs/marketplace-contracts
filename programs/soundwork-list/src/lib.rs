pub mod constants;
pub mod error;
pub mod helpers;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EA4ptgF3TYjDBGYJApAoZoyCbCYw6P5mGU5noCe1Z97");

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

    /// Initialize user escrow wallet.
    ///
    pub fn init_user_escrow_wallet(ctx: Context<InitWallet>) -> Result<()> {
        InitWallet::init_wallet(ctx)
    }

    /// Deposit SOL into the user escrow wallet.
    ///
    pub fn deposit_sol(ctx: Context<DepositSol>, params: DepositSolParams) -> Result<()> {
        DepositSol::deposit_sol(ctx, params)
    }

    /// Withdraw SOL into the user's escrow wallet.
    ///
    pub fn withdraw_sol(
        ctx: Context<WithdrawSol>,
        params: Option<WithdrawSolParams>,
    ) -> Result<()> {
        WithdrawSol::withdraw_sol(ctx, params)
    }

    /// Deposit SOL into the user escrow wallet.
    ///
    pub fn deposit_token(ctx: Context<DepositToken>, params: DepositTokenParams) -> Result<()> {
        DepositToken::deposit_token(ctx, params)
    }

    /// Withdraw tokens from the user escrow wallet.
    ///
    pub fn withdraw_token(ctx: Context<WithdrawToken>, params: WithdrawTokenParams) -> Result<()> {
        WithdrawToken::withdraw_token(ctx, params)
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
