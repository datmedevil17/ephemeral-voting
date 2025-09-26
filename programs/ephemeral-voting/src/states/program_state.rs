use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProgramState {
    pub initialized: bool,
    pub election_count: u64,
    pub admin: Pubkey,
    pub ephemeral_rollup_authority: Pubkey,
    pub total_votes_cast: u64,
    pub total_elections_finalized: u64,
}