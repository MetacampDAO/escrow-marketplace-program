use anchor_lang::{prelude::*, solana_program::system_instruction};
use anchor_lang::solana_program::program::{invoke};
use anchor_spl::token::{self, Mint, CloseAccount, TokenAccount, Transfer};

use crate::states::*;

#[derive(Accounts)]
pub struct PurchaseListing<'info> {
  #[account(mut)]
  pub buyer: Signer<'info>,
  #[account(
    mut,
    constraint = buyer_token.owner == buyer.key()
  )]
  pub buyer_token: Account<'info, TokenAccount>,
  #[account(
    constraint = nft_mint.decimals == 0,
    constraint = nft_mint.supply == 1,
    constraint = buyer_token.mint == nft_mint.key()
)]
  pub nft_mint: Account<'info, Mint>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  #[account(mut)]
  pub seller: AccountInfo<'info>,
  #[account(
    mut, 
    seeds = [escrow_info.seller_token.key().as_ref()], 
    bump = escrow_info.bump,
    constraint = escrow_info.nft_mint == nft_mint.key(),
    constraint = escrow_info.seller_key == seller.key(),
    constraint = escrow_info.escrow_token == escrow_token.key(),
    close = seller,
  )]
  pub escrow_info: Account<'info, EscrowInfo>,
  #[account(
    mut,
    seeds = [escrow_info.seller_token.key().as_ref(), b"escrow-token"],
    bump,
    token::mint = nft_mint,
    token::authority = escrow_info,
    constraint = escrow_token.amount == 1,
)]
  pub escrow_token: Account<'info, TokenAccount>,
  pub system_program: Program<'info, System>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub token_program: AccountInfo<'info>,
}

impl <'info> PurchaseListing <'info> {
  fn transfer_to_buyer_token_account(
    &self,
    authority_seeds: &[&[u8]]
  ) -> Result<()> {
    let cpi_accounts = Transfer {
        from: self.escrow_token.to_account_info().clone(),
        to: self
            .buyer_token
            .to_account_info()
            .clone(),
        authority: self.escrow_info.to_account_info().clone(),
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
          destination: self.seller.clone(),
          authority: self.escrow_info.to_account_info().clone(),
      };
      token::close_account(
        CpiContext::new(self.token_program.clone(), cpi_accounts)
          .with_signer(&[&authority_seeds[..]])
      )?;
      Ok(())
  }
}

pub fn handler(ctx: Context<PurchaseListing>) -> Result<()> {
  let escrow_info_seed = ctx.accounts.escrow_info.seller_token.as_ref();
  let authority_seeds = &[&escrow_info_seed[..], &[ctx.accounts.escrow_info.bump]];

  ctx.accounts.transfer_to_buyer_token_account(authority_seeds)?;
  ctx.accounts.close_escrow_token_account(authority_seeds)?;

  invoke(
    &system_instruction::transfer(
      ctx.accounts.buyer.key, 
      ctx.accounts.seller.key,
      ctx.accounts.escrow_info.list_price as u64
    ),
    &[
      ctx.accounts.buyer.to_account_info().clone(),
      ctx.accounts.seller.clone(),
      ctx.accounts.system_program.to_account_info().clone()
    ],
  )?;

  Ok(())
}