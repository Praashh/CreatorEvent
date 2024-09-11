use anchor_lang::prelude::*;
mod state;
mod error;

declare_id!("F8hZjtco3YmBv3hhoPTHVXqQM2LD3JgkctQcaNCsw4FJ");


#[program]
pub mod nf_ticket {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name:String, fee:u16) -> Result<()> {
        // ctx.accounts.initialize(name, fee);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
