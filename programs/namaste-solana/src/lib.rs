use anchor_lang::prelude::*;
mod state;
mod error;

declare_id!("F8hZjtco3YmBv3hhoPTHVXqQM2LD3JgkctQcaNCsw4FJ");


#[program]
pub mod nf_ticket {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name:String, fee:u16) -> Result<()> {
        ctx.accounts.initialize(name, fee);
        Ok(())
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.list(price, ctx.bumps)
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.delist(ctx.bumps)
    }

    pub fn buy(ctx: Context<Buy>) -> Result<()> {
        ctx.accounts.buy(ctx.bumps)
    }

    pub fn bid(ctx: Context<Bid>, amount: u64) -> Result<()> {
        ctx.accounts.bid(amount)
    }

    pub fn accept_bid(ctx: Context<AcceptBid>) -> Result<()> {
        ctx.accounts.accept_bid(ctx.bumps)
    }

    pub fn cancel_bid(ctx: Context<CancelBid>) -> Result<()> {
        ctx.accounts.cancel_bid(ctx.bumps)
    }

    pub fn modify_bid(ctx: Context<ModifyBid>, amount: u64) -> Result<()> {
        ctx.accounts.modify_bid(amount, ctx.bumps)
    }
}

