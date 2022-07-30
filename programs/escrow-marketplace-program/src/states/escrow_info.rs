use anchor_lang::prelude::*;

pub const ESCROW_INFO_LEN: usize = 32 // NFT MINT
  + 32  // SELLER KEY
  + 32  // SELLER TOKEN ACCOUNT
  + 32  // ESCROW TOKEN ACCOUNT
  + 16; // LIST PRICE 

#[account]
pub struct EscrowInfo {
  pub nft_mint: Pubkey,
  pub seller_key: Pubkey,
  pub seller_token: Pubkey,
  pub escrow_token: Pubkey,
  pub list_price: u128,
}

impl EscrowInfo {
  pub fn init_escrow_info(&mut self, nft_mint: Pubkey, seller_key: Pubkey, seller_token: Pubkey, escrow_token: Pubkey, list_price: u128) {
    self.nft_mint = nft_mint;
    self.seller_key = seller_key;
    self.seller_token = seller_token;
    self.escrow_token = escrow_token;
    self.list_price = list_price;
  }
}