use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, CloseAccount, TokenAccount, Transfer};

use crate::states::*;

#[derive(Accounts)]
pub struct CancelListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
      mut,
      constraint = seller_token.owner == seller.to_account_info().key()
  )]
    pub seller_token: Account<'info, TokenAccount>,
    #[account(
      constraint = nft_mint.decimals == 0,
      constraint = nft_mint.supply == 1,
      constraint = seller_token.mint == nft_mint.key()
  )]
    pub nft_mint: Account<'info, Mint>,
    #[account(
      mut, 
      seeds = [listing_proof.seller_token.key().as_ref()], 
      bump = listing_proof.bump,
      constraint = listing_proof.nft_mint == nft_mint.key(),
      constraint = listing_proof.seller_key == seller.key(),
      constraint = listing_proof.seller_token == seller_token.key(),
      constraint = listing_proof.escrow_token == escrow_token.key(),
      close = seller,
    )]
    pub listing_proof: Account<'info, ListingProof>,
    #[account(
      mut,
      seeds = [listing_proof.seller_token.key().as_ref(), b"escrow-token"],
      bump,
      token::mint = nft_mint,
      token::authority = listing_proof,
      constraint = escrow_token.amount == 1,
  )]
    pub escrow_token: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

impl <'info> CancelListing <'info> {
  fn transfer_to_seller_token_account(
    &self,
    authority_seeds: &[&[u8]]
  ) -> Result<()> {
    let cpi_accounts = Transfer {
        from: self.escrow_token.to_account_info().clone(),
        to: self
            .seller_token
            .to_account_info()
            .clone(),
        authority: self.listing_proof.to_account_info().clone(),
    };

    token::transfer(
      CpiContext::new(self.token_program.clone(), cpi_accounts).
        with_signer(&[&authority_seeds[..]]),
      1
    )?;

    Ok(())
  }

  fn close_escrow_token_account(
    &self,
    authority_seeds: &[&[u8]]
  ) -> Result<()> {
      let cpi_accounts = CloseAccount {
          account: self.escrow_token.to_account_info().clone(),
          destination: self.seller.to_account_info().clone(),
          authority: self.listing_proof.to_account_info().clone(),
      };
      token::close_account(
        CpiContext::new(self.token_program.clone(), cpi_accounts)
          .with_signer(&[&authority_seeds[..]])
      )?;
      Ok(())
  }
}

pub fn handler(ctx: Context<CancelListing>) -> Result<()> {
  let listing_proof_seed = ctx.accounts.seller_token.to_account_info().key.as_ref();
  let authority_seeds = &[&listing_proof_seed[..], &[ctx.accounts.listing_proof.bump]];

  ctx.accounts.transfer_to_seller_token_account(authority_seeds)?;
  ctx.accounts.close_escrow_token_account(authority_seeds)?;
  Ok(())
}

