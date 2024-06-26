use anchor_lang::prelude::*;

use crate::{helpers::Core, AssetManager, ListingData};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateListingParams {
    /// edit listing amount/price in lamports
    pub amount: u64,
}

/// update a listed MPL core asset
///
/// ### Accounts:
///
/// 1. `[writeable, signer]` payer
/// 2. `[writeable]` asset
/// 3. `[writeable]` listing data account
/// 4. `[]` asset manager
/// 5. `[]` core program
/// 6. `[]` system program
///
/// ### Params
///
/// 1. params: [UpdateListingParams]
///
#[derive(Accounts)]
#[instruction(params: UpdateListingParams)]
pub struct UpdateListing<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: checked by us
    #[account(mut)]
    pub asset: AccountInfo<'info>,

    /// CHECK: checked by us
    #[account(mut)]
    pub listing_data: Account<'info, ListingData>,

    pub asset_manager: Account<'info, AssetManager>,

    pub core_program: Program<'info, Core>,

    pub system_program: Program<'info, System>,
}

impl UpdateListing<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// list MPL core asset on the marketplace
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn update_listing(ctx: Context<UpdateListing>, params: UpdateListingParams) -> Result<()> {
        let listing_data = &mut ctx.accounts.listing_data;

        listing_data.amount = ListingData::update_amount(params.amount);

        Ok(())
    }
}
