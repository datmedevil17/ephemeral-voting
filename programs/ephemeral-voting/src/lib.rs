#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::{commit, delegate, ephemeral};
use ephemeral_rollups_sdk::cpi::DelegateConfig;
use ephemeral_rollups_sdk::ephem::{commit_accounts, commit_and_undelegate_accounts};

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("EmPjmyFcsQ6PaGZL8FE3QAsJXWwNuogAZXgmxZGNfDin");

#[program]
pub mod voting_platform {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeCtx>,
        ephemeral_rollup_authority: Pubkey,
    ) -> Result<()> {
        instructions::initialize(ctx, ephemeral_rollup_authority)
    }

    pub fn create_election(
        ctx: Context<CreateElectionCtx>,
        title: String,
        description: String,
        duration_hours: u64,
    ) -> Result<()> {
        instructions::create_election(ctx, title, description, duration_hours)
    }

    pub fn add_candidate(
        ctx: Context<AddCandidateCtx>,
        election_id: u64,
        name: String,
        description: String,
    ) -> Result<()> {
        instructions::add_candidate(ctx, election_id, name, description)
    }

    #[ephemeral]
    pub fn delegate_voting(
        ctx: Context<DelegateVotingCtx>,
        election_id: u64,
        delegate_config: DelegateConfig,
    ) -> Result<()> {
        instructions::delegate_voting(ctx, election_id, delegate_config)
    }

    #[ephemeral]
    pub fn cast_vote(
        ctx: Context<CastVoteCtx>,
        election_id: u64,
        candidate_index: u8,
    ) -> Result<()> {
        instructions::cast_vote(ctx, election_id, candidate_index)
    }

    #[commit]
    pub fn finalize_results(
        ctx: Context<FinalizeResultsCtx>,
        election_id: u64,
        vote_tallies: Vec<u64>,
        total_votes: u64,
    ) -> Result<()> {
        instructions::finalize_results(ctx, election_id, vote_tallies, total_votes)
    }

    pub fn get_election_results(
        ctx: Context<GetElectionResultsCtx>,
        election_id: u64,
    ) -> Result<()> {
        instructions::get_election_results(ctx, election_id)
    }
}