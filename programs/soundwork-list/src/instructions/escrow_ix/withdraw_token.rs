use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};

use crate::{
    constants::{SEED_PREFIX, SEED_WALLET},
    error::ListErrorCode,
    Wallet,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawTokenParams {
    ///  amount to withdraw in lamports
    pub amount: u64,
}

/// Withdraw Tokens from escrow wallet managed by the list program
///
/// Expects the following accounts:
/// 1. `[writeable, signer]` payer
/// 2. `[writeable]` authority
/// 3. `[writeable]` wallet
/// 4. `[writeable, optional]` mint
/// 5. `[writeable, optional]` authority associated token address
/// 6. `[writeable, optional]` wallet associated token address
/// 7. `[]` token program
/// 8. `[]` associated token program
/// 9. `[]` system program
///
/// Expects the following arguments
/// 1. params: WithdrawTokenParams
///
#[derive(Accounts)]
pub struct WithdrawToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: checked in the ix
    #[account(
        mut,
        address = wallet.authority @ ListErrorCode::InvalidAuthority
    )]
    pub authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub wallet: Account<'info, Wallet>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub wallet_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl WithdrawToken<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// Deposit native sol into user's wallet
    ///
    /// todo(Jimii): withdraw to another wallet per user's request
    #[access_control(ctx.accounts.validate())]
    pub fn withdraw_token(ctx: Context<WithdrawToken>, params: WithdrawTokenParams) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;
        let authority = &mut ctx.accounts.authority;
        let WithdrawTokenParams { amount } = params;

        // sanity check
        if amount == 0 {
            return Err(error!(ListErrorCode::ZeroValueNotAllowed));
        }

        let bump = &[wallet.bump];
        let signer_seeds = &[&[SEED_PREFIX, SEED_WALLET, authority.key.as_ref(), bump][..]];

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: ctx.accounts.wallet_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.authority_token_account.to_account_info(),
            authority: wallet.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, amount, ctx.accounts.mint.decimals)?;

        Ok(())
    }
}
