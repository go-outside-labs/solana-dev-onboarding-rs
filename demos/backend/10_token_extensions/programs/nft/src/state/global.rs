use anchor_lang::prelude::*;

#[account]
pub struct NftData {
    pub mint: Pubkey,
    pub rule: Pubkey,
    pub expiry: i64,
}

#[account]
pub struct NftRule {
    pub seed: u64,
    pub creator: Pubkey,
    pub price: u64,
    pub treasury: Pubkey,
}


impl Space for NftData {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8;
}

impl Space for NftRule {
    const INIT_SPACE: usize = 8 + 8 + 32 + 8 + 32;
}
