use anchor_lang::prelude::*;

use crate::{AssetManager, constants::{SEED_ASSET_MANAGER, SEED_PREFIX, ADMIN_ADDRESS}, helpers::Core};

/// Initialize AssetManager escrow account
///
/// Expects the following accounts:
/// 1. `[writeable, signer]` payer
/// 2. `[writeable, signer]` assetManager
/// 3. `[]` core program
/// 4. `[]` `system program`

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    // todo: move this admin address check to the validate functions 
    // find out a way to do this using a slice or vector of verified addresses
    #[account(mut, address = ADMIN_ADDRESS)]
    pub payer: Signer<'info>,

    #[account(
        init, 
        payer=payer,
        space=AssetManager::LEN,
        seeds = [SEED_PREFIX, SEED_ASSET_MANAGER],
        bump
    )]
    pub asset_manager: Account<'info, AssetManager>,

    pub core_program: Program<'info, Core>,

    pub system_program: Program<'info, System>,
}

impl InitEscrow<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// Initialize the Asset Manager escrow account
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn init_escrow(ctx: Context<InitEscrow>) -> Result<()> {
        msg!("initialized escrow account");

        let asset_manager = &mut ctx.accounts.asset_manager;
        asset_manager.bump = ctx.bumps.asset_manager; // ? is this safe

        Ok(())
    }
}
