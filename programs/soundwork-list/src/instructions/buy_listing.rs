use anchor_lang::prelude::*;
use mpl_core::instructions::TransferV1CpiBuilder;

use crate::{
    calculate_total_buy_fee,
    constants::{SEED_ASSET_MANAGER, SEED_LISTING_DATA, SEED_PREFIX},
    error::ListErrorCode,
    AssetManager, ListingData, MarketPlaceConfig, PaymentOption, Wallet,
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
pub struct BuyAssetParams {
    ///  only used when buying using a bid amount
    pub bid_amount: u64,
}

/// Buy a listed MPL core asset on soundwork
///
/// Expects the following accounts:
/// 1. `[writeable, signer]` payer
/// . `[writeable]` buyer
/// . `[writeable]` seller
/// . `[writeable]` asset
/// . `[writeable]` listing data account
/// . `[]` marketplace config
/// . `[]` asset manager
/// . `[]` core program
/// . `[]` system program
///
/// Expects the following arguments
/// 1. params: ListTokenParams
///
#[derive(Accounts)]
pub struct BuyAsset<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub buyer: SystemAccount<'info>,

    #[account(mut)]
    pub seller: SystemAccount<'info>,

    #[account(mut)]
    pub escrow_wallet_as_buyer: Option<Account<'info, Wallet>>,

    /// CHECK: checked by us
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,

    #[account(mut, close = seller)]
    pub listing_data: Account<'info, ListingData>,

    pub marketplace_config: Account<'info, MarketPlaceConfig>,

    pub treasury: SystemAccount<'info>,

    pub asset_manager: Account<'info, AssetManager>,

    pub core_program: Program<'info, Core>,

    pub system_program: Program<'info, System>,
}

impl BuyAsset<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// buy a MPL core asset listed on the marketplace
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn buy_asset(&self, ctx: Context<BuyAsset>, params: Option<BuyAssetParams>) -> Result<()> {
        let listing_data = &mut ctx.accounts.listing_data;
        let asset_manager = &ctx.accounts.asset_manager;
        let escrow_wallet = &mut ctx.accounts.escrow_wallet_as_buyer;

        // using the optional escrow_wallet_as buyer account to check who will be paying for the NFT
        // if escrow wallet is provided, use to pay for the asset, else use payer
        match (escrow_wallet, params) {
            // only use escrow wallet to buy asset if both the account and bid_amount is provided
            (Some(wallet), Some(params)) => {
                let taker_fee_bps = ctx.accounts.marketplace_config.taker_fee_bps;
                let total_cost = calculate_total_buy_fee(params.bid_amount, taker_fee_bps).unwrap();
                let protocol_take = total_cost.checked_sub(params.bid_amount).unwrap(); // if fee calculation OK, assume safe to unwrap here

                if wallet.get_lamports() < total_cost {
                    return Err(error!(ListErrorCode::InsufficientFunds));
                }

                // if use wanted to pay with tokens
                match listing_data.payment_option {
                    PaymentOption::Native => {
                        // transfer to user
                        wallet.sub_lamports(params.bid_amount)?;
                        ctx.accounts.seller.add_lamports(params.bid_amount)?;

                        // transfer to protocol treasury
                        wallet.sub_lamports(protocol_take)?;
                        ctx.accounts.treasury.add_lamports(protocol_take)?;
                    }
                    PaymentOption::Token { mint } => todo!(),
                };
            }

            // use buyers account to buy asset
            (None, None) => {
                //
            }

            // invalid, panic
            (None, Some(_)) => todo!("Create suitable error message for this"),

            // invalid, panic
            (Some(_), None) => todo!("Create suitable error message for this"),
        };

        // transfer to buyer
        let bump = &[asset_manager.bump];
        let signer_seeds = &[&[SEED_PREFIX, SEED_ASSET_MANAGER, bump][..]];

        TransferV1CpiBuilder::new(&ctx.accounts.core_program)
            .asset(&ctx.accounts.asset)
            .payer(&ctx.accounts.payer)
            .authority(Some(&asset_manager.to_account_info()))
            .new_owner(&ctx.accounts.payer.to_account_info())
            .system_program(Some(&ctx.accounts.system_program))
            .invoke_signed(signer_seeds)?;

        Ok(())
    }
}
