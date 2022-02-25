use crate::state::*;

use anchor_lang::prelude::*;
use solana_program::system_program;

use std::mem::size_of;

#[derive(Accounts)]
#[instruction(
    daemon_bump: u8,
    fee_bump: u8,
)]
pub struct DaemonCreate<'info> {
    #[account(
        init,
        seeds = [
            SEED_DAEMON, 
            owner.key().as_ref()
        ],
        bump,
        payer = owner,
        space = 8 + size_of::<Daemon>(),
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        init,
        seeds = [
            SEED_FEE, 
            daemon.key().as_ref()
        ],
        bump,
        payer = owner,
        space = 8 + size_of::<Fee>(),
    )]
    pub fee: Account<'info, Fee>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DaemonCreate>, daemon_bump: u8, fee_bump: u8) -> Result<()> {
    let daemon = &mut ctx.accounts.daemon;
    let fee = &mut ctx.accounts.fee;
    let owner = &ctx.accounts.owner;

    daemon.init(owner.key(), daemon_bump)?;
    fee.init(daemon.key(), fee_bump)
}