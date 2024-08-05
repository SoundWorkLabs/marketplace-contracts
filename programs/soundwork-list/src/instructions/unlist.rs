use anchor_lang::prelude::*;
use mpl_core::instructions::TransferV1CpiBuilder;

use crate::{constants::SEED_PREFIX, helpers::Core, AssetManager, ListingData, SEED_ASSET_MANAGER};

/// Un-list an MPL core asset on soundwork
///
/// ### Accounts
///
/// 1. `[writable, signer]` payer
/// 2. `[writable]` asset
/// 3. `[writable]` collection
/// 4. `[writable]` listing data account
/// 5. `[]` asset manager
/// 6. `[]` core program
/// 7. `[]` system program
///
#[derive(Accounts)]
pub struct UnListAsset<'info> {
    #[account(mut, address = listing_data.authority)]
    pub payer: Signer<'info>,

    /// CHECK: checked by us
    #[account(mut)]
    pub asset: AccountInfo<'info>,

    /// CHECK: checked by us
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,

    #[account(mut, close = payer)]
    pub listing_data: Account<'info, ListingData>,

    pub asset_manager: Account<'info, AssetManager>,

    pub core_program: Program<'info, Core>,

    pub system_program: Program<'info, System>,
}

impl UnListAsset<'_> {
    /// validation helper for our IX
    pub fn validate(&self, ctx: &Context<Self>) -> Result<()> {
        let asset_manager = Pubkey::create_program_address(
            &[
                SEED_PREFIX,
                SEED_ASSET_MANAGER,
                &[ctx.accounts.asset_manager.bump],
            ],
            &ctx.program_id,
        )
        .unwrap();

        assert_eq!(asset_manager, ctx.accounts.asset_manager.key());

        return Ok(());
    }

    /// un-list MPL core asset on the marketplace
    ///
    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn unlist(ctx: Context<UnListAsset>) -> Result<()> {
        let asset_manager = &ctx.accounts.asset_manager;
        // asset manager signer seeds
        let bump = &[asset_manager.bump];
        let signer_seeds = &[&[SEED_PREFIX, SEED_ASSET_MANAGER, bump][..]];

        // transfer asset back to owner
        TransferV1CpiBuilder::new(&ctx.accounts.core_program)
            .asset(&ctx.accounts.asset)
            .collection(ctx.accounts.collection.as_ref())
            .payer(&ctx.accounts.payer)
            .authority(Some(&asset_manager.to_account_info()))
            .new_owner(&ctx.accounts.payer.to_account_info())
            .system_program(Some(&ctx.accounts.system_program))
            .invoke_signed(signer_seeds)?;

        Ok(())
    }
}
