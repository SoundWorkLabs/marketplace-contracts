pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

#[allow(unused_imports)]
use solana_security_txt::security_txt;

pub use instructions::*;

declare_id!("FegMMZtuFu8ZUTjdkt2yRR1TmGEAFZbjpDJWpQ4ueqyG");

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Soundwork Create Program",
    project_url: "https://soundwork.io",
    contacts: "email:info@soundwork.io, twitter:@SoundworkSounds",
    policy: "https://github.com/SoundWorkLabs/marketplace-contracts/blob/master/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/SoundWorkLabs/marketplace-contracts"
}

/// SOUNDWORK CREATE
///
/// admin IXs to interact with the soundwork programs
#[program]
pub mod soundwork_create {
    use super::*;

    /// Create MPL Core Asset
    ///
    /// Expect
    /// 1. name - title of the asset
    /// 2. uri - off chain metadata uri
    pub fn create(ctx: Context<CreateAsset>, name: String, uri: String) -> Result<()> {
        CreateAsset::create_asset(ctx, name, uri)
    }
}
