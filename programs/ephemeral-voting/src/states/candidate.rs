use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    pub election_id: u64,
    pub candidate_index: u8,
    #[max_len(64)]
    pub name: String,
    #[max_len(256)]
    pub description: String,
    pub vote_count: u64,
    pub percentage: u16, // Percentage * 100 (e.g., 2550 = 25.50%)
}