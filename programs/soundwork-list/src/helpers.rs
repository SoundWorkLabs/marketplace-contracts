use std::result::Result as StdResult;

use solana_program::pubkey::Pubkey;

// todo(Jimii): use royalty plugin
// selling price + protocol fee + royalty
pub fn calculate_total_buy_fee(amount: u64, taker_fee_bps: u8) -> StdResult<u64, &'static str> {
    let fee = amount
        .checked_mul(taker_fee_bps as u64)
        .ok_or("fee calculation overflow")?
        .checked_div(10000)
        .ok_or("fee calculation overflow")?;

    // todo: royalty

    let total = amount.checked_add(fee).ok_or("")?;

    Ok(total)
}

// export core program type
// todo: remove and use SPL typed account
#[derive(Clone)]
pub struct Core;

impl anchor_lang::Id for Core {
    fn id() -> Pubkey {
        mpl_core::ID
    }
}
