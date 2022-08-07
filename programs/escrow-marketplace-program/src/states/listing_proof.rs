use anchor_lang::prelude::*;

pub const LISTING_PROOF_LEN: usize = 32 // NFT MINT
  + 32  // SELLER KEY
  + 32  // SELLER TOKEN ACCOUNT
  + 32  // ESCROW TOKEN ACCOUNT
  + 16  // LIST PRICE 
  + 1; // bump

#[account]
pub struct ListingProof {
    pub nft_mint: Pubkey,       // NFT Mint listed, used to verify if Token Account passed has same Mint 
    pub seller_key: Pubkey,     // Seller who listed, used to verify when cancelling and buying
    pub seller_token: Pubkey,   // Seller's token account, used to verify when cancelling
    pub escrow_token: Pubkey,   // Escrow's token account, used to verify when cancelling and buyuing
    pub list_price: u128,       // Listing price
    pub bump: u8,               // to verify the bump we passed in is consistent
}

impl ListingProof {
    pub fn init_listing_proof(
        &mut self,
        nft_mint: Pubkey,
        seller_key: Pubkey,
        seller_token: Pubkey,
        escrow_token: Pubkey,
        list_price: u128,
        bump: u8,
    ) {
        self.nft_mint = nft_mint;
        self.seller_key = seller_key;
        self.seller_token = seller_token;
        self.escrow_token = escrow_token;
        self.list_price = list_price;
        self.bump = bump;
    }
}
