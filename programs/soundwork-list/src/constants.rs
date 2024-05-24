use anchor_lang::{prelude::*, solana_program::pubkey};

#[constant]
pub const SEED_PREFIX: &[u8] = b"Kessoku";

#[constant]
pub const SEED_ASSET_MANAGER: &[u8] = b"Seika";

#[constant]
pub const SEED_MARKETPLACE_CONFIG: &[u8] = b"Ijichi";

#[constant]
pub const SEED_LISTING_DATA: &[u8] = b"Hitori";

#[constant]
pub const SEED_WALLET: &[u8] = b"Yamada";

#[constant]
pub const ADMIN_ADDRESS: Pubkey = pubkey!("4kg8oh3jdNtn7j2wcS7TrUua31AgbLzDVkBZgTAe44aF");

#[constant]
pub const TREASURY_ADDRESS: Pubkey = pubkey!("4kg8oh3jdNtn7j2wcS7TrUua31AgbLzDVkBZgTAe44aF");
