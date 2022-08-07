use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, CloseAccount, TokenAccount, Transfer};

use crate::states::*;

#[derive(Accounts)]
pub struct CancelListing<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

impl <'info> CancelListing <'info> {
  fn transfer_to_seller_token_account(
    &self,
    authority_seeds: &[&[u8]]
  ) -> Result<()> {

    Ok(())
  }

  fn close_escrow_token_account(
    &self,
    authority_seeds: &[&[u8]]
  ) -> Result<()> {
      Ok(())
  }
}

pub fn handler(ctx: Context<CancelListing>) -> Result<()> {
  Ok(())
}

