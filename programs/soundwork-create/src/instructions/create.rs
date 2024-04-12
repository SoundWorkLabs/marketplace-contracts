use anchor_lang::prelude::*;
use mpl_core::instructions::CreateV1CpiBuilder;

// todo: remove and use SPL typed account
#[derive(Clone)]
pub struct Core;

impl anchor_lang::Id for Core {
    fn id() -> Pubkey {
        mpl_core::ID
    }
}

/// Create MPL Core Asset context
///
/// Expects the following accounts:
/// 1. `[writeable, signer]` payer
/// 2. `[writeable, signer]` asset
/// 3. `[]` core program
/// 4. `[]` `system program`
///
/// Expects the following arguments  
/// 1. name: string
/// 2. uri: string
#[derive(Accounts)]
pub struct CreateAsset<'info> {
    pub payer: Signer<'info>,

    /// CHECK: we are passing this in ourselves
    #[account(mut, signer)]
    pub asset: UncheckedAccount<'info>,

    pub core_program: Program<'info, Core>,

    pub system_program: Program<'info, System>,
}

impl CreateAsset<'_> {
    /// validation helper for our IX
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// CPI into mpl_core program and mint our asset.
    ///
    /// *  name - the title of the asset being minted
    /// *  uri – off-chain URI of the metadata
    ///
    /// Treasury is given via instruction accounts.
    #[access_control(ctx.accounts.validate())]
    pub fn create_asset(ctx: Context<CreateAsset>, name: String, uri: String) -> Result<()> {
        CreateV1CpiBuilder::new(&ctx.accounts.core_program)
            .asset(&ctx.accounts.asset)
            .collection(None)
            .authority(Some(&ctx.accounts.payer))
            .payer(&ctx.accounts.payer)
            .owner(Some(&ctx.accounts.payer))
            .update_authority(Some(&ctx.accounts.payer))
            .system_program(&ctx.accounts.system_program)
            .name(name)
            .uri(uri)
            .invoke()?;

        Ok(())
    }
}
