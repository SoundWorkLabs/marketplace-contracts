use anchor_lang::prelude::*;

use crate::{
    constants::{ADMIN_ADDRESS, SEED_MARKETPLACE_CONFIG, SEED_PREFIX},
    MarketPlaceConfig,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitMarketPlaceConfigParams {
    ///  taker fee basis points, /100%
    pub taker_fee_bps: u8,

    /// treasury address
    pub treasury_address: Pubkey,
}

/// Initialize protocol state accounts
///
/// Expects the following accounts:
/// 1. `[writeable, signer]` payer
/// 2. `[writeable, signer]` assetManager
/// 3. `[]` `system program`

#[derive(Accounts)]
#[instruction(params: InitMarketPlaceConfigParams)]
pub struct InitMarketplaceConfig<'info> {
    // todo: move this admin address check to the validate functions
    // find out a way to do this using a slice or vector of verified addresses
    #[account(mut, address = ADMIN_ADDRESS)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = MarketPlaceConfig::LEN,
        seeds = [SEED_PREFIX, SEED_MARKETPLACE_CONFIG],
        bump
    )]
    pub marketplace_config: Account<'info, MarketPlaceConfig>,

    pub system_program: Program<'info, System>,
}

impl InitMarketplaceConfig<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// Initialize the marketplace config account
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn init_marketplace_config(
        ctx: Context<InitMarketplaceConfig>,
        params: InitMarketPlaceConfigParams,
    ) -> Result<()> {
        msg!("initialized marketplace config account");

        let marketplace_config = &mut ctx.accounts.marketplace_config;
        **marketplace_config = MarketPlaceConfig::new(
            ctx.bumps.marketplace_config,
            params.treasury_address,
            params.taker_fee_bps,
        );

        Ok(())
    }
}
