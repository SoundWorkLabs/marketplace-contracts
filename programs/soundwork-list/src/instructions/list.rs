use anchor_lang::prelude::*;
use mpl_core::instructions::TransferV1CpiBuilder;

use crate::{
    constants::{SEED_LISTING_DATA, SEED_PREFIX},
    AssetManager, ListingData, PaymentOption,
};

// todo: remove and use SPL typed account
#[derive(Clone)]
pub struct Core;

impl anchor_lang::Id for Core {
    fn id() -> Pubkey {
        mpl_core::ID
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ListTokenParams {
    ///  listing amount/price in lamports
    pub amount: u64,

    /// which method can be used to purchase the listed asset
    pub payment_option: PaymentOption,
}

/// LIST an MPL core asset on soundwork
///
/// Expects the following accounts:
/// 1. `[writeable, signer]` payer
/// 2. `[writeable]` asset
/// 3. `[writeable]` listing data account
/// 4. `[]` asset manager
/// 5. `[]` core program
/// 6. `[]` system program
///
/// Expects the following arguments
/// 1. params: ListTokenParams
///
#[derive(Accounts)]
#[instruction(params: ListTokenParams)]
pub struct ListAsset<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: checked by us
    #[account(mut)]
    pub asset: AccountInfo<'info>,

    /// CHECK: checked by us
    #[account(
        init,
        payer = payer,
        space = ListingData::LEN,
        seeds = [SEED_PREFIX, SEED_LISTING_DATA, asset.key().as_ref()],
        bump
    )]
    pub listing_data: Account<'info, ListingData>,

    pub asset_manager: Account<'info, AssetManager>,

    pub core_program: Program<'info, Core>,

    pub system_program: Program<'info, System>,
}

impl ListAsset<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// list MPL core asset on the marketplace
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn list_asset(ctx: Context<ListAsset>, params: ListTokenParams) -> Result<()> {
        let listing_data = &mut ctx.accounts.listing_data;

        **listing_data = ListingData::new(
            ctx.bumps.listing_data,
            params.amount,
            ctx.accounts.payer.key(),
            ctx.accounts.asset.key(),
            params.payment_option,
        );

        // transfer to assetManager

        TransferV1CpiBuilder::new(&ctx.accounts.core_program)
            .asset(&ctx.accounts.asset)
            .payer(&ctx.accounts.payer)
            .authority(Some(&ctx.accounts.payer))
            .new_owner(&ctx.accounts.asset_manager.to_account_info())
            .system_program(Some(&ctx.accounts.system_program))
            .invoke()?;

        Ok(())
    }
}
