use anchor_lang::prelude::*;

use crate::states::*;
use anchor_spl::token::{self, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;

#[derive(Accounts)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
      mut,
      constraint = seller_token.amount == 1,
      constraint = seller_token.owner == seller.to_account_info().key()
  )]
    pub seller_token: Account<'info, TokenAccount>,
    #[account(
      constraint = nft_mint.decimals == 0,
      constraint = nft_mint.supply == 1,
      constraint = seller_token.mint == nft_mint.key()
  )]
    pub nft_mint: Account<'info, Mint>,
    #[account(init, payer = seller, space = 8 + LISTING_PROOF_LEN, seeds = [seller_token.key().as_ref()], bump)]
    pub listing_proof: Account<'info, ListingProof>,
    #[account(
      init,
      seeds = [seller_token.key().as_ref(), b"escrow-token"],
      bump,
      payer = seller,
      token::mint = nft_mint,
      token::authority = seller,
  )]
    pub escrow_token: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

impl<'info> CreateListing<'info> {
    fn set_authority_to_listing_proof(&self) -> Result<()> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.escrow_token.to_account_info().clone(),
            current_authority: self.seller.to_account_info().clone(),
        };

        token::set_authority(
            CpiContext::new(self.token_program.clone(), cpi_accounts),
            AuthorityType::AccountOwner,
            Some(self.listing_proof.key()),
        )?;

        Ok(())
    }

    fn transfer_to_escrow_token(&self) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.seller_token.to_account_info().clone(),
            to: self.escrow_token.to_account_info().clone(),
            authority: self.seller.to_account_info().clone(),
        };

        token::transfer(CpiContext::new(self.token_program.clone(), cpi_accounts), 1)?;

        Ok(())
    }
}

pub fn handler(ctx: Context<CreateListing>, list_price: u128, listing_proof_bump: u8) -> Result<()> {
    ctx.accounts.listing_proof.init_listing_proof(
        ctx.accounts.nft_mint.key(),
        ctx.accounts.seller.key(),
        ctx.accounts.seller_token.key(),
        ctx.accounts.escrow_token.key(),
        list_price,
        listing_proof_bump,
    );

    ctx.accounts.set_authority_to_listing_proof()?;
    ctx.accounts.transfer_to_escrow_token()?;

    Ok(())
}