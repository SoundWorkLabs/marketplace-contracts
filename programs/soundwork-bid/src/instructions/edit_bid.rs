use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use soundwork_list::{
    cpi::{
        accounts::{DepositSol, DepositToken},
        deposit_sol, deposit_token,
    },
    program::SoundworkList,
    state::{ListingData, Wallet},
    DepositSolParams, DepositTokenParams, PaymentOption,
};

use crate::{error::BidErrorCode, BidData};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct EditBidParams {
    ///  bid amount/price in lamports
    pub amount: Option<u64>,

    ///  expiry timestamp
    pub expiry_ts: Option<i64>,
}

/// Edit Bid
///
/// Expects the following accounts:
/// 1. `[writeable, signer]` bidder
/// 2. `[writeable]` asset
/// 3. `[writeable]` bid data account
/// 4. `[writeable]` bidder escrow wallet
/// 5. `[writeable]` listing data account
/// 6. `[writeable]` payment mint
/// 7. `[writeable, optional]` bidder token account
/// 8. `[writeable, options]` wallet token account
/// 9. `[]` soundwork list program
/// 10. `[]` token program
/// 11. `[]` associated token program
/// 12. `[]` system program
///
/// Expects the following arguments
/// 1. params: EditBidParams
///
#[derive(Accounts)]
#[instruction(params: EditBidParams)]
pub struct EditBid<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,

    /// CHECK: checked by us
    #[account(mut)]
    pub asset: AccountInfo<'info>,

    /// CHECK: checked by us
    #[account(mut)]
    pub bid_data: Account<'info, BidData>,

    #[account(mut)]
    pub bidder_escrow_wallet: Box<Account<'info, Wallet>>,

    #[account(mut)]
    pub listing_data: Account<'info, ListingData>,

    #[account(mut)]
    pub payment_mint: Option<Box<Account<'info, Mint>>>,

    #[account(mut)]
    pub bidder_token_account: Option<Box<Account<'info, TokenAccount>>>,

    #[account(mut)]
    pub wallet_token_account: Option<Box<Account<'info, TokenAccount>>>,

    pub soundwork_list: Program<'info, SoundworkList>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

impl EditBid<'_> {
    /// validation helper for our IX
    pub fn validate(&self, params: &EditBidParams) -> Result<()> {
        // check if bidder has enough amount

        if let Some(amt) = params.amount {
            let diff = amt.checked_sub(self.bid_data.amount);
            if diff.is_none() {
                return Err(error!(BidErrorCode::Overflow));
            }

            match self.listing_data.payment_option {
                PaymentOption::Native => {
                    if self.bidder.get_lamports() < diff.unwrap() {
                        msg!("insufficient lamports");
                        return Err(error!(BidErrorCode::InsufficientFunds));
                    }
                }

                PaymentOption::Token { mint } => {
                    let payment_mint = self.payment_mint.as_ref();
                    let bidder_token_account = self.bidder_token_account.as_ref();

                    if bidder_token_account.is_none() || payment_mint.is_none() {
                        return Err(error!(BidErrorCode::MissingAccount));
                    }

                    if mint != payment_mint.unwrap().key() {
                        return Err(error!(BidErrorCode::PaymentMintAddressMismatch));
                    }

                    if bidder_token_account.unwrap().amount < diff.unwrap() {
                        msg!("insufficient tokens");
                        return Err(error!(BidErrorCode::InsufficientFunds));
                    }
                } //
            }
        }

        // todo: sanity checks for expiry timestamp

        return Ok(());
    }

    /// Edit MPL core asset on the marketplace
    ///
    #[access_control(ctx.accounts.validate(&params))]
    pub fn edit_bid(ctx: Context<EditBid>, params: EditBidParams) -> Result<()> {
        let bid_data = &mut ctx.accounts.bid_data;
        let list_data = &mut ctx.accounts.listing_data;
        let cpi_program = ctx.accounts.soundwork_list.to_account_info();

        let EditBidParams { amount, expiry_ts } = params;

        // update expiry
        if let Some(expire_ts) = expiry_ts {
            bid_data.expiry_ts = expire_ts;
        }

        // check if bidder increased his bid amt and if true, transfer the
        // difference with new price to escrow wallet to be able to cover buying
        // nft if seller accepts offer
        if let Some(amt) = amount {
            if amt > bid_data.amount {
                let diff = amt.checked_sub(bid_data.amount);
                if diff.is_none() {
                    return Err(error!(BidErrorCode::Overflow));
                }

                match list_data.payment_option {
                    PaymentOption::Native => {
                        let deposit_sol_cpi_accounts = DepositSol {
                            authority: ctx.accounts.bidder.to_account_info(),
                            wallet: ctx.accounts.bidder_escrow_wallet.to_account_info(),
                            system_program: ctx.accounts.system_program.to_account_info(),
                        };

                        let deposit_sol_ctx =
                            CpiContext::new(cpi_program, deposit_sol_cpi_accounts);

                        deposit_sol(
                            deposit_sol_ctx,
                            DepositSolParams {
                                amount: diff.unwrap(),
                            },
                        )?;
                    }

                    PaymentOption::Token { mint: _ } => {
                        let diff = amt.checked_sub(bid_data.amount);
                        if diff.is_none() {
                            return Err(error!(BidErrorCode::Overflow));
                        }

                        let deposit_token_cpi_accounts = DepositToken {
                            authority: ctx.accounts.bidder.to_account_info(),
                            wallet: ctx.accounts.bidder_escrow_wallet.to_account_info(),
                            mint: ctx
                                .accounts
                                .payment_mint
                                .as_ref()
                                .unwrap()
                                .to_account_info(), // safe to unwrap. checked in validator constraint
                            authority_token_account: ctx
                                .accounts
                                .bidder_token_account
                                .as_ref()
                                .unwrap()
                                .to_account_info(), // safe to unwrap here too
                            wallet_token_account: ctx
                                .accounts
                                .wallet_token_account
                                .as_ref()
                                .unwrap()
                                .to_account_info(),
                            token_program: ctx.accounts.token_program.to_account_info(),
                            associated_token_program: ctx
                                .accounts
                                .associated_token_program
                                .to_account_info(),
                            system_program: ctx.accounts.system_program.to_account_info(),
                        };

                        let deposit_token_ctx =
                            CpiContext::new(cpi_program, deposit_token_cpi_accounts);

                        deposit_token(
                            deposit_token_ctx,
                            DepositTokenParams {
                                amount: diff.unwrap(),
                            },
                        )?;
                    }
                }

                bid_data.amount += diff.unwrap(); // ! test me
            }

            // do nothing. escrow wallet has enough funds to purchase asset if accepted
            msg!("escrow wallet balance remains unchanged");
        }

        Ok(())
    }
}
