use anchor_lang::prelude::*;

pub use instructions::*;

pub mod instructions;
pub mod states;
declare_id!("DBZ5u3AaFpJMEGKhfQgmXQqUtTzQ5KpjXG9eZRn9R7cV");

#[program]
pub mod escrow_marketplace_program {

    use super::*;

    pub fn create_listing(
        ctx: Context<CreateListing>,
        list_price: u128,
        listing_proof_bump: u8,
    ) -> Result<()> {
        instructions::create_listing::handler(ctx, list_price, listing_proof_bump)
    }

    pub fn purchase_listing(ctx: Context<PurchaseListing>) -> Result<()> {
        instructions::purchase_listing::handler(ctx)
    }

    pub fn cancel_listing(ctx: Context<CancelListing>)  -> Result<()> {
        instructions::cancel_listing::handler(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
