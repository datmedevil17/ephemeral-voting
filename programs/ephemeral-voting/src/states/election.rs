use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Election {
    pub eid: u64,
    pub creator: Pubkey,
    #[max_len(128)]
    pub title: String,
    #[max_len(512)]
    pub description: String,
    pub start_time: u64,
    pub end_time: u64,
    pub candidate_count: u8,
    pub total_votes: u64,
    pub active: bool,
    pub results_finalized: bool,
    pub delegated_to_ephemeral: bool,
    pub ephemeral_rollup_pda: Pubkey, // PDA for ephemeral rollup delegation
    pub winner_candidate_index: u8,
    pub created_at: u64,
}
