use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use soundwork_create::Core;
use soundwork_list::{
    cpi::{accounts::BuyAsset, buy_asset},
    program::SoundworkList,
    state::ListingData,
    AssetManager, BuyAssetParams, MarketPlaceConfig, PaymentOption,
};

use crate::{error::BidErrorCode, BidData};

/// Accept a placed Bid

/// Expects the following accounts:
/// 1. `[writeable, signer]` seller
/// 2. `[writeable]` bidder
/// 3. `[writeable]` asset
/// 4. `[writeable]` bid data account
/// 5. `[writeable]` bidder escrow wallet
/// 6. `[writeable]` listing data account
/// 7. `[writeable, optional]` bidder token account
/// 8. `[writeable, optional]` seller token account
/// 9. `[writeable, optional]` wallet token account
/// 10. `[writeable, optional]` treasury token account
/// 11. `[writeable]` treasury
/// 12. `[writeable,]` asset manager
/// 13. `[writeable,]` marketplace config account
/// 14. `[writeable]` payment mint  
/// 15. `[]` soundwork list program
/// 16. `[]` core program   
/// 17. `[]` token program   
/// 18. `[]` associated token program  
/// 19. `[]` system program  
///
#[derive(Accounts)]
pub struct AcceptBid<'info> {
    #[account(
        mut,
        address = listing_data.authority @ BidErrorCode::UnrecognizedSigner
    )]
    pub seller: Signer<'info>,

    #[account(
        mut,
        address = bid_data.authority @ BidErrorCode::UnrecognizedSigner
    )]
    pub bidder: SystemAccount<'info>,

    /// CHECK: checked by us
    #[account(mut)]
    pub asset: AccountInfo<'info>,

    #[account(
        mut,
        close = bidder,
    )]
    pub bid_data: Account<'info, BidData>,

    /// CHECK: initialized by list program through the CPI
    #[account(
        mut,
        owner = soundwork_list.key()
    )]
    pub bidder_escrow_wallet: UncheckedAccount<'info>,

    #[account(mut)]
    pub listing_data: Account<'info, ListingData>,

    #[account(mut)]
    pub payment_mint: Option<Box<Account<'info, Mint>>>,

    #[account(mut)]
    pub bidder_token_account: Option<Box<Account<'info, TokenAccount>>>,

    // unchecked because this might be uninitialized
    #[account(mut)]
    pub seller_token_account: Option<UncheckedAccount<'info>>,

    // unchecked because this might be uninitialized
    #[account(mut)]
    pub wallet_token_account: Option<Box<Account<'info, TokenAccount>>>,

    // unchecked because this might be uninitialized
    #[account(mut)]
    pub treasury_token_account: Option<UncheckedAccount<'info>>,

    #[account(
        mut,
        address = marketplace_config.treasury_address
    )]
    pub treasury: SystemAccount<'info>,

    pub asset_manager: Box<Account<'info, AssetManager>>,

    pub marketplace_config: Box<Account<'info, MarketPlaceConfig>>,

    pub soundwork_list: Program<'info, SoundworkList>,

    pub core_program: Program<'info, Core>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

impl AcceptBid<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        match self.listing_data.payment_option {
            PaymentOption::Native => {} // default anchor account checks

            PaymentOption::Token { mint: _ } => {
                let payment_mint = self.payment_mint.as_ref();
                let bidder_token_account = self.bidder_token_account.as_ref();
                let wallet_token_account = self.wallet_token_account.as_ref();
                let treasury_token_account = self.treasury_token_account.as_ref();
                let seller_token_account = self.seller_token_account.as_ref();

                let bidder_escrow_wallet = self.bidder_escrow_wallet.as_ref();

                if bidder_token_account.is_none()
                    || payment_mint.is_none()
                    || wallet_token_account.is_none()
                    || seller_token_account.is_none()
                    || treasury_token_account.is_none()
                {
                    return Err(error!(BidErrorCode::MissingAccount));
                }
            } //
        }

        return Ok(());
    }

    /// Accept placed bid for an MPL core asset on the marketplace
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn accept_bid(ctx: Context<AcceptBid>) -> Result<()> {
        let bid_data = &mut ctx.accounts.bid_data;
        let list_data = &mut ctx.accounts.listing_data;
        let cpi_program = ctx.accounts.soundwork_list.to_account_info();

        // transfer sol or tokens back to bidders wallet

        match list_data.payment_option {
            PaymentOption::Native => {
                let buy_asset_cpi_accounts = BuyAsset {
                    payer: ctx.accounts.seller.to_account_info(),
                    buyer: ctx.accounts.bidder.to_account_info(),
                    seller: ctx.accounts.seller.to_account_info(),
                    wallet_as_buyer: ctx.accounts.bidder_escrow_wallet.to_account_info().into(),
                    asset: ctx.accounts.asset.to_account_info(),
                    payment_mint: None,         // safe to unwrap
                    wallet_token_account: None, // safe to unwrap
                    buyer_token_account: None,
                    seller_token_account: None,
                    treasury_token_account: None,
                    treasury: ctx.accounts.treasury.to_account_info(),
                    listing_data: ctx.accounts.listing_data.to_account_info(),
                    asset_manager: ctx.accounts.asset_manager.to_account_info(),
                    marketplace_config: ctx.accounts.marketplace_config.to_account_info(),
                    core_program: ctx.accounts.core_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    associated_token_program: ctx
                        .accounts
                        .associated_token_program
                        .to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                };

                let buy_asset_ctx = CpiContext::new(cpi_program, buy_asset_cpi_accounts);

                buy_asset(buy_asset_ctx, None)?;
            }

            PaymentOption::Token { mint: _ } => {
                let buy_listing_cpi_accounts = BuyAsset {
                    payer: ctx.accounts.seller.to_account_info(),
                    buyer: ctx.accounts.bidder.to_account_info(),
                    seller: ctx.accounts.seller.to_account_info(),
                    wallet_as_buyer: ctx.accounts.bidder_escrow_wallet.to_account_info().into(),
                    asset: ctx.accounts.asset.to_account_info(),
                    payment_mint: Some(
                        ctx.accounts.payment_mint.clone().unwrap().to_account_info(),
                    ), // safe to unwrap
                    wallet_token_account: Some(
                        ctx.accounts
                            .wallet_token_account
                            .clone()
                            .unwrap()
                            .to_account_info(),
                    ), // safe to unwrap
                    buyer_token_account: Some(
                        ctx.accounts
                            .bidder_token_account
                            .clone()
                            .unwrap()
                            .to_account_info(),
                    ),
                    seller_token_account: Some(
                        ctx.accounts
                            .seller_token_account
                            .clone()
                            .unwrap()
                            .to_account_info(),
                    ),
                    treasury_token_account: Some(
                        ctx.accounts
                            .treasury_token_account
                            .clone()
                            .unwrap()
                            .to_account_info(),
                    ),
                    treasury: ctx.accounts.treasury.to_account_info(),
                    listing_data: ctx.accounts.listing_data.to_account_info(),
                    asset_manager: ctx.accounts.asset_manager.to_account_info(),
                    marketplace_config: ctx.accounts.marketplace_config.to_account_info(),
                    core_program: ctx.accounts.core_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    associated_token_program: ctx
                        .accounts
                        .associated_token_program
                        .to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                };

                let buy_asset_ctx = CpiContext::new(cpi_program, buy_listing_cpi_accounts);

                buy_asset(
                    buy_asset_ctx,
                    BuyAssetParams {
                        bid_amount: bid_data.amount,
                    }
                    .into(),
                )?;
            }
        }

        Ok(())
    }
}
