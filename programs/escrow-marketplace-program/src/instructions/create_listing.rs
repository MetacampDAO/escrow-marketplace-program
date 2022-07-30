use anchor_lang::prelude::*;

use crate::states::*;
use spl_token::instruction::AuthorityType;
use anchor_spl::token::{self, Mint, SetAuthority, TokenAccount, Transfer};

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
  pub escrow_token: Account<'info, TokenAccount>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub token_program: AccountInfo<'info>,
}

impl <'info> CreateListing <'info> {
  fn transfer_to_escrow_token(&self) -> Result<()> {
      let cpi_accounts = Transfer {
          from: self
              .seller_token
              .to_account_info()
              .clone(),
          to: self.escrow_token.to_account_info().clone(),
          authority: self.seller.to_account_info().clone(),
      };
      token::transfer(CpiContext::new(self.token_program.clone(), cpi_accounts), 1);
      Ok(())
  }

  fn set_authority_to_escrow_info(&self) -> Result<()> {
      let cpi_accounts = SetAuthority {
          account_or_mint: self.escrow_token.to_account_info().clone(),
          current_authority: self.seller.to_account_info().clone(),
      };
      token::set_authority(
        CpiContext::new(self.token_program.clone(), cpi_accounts), 
        AuthorityType::AccountOwner, 
        Some(self.escrow_info.key())
      );
      Ok(())
  }
}