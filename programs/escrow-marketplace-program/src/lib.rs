use anchor_lang::prelude::*;

pub use instructions::*;

pub mod states;
pub mod instructions;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod escrow_marketplace_program {

    use super::*;

    pub fn create_listing(ctx: Context<CreateListing>, list_price: u128, escrow_info_bump: u8) -> Result<()> {
        instructions::create_listing::handler(ctx, list_price, escrow_info_bump)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
