use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VoteRecord {
    pub voter: Pubkey,
    pub election_id: u64,
    pub candidate_index: u8,
    pub timestamp: u64,
    pub processed_in_ephemeral: bool,
}