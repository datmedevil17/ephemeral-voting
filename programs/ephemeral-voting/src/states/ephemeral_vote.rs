use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EphemeralVote {
    pub voter: Pubkey,
    pub election_id: u64,
    pub candidate_index: u8,
    pub timestamp: u64,
    pub vote_weight: u64, // For weighted voting if needed
}