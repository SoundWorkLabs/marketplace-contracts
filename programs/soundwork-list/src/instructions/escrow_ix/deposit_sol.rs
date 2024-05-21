use anchor_lang::{
    prelude::*,
    system_program::{self, System, Transfer},
};

use crate::Wallet;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositSolParams {
    ///  amount to deposit in lamports
    pub amount: u64,
}

/// Deposit Sol into escrow wallet managed by the list program
///
/// Expects the following accounts:
/// 1. `[writeable, signer]` authority
/// 2. `[writeable]` wallet
/// 3. `[]` system program
///
/// Expects the following arguments
/// 1. params: DepositSolParams
///
#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub wallet: Account<'info, Wallet>,

    pub system_program: Program<'info, System>,
}

impl DepositSol<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// Deposit native sol into user's wallet
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn deposit_sol(ctx: Context<DepositSol>, params: DepositSolParams) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.authority.to_account_info(),
            to: ctx.accounts.wallet.to_account_info(),
        };
        let cpi_program = ctx.accounts.system_program.to_account_info();

        let cpi_context: CpiContext<'_, '_, '_, '_, Transfer<'_>> =
            CpiContext::new(cpi_program, cpi_accounts);

        system_program::transfer(cpi_context, params.amount)?;

        Ok(())
    }
}
