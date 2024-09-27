use anchor_lang::prelude::*;
use mpl_core::{
    ID as MPL_CORE_ID,
    accounts::BaseCollectionV1, 
};

use crate::states::Manager;
use crate::states::Platform;

#[derive(Accounts)]
pub struct CreateTicket<'info> {
   pub signer: Signer<'info>,
   #[account(mut)]
   pub payer: Signer<'info>,
   #[account(mut)]
   #[account(
       seeds = [b"manager", signer.key().as_ref()],
       bump = manager.bump
   )]
   pub manager: Account<'info, Manager>,
   #[account(
       seeds = [b"platform", platform.platform_name.as_str().as_bytes()],
       bump = platform.bump,
   )]
   platform: Box<Account<'info, Platform>>,
   #[account(
       mut,
       constraint = event.update_authority == manager.key(),
   )]
   pub event: Account<'info, BaseCollectionV1>,
   #[account(mut)]
   pub ticket: Signer<'info>,
   #[account(
       seeds = [b"treasury", platform.key().as_ref()],
       bump = platform.treasury_bump,
   )]
   pub treasury: SystemAccount<'info>,
   pub system_program: Program<'info, System>,
   #[account(address = MPL_CORE_ID)]
   /// CHECK: This is checked by the address constraint
   pub mpl_core_program: UncheckedAccount<'info>
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateTicketArgs {
    pub name: String,
    pub uri: String,
    pub price: u64,
    pub venue_authority: Pubkey,
    pub screen: Option<String>,
    pub row: Option<String>,
    pub seat: Option<String>,
}


