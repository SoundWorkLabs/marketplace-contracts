use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};

use crate::Wallet;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositTokenParams {
    ///  amount to deposit in lamports
    pub amount: u64,
}

/// Deposit Tokens into escrow wallet managed by the list program
///
///  ### Accounts:
/// 1. `[writeable, signer]` authority
/// 2. `[writeable]` wallet
/// 3. `[writeable, optional]` mint
/// 4. `[writeable, optional]` authority associated token address
/// 5. `[writeable, optional]` wallet associated token address
/// 6. `[]` token program
/// 7. `[]` associated token program
/// 8. `[]` system program
///
/// ### Parameters
/// 1. params: [DepositTokenParams]
///
#[derive(Accounts)]
pub struct DepositToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub wallet: Account<'info, Wallet>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = wallet,
    )]
    pub wallet_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl DepositToken<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// Deposit native sol into user's wallet
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn deposit_token(ctx: Context<DepositToken>, params: DepositTokenParams) -> Result<()> {
        let DepositTokenParams { amount } = params;

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: ctx.accounts.authority_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.wallet_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, amount, ctx.accounts.mint.decimals)?;

        Ok(())
    }
}
