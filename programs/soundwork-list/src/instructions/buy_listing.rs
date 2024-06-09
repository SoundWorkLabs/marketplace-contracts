use anchor_lang::{
    prelude::*,
    system_program::{self, Transfer as SOLTransfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked as SPLTransferChecked},
};
use mpl_core::instructions::TransferV1CpiBuilder;

use crate::{
    constants::{SEED_ASSET_MANAGER, SEED_PREFIX},
    error::ListErrorCode,
    helpers::{calculate_total_buy_fee, Core},
    AssetManager, ListingData, MarketPlaceConfig, PaymentOption, Wallet, SEED_WALLET,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct BuyAssetParams {
    ///  only used when buying using a bid amount
    pub bid_amount: u64,
}

/// Buy a listed MPL core asset on soundwork
///
///  ### Accounts:
///
/// 1. `[writeable, signer]` payer
/// 2. `[writeable]` buyer
/// 3. `[writeable]` seller
/// 4. `[writeable, optional]` wallet as buyer
/// 5. `[writeable]` asset
/// 6. `[writeable, optional]` payment mint
/// 7. `[writeable, optional]` wallet token account
/// 8. `[writeable, optional]` buyer token account
/// 9. `[writeable, optional]` seller token account
/// 10. `[writeable, optional]` treasury token account
/// 11. `[writeable]` treasury
/// 12. `[writeable]` listing data account
/// 13. `[]` asset manager
/// 14. `[]` marketplace config
/// 15. `[]` core program
/// 16. `[]` token program
/// 17. `[]` associated token program
/// 18. `[]` system program
///
/// ### Parameters
///
/// 1. params: [BuyAssetParams]
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
    pub wallet_as_buyer: Option<Box<Account<'info, Wallet>>>,

    /// CHECK: checked by us
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,

    #[account(mut)]
    pub payment_mint: Option<Box<Account<'info, Mint>>>,

    // we expect this to be initialized because this is used only for bids
    // a check before placing a bid, makes sure bidder has enough funds to make bid
    #[account(mut)]
    pub wallet_token_account: Option<Box<Account<'info, TokenAccount>>>,

    #[account(mut)]
    pub buyer_token_account: Option<Box<Account<'info, TokenAccount>>>,

    // maybe we offered seller a list of tokens to chose from,
    // hence seller token account might not be initialized
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = payment_mint,
        associated_token::authority = seller,
    )]
    pub seller_token_account: Option<Box<Account<'info, TokenAccount>>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = payment_mint,
        associated_token::authority = treasury,
    )]
    pub treasury_token_account: Option<Account<'info, TokenAccount>>,

    #[account(
        mut,
        address = marketplace_config.treasury_address
    )]
    pub treasury: SystemAccount<'info>,

    #[account(mut, close = seller)]
    pub listing_data: Box<Account<'info, ListingData>>,

    pub asset_manager: Box<Account<'info, AssetManager>>,

    pub marketplace_config: Box<Account<'info, MarketPlaceConfig>>,

    pub core_program: Program<'info, Core>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

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
    pub fn buy_asset(ctx: Context<BuyAsset>, params: Option<BuyAssetParams>) -> Result<()> {
        let listing_data = &mut ctx.accounts.listing_data;
        let asset_manager = &ctx.accounts.asset_manager;
        let escrow_wallet = &mut ctx.accounts.wallet_as_buyer;

        // using the optional escrow_wallet_as buyer account to check who will be paying for the NFT
        // if escrow wallet is provided, use to pay for the asset, else use payer
        match (escrow_wallet, params) {
            // only use escrow wallet to buy asset if both the account and bid_amount is provided
            (Some(wallet), Some(params)) => {
                let taker_fee_bps = ctx.accounts.marketplace_config.taker_fee_bps;
                let total_cost = calculate_total_buy_fee(params.bid_amount, taker_fee_bps).unwrap();
                let protocol_take = total_cost.checked_sub(params.bid_amount).unwrap(); // if fee calculation OK, assume safe to unwrap here

                // if use wanted to pay with tokens
                match listing_data.payment_option {
                    PaymentOption::Native => {
                        if wallet.get_lamports() < total_cost {
                            return Err(error!(ListErrorCode::InsufficientFunds));
                        }

                        // transfer to seller
                        wallet.sub_lamports(params.bid_amount)?;
                        ctx.accounts.seller.add_lamports(params.bid_amount)?;

                        // transfer to protocol treasury
                        wallet.sub_lamports(protocol_take)?;
                        ctx.accounts.treasury.add_lamports(protocol_take)?;
                    }
                    PaymentOption::Token { mint } => {
                        let payment_mint = &ctx.accounts.payment_mint.as_ref();
                        let wallet_token_account = &ctx.accounts.wallet_token_account.as_ref();
                        let seller_token_account = &ctx.accounts.seller_token_account.as_ref();
                        let treasury_token_account = &ctx.accounts.treasury_token_account.as_ref();

                        // allow us to use unwrap() safely
                        if payment_mint.is_none()
                            || wallet_token_account.is_none()
                            || seller_token_account.is_none()
                            || treasury_token_account.is_none()
                        {
                            return Err(error!(ListErrorCode::MissingAccount));
                        }

                        if payment_mint.unwrap().key() != mint {
                            return Err(error!(ListErrorCode::PaymentMintAddressMismatch));
                        }

                        if wallet_token_account.unwrap().amount < total_cost {
                            return Err(error!(ListErrorCode::InsufficientFunds));
                        }

                        // signer seeds
                        let bump = &[wallet.bump];
                        let signer_seeds = &[&[
                            SEED_PREFIX,
                            SEED_WALLET,
                            ctx.accounts.buyer.key.as_ref(), // owns escrow wallet on soundwork
                            bump,
                        ][..]];

                        let cpi_program = ctx.accounts.token_program.to_account_info();

                        let seller_cpi_accounts = SPLTransferChecked {
                            from: wallet_token_account.unwrap().to_account_info(),
                            mint: payment_mint.unwrap().to_account_info(),
                            to: seller_token_account.unwrap().to_account_info(),
                            authority: wallet.to_account_info(),
                        };

                        let seller_cpi_ctx = CpiContext::new_with_signer(
                            cpi_program.clone(),
                            seller_cpi_accounts,
                            signer_seeds,
                        );

                        transfer_checked(
                            seller_cpi_ctx,
                            params.bid_amount,
                            payment_mint.unwrap().decimals,
                        )?;

                        // transfer to protocol
                        let protocol_cpi_accounts = SPLTransferChecked {
                            from: wallet_token_account.unwrap().to_account_info(),
                            mint: payment_mint.unwrap().to_account_info(),
                            to: treasury_token_account.unwrap().to_account_info(),
                            authority: wallet.to_account_info(),
                        };

                        let protocol_cpi_ctx = CpiContext::new_with_signer(
                            cpi_program.clone(),
                            protocol_cpi_accounts,
                            signer_seeds,
                        );

                        transfer_checked(
                            protocol_cpi_ctx,
                            protocol_take,
                            payment_mint.unwrap().decimals,
                        )?;
                    }
                };
            }

            // use buyers account to buy asset
            (None, None) => {
                // price calc
                let taker_fee_bps = ctx.accounts.marketplace_config.taker_fee_bps;
                let total_cost =
                    calculate_total_buy_fee(listing_data.amount, taker_fee_bps).unwrap();
                let protocol_take = total_cost.checked_sub(listing_data.amount).unwrap(); // if fee calculation OK, assume safe to unwrap here

                match listing_data.payment_option {
                    PaymentOption::Native => {
                        // accounts
                        let buyer = ctx.accounts.buyer.as_ref();
                        let seller = ctx.accounts.seller.as_ref();

                        if buyer.get_lamports() < total_cost {
                            return Err(error!(ListErrorCode::InsufficientFunds));
                        }

                        // transfers
                        let cpi_program = ctx.accounts.system_program.to_account_info();

                        // transfer to seller
                        let seller_cpi_accounts = SOLTransfer {
                            from: buyer.to_account_info(),
                            to: seller.to_account_info(),
                        };

                        let seller_cpi_context =
                            CpiContext::new(cpi_program.clone(), seller_cpi_accounts);

                        system_program::transfer(seller_cpi_context, listing_data.amount)?;

                        // protocol take
                        let protocol_cpi_accounts = SOLTransfer {
                            from: buyer.to_account_info(),
                            to: seller.to_account_info(),
                        };

                        let protocol_cpi_context =
                            CpiContext::new(cpi_program.clone(), protocol_cpi_accounts);

                        system_program::transfer(protocol_cpi_context, protocol_take)?;
                    }
                    PaymentOption::Token { mint } => {
                        // accounts
                        let payment_mint = &ctx.accounts.payment_mint.as_ref();
                        let buyer = ctx.accounts.buyer.as_ref();
                        let buyer_token_account = ctx.accounts.buyer_token_account.as_ref();
                        let seller_token_account = ctx.accounts.seller_token_account.as_ref();
                        let treasury_token_account = ctx.accounts.treasury_token_account.as_ref();

                        // check if optional accounts exist so that we can safely `unwrap()`
                        if payment_mint.is_none()
                            || buyer_token_account.is_none()
                            || seller_token_account.is_none()
                            || treasury_token_account.is_none()
                        {
                            return Err(error!(ListErrorCode::MissingAccount));
                        }

                        // check payment mint
                        if payment_mint.unwrap().key() != mint {
                            return Err(error!(ListErrorCode::PaymentMintAddressMismatch));
                        }

                        // check buyer amount
                        if buyer_token_account.unwrap().amount < total_cost {
                            return Err(error!(ListErrorCode::InsufficientFunds));
                        }

                        // transfers
                        let cpi_program = ctx.accounts.token_program.to_account_info();

                        // seller
                        let seller_cpi_accounts = SPLTransferChecked {
                            from: buyer_token_account.unwrap().to_account_info(),
                            mint: payment_mint.unwrap().to_account_info(),
                            to: seller_token_account.unwrap().to_account_info(),
                            authority: buyer.to_account_info(),
                        };

                        let seller_cpi_ctx =
                            CpiContext::new(cpi_program.clone(), seller_cpi_accounts);

                        transfer_checked(
                            seller_cpi_ctx,
                            listing_data.amount,
                            payment_mint.unwrap().decimals,
                        )?;

                        // protocol take
                        let buyer_cpi_accounts = SPLTransferChecked {
                            from: buyer_token_account.unwrap().to_account_info(),
                            mint: payment_mint.unwrap().to_account_info(),
                            to: treasury_token_account.unwrap().to_account_info(),
                            authority: buyer.to_account_info(),
                        };

                        let seller_cpi_ctx =
                            CpiContext::new(cpi_program.clone(), buyer_cpi_accounts);

                        transfer_checked(
                            seller_cpi_ctx,
                            protocol_take,
                            payment_mint.unwrap().decimals,
                        )?;
                    }
                }
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
