use anchor_lang::{prelude::*, system_program::System};

use crate::{error::ListErrorCode, Wallet};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawSolParams {
    ///  amount to withdraw in lamports
    pub amount: u64,
}

///  Withdraw SOL from escrow wallet managed by the program.
///
/// Expects the following accounts:
/// 1. `[writeable, signer]` payer
/// 2. `[writeable]` authority
/// 3. `[writeable]` wallet
/// 4. `[]` system program
///
/// Expects the following arguments
/// 1. params: WithdrawSolParams
///
#[derive(Accounts)]
pub struct WithdrawSol<'info> {
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

    pub system_program: Program<'info, System>,
}

impl WithdrawSol<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// Deposit native sol into user's wallet
    ///
    /// todo(Jimii): withdraw to another wallet per user's request
    #[access_control(ctx.accounts.validate())]
    pub fn withdraw_sol(
        ctx: Context<WithdrawSol>,
        params: Option<WithdrawSolParams>,
    ) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        if let Some(WithdrawSolParams { amount }) = params {
            if amount > wallet.get_lamports() {
                return Err(error!(ListErrorCode::InsufficientFunds));
            }

            // transfer requested amount
            wallet.sub_lamports(amount)?;
            ctx.accounts.authority.add_lamports(amount)?;

            return Ok(());
        }

        // else close account and return everything to user
        wallet.close(ctx.accounts.authority.to_account_info())?;

        Ok(())
    }
}
