use anchor_lang::prelude::*;

use crate::{
    constants::{SEED_PREFIX, SEED_WALLET},
    Wallet,
};

/// Initialize user escrow wallet
///
/// ### Accounts:
///
/// 1. `[writeable, signer]` authority
/// 2. `[writeable]` wallet
/// 3. `[]` system program
///
#[derive(Accounts)]
pub struct InitWallet<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Wallet::LEN,
        seeds = [SEED_PREFIX, SEED_WALLET, authority.key().as_ref()],
        bump
    )]
    pub wallet: Account<'info, Wallet>,

    pub system_program: Program<'info, System>,
}

impl InitWallet<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// Initialize our escrow marketplace wallet
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn init_wallet(ctx: Context<InitWallet>) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        **wallet = Wallet::new(&ctx.accounts.authority.key(), ctx.bumps.wallet);

        Ok(())
    }
}
