use anchor_lang::prelude::*;
use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::errors::ErrorCode::AlreadyInitialized;
use crate::states::ProgramState;

pub fn initialize(
    ctx: Context<InitializeCtx>, 
    ephemeral_rollup_authority: Pubkey
) -> Result<()> {
    let state = &mut ctx.accounts.program_state;
    let deployer = &ctx.accounts.deployer;

    if state.initialized {
        return Err(AlreadyInitialized.into());
    }

    state.election_count = 0;
    state.admin = deployer.key();
    state.ephemeral_rollup_authority = ephemeral_rollup_authority;
    state.total_votes_cast = 0;
    state.total_elections_finalized = 0;
    state.initialized = true;

    msg!("Voting platform initialized");
    msg!("Admin: {}", deployer.key());
    msg!("Ephemeral rollup authority: {}", ephemeral_rollup_authority);

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeCtx<'info> {
    #[account(
        init,
        payer = deployer,
        space = ANCHOR_DISCRIMINATOR_SIZE + ProgramState::INIT_SPACE,
        seeds = [b"program_state"],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,
    #[account(mut)]
    pub deployer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
