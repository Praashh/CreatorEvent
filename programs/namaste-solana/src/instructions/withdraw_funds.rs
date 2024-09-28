use anchor_lang::prelude::*;

use crate::states::Platform;

#[derive(Accounts)]
pub struct WithdrawFromTreasury<'info> {
    #[account(mut)]
pub admin: Signer<'info>,
    #[account(
        seeds = [b"platform", platform.platform_name.as_bytes()],
        bump = platform.bump,
        has_one = admin
    )]
pub platform: Account<'info, Platform>,
    #[account(
        mut,
        seeds = [b"treasury", platform.key().as_ref()],
        bump = platform.treasury_bump,
    )]
pub treasury: SystemAccount<'info>,
pub system_program: Program<'info, System>,
}