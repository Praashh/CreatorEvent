use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub admin: Pubkey, // 32
    pub fee: u16,      // 2
    pub name: String,  // 24 base size of string ( without content )
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 32 + 2 + 24;
}

#[account]
pub struct Listing {
    pub lister: Pubkey,     // 32
    pub nft: Pubkey,        // 2
    pub collection: Pubkey, // 32
    pub price: u64,         // 8
}

impl Space for Listing {
    const INIT_SPACE: usize = 8 + 3 * 32 + 8;
}

#[account]
pub struct BidState {
    pub bidder: Pubkey, // 32
    pub price: u64,     // 8
}

impl Space for BidState {
    const INIT_SPACE: usize = 8 + 32 + 8;
}
