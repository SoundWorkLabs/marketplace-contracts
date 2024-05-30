use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use soundwork_list::{
    cpi::{
        accounts::{WithdrawSol, WithdrawToken},
        withdraw_sol, withdraw_token,
    },
    program::SoundworkList,
    state::ListingData,
    PaymentOption, WithdrawSolParams, WithdrawTokenParams,
};

use crate::{error::BidErrorCode, BidData};

/// Revoke placed Bid
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

#[derive(Accounts)]
pub struct RevokeBid<'info> {
    #[account(
        mut,
        address = bid_data.authority @ BidErrorCode::UnrecognizedSigner
    )]
    pub bidder: Signer<'info>,

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

    #[account(mut)]
    pub wallet_token_account: Option<Box<Account<'info, TokenAccount>>>,

    pub soundwork_list: Program<'info, SoundworkList>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

impl RevokeBid<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        match self.listing_data.payment_option {
            PaymentOption::Native => {}

            PaymentOption::Token { mint: _ } => {
                let payment_mint = self.payment_mint.as_ref();
                let bidder_token_account = self.bidder_token_account.as_ref();

                if bidder_token_account.is_none() || payment_mint.is_none() {
                    return Err(error!(BidErrorCode::MissingAccount));
                }
            } //
        }

        return Ok(());
    }

    /// revoke previously placed bid for an MPL core asset on the marketplace
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn revoke_bid(ctx: Context<RevokeBid>) -> Result<()> {
        let bid_data = &mut ctx.accounts.bid_data;
        let list_data = &mut ctx.accounts.listing_data;
        let cpi_program = ctx.accounts.soundwork_list.to_account_info();

        // transfer sol or tokens back to bidders wallet

        match list_data.payment_option {
            PaymentOption::Native => {
                let withdraw_sol_cpi_accounts = WithdrawSol {
                    payer: ctx.accounts.bidder.to_account_info(),
                    authority: ctx.accounts.bidder.to_account_info(),
                    wallet: ctx.accounts.bidder_escrow_wallet.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                };

                let withdraw_sol_ctx = CpiContext::new(cpi_program, withdraw_sol_cpi_accounts);

                withdraw_sol(
                    withdraw_sol_ctx,
                    WithdrawSolParams {
                        amount: bid_data.amount,
                    }
                    .into(),
                )?;
            }

            PaymentOption::Token { mint: _ } => {
                let withdraw_token_cpi_accounts = WithdrawToken {
                    payer: ctx.accounts.bidder.to_account_info(),
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

                let withdraw_token_ctx = CpiContext::new(cpi_program, withdraw_token_cpi_accounts);

                withdraw_token(
                    withdraw_token_ctx,
                    WithdrawTokenParams {
                        amount: bid_data.amount,
                    }
                    .into(),
                )?;
            }
        }

        Ok(())
    }
}
