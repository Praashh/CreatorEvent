use anchor_lang::prelude::*;

declare_id!("F8hZjtco3YmBv3hhoPTHVXqQM2LD3JgkctQcaNCsw4FJ");

#[program]
pub mod namaste_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
