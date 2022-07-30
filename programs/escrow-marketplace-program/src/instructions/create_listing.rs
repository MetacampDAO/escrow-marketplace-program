use anchor_lang::prelude::*;

use crate::states::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
pub struct CreateListing<'info>{
  #[account(mut)]
  pub seller: Signer<'info>,
  #[account(
      constraint = seller_token.amount == 1,
      constraint = seller_token.owner == seller.to_account_info().key()
  )]
  pub seller_token: Account<'info, TokenAccount>,
  #[account(
      constraint = seller_token.mint == nft_mint.key()
  )]
  pub nft_mint: Account<'info, Mint>,
  #[account(init, payer = seller, space = 8 + ESCROW_INFO_LEN, seeds = [nft_mint.key().as_ref(), b"escrow-info"], bump)]
  pub escrow_info: Account<'info, EscrowInfo>,
  #[account(
      init,
      seeds = [nft_mint.key().as_ref(), b"escrow-token"],
      bump,
      payer = seller,
      token::mint = nft_mint,
      token::authority = seller,
  )]
  pub vault_account: Account<'info, TokenAccount>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub token_program: AccountInfo<'info>,
}