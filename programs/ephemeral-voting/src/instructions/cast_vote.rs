use anchor_lang::prelude::*;
use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::errors::ErrorCode::*;
use crate::states::{Election, EphemeralVote, VoteRecord};

pub fn cast_vote(
    ctx: Context<CastVoteCtx>,
    election_id: u64,
    candidate_index: u8,
) -> Result<()> {
    let election = &ctx.accounts.election;
    let ephemeral_vote = &mut ctx.accounts.ephemeral_vote;
    let vote_record = &mut ctx.accounts.vote_record;

    if election.eid != election_id {
        return Err(ElectionNotFound.into());
    }

    if !election.active {
        return Err(ElectionNotActive.into());
    }

    if !election.delegated_to_ephemeral {
        return Err(ElectionNotDelegated.into());
    }

    let current_time = Clock::get()?.unix_timestamp as u64;
    if current_time > election.end_time {
        return Err(ElectionAlreadyEnded.into());
    }

    if candidate_index >= election.candidate_count {
        return Err(InvalidCandidateIndex.into());
    }

    // Store vote in ephemeral account (will be processed in rollup)
    ephemeral_vote.voter = ctx.accounts.voter.key();
    ephemeral_vote.election_id = election_id;
    ephemeral_vote.candidate_index = candidate_index;
    ephemeral_vote.timestamp = current_time;
    ephemeral_vote.vote_weight = 1; // Default weight

    // Create vote record for tracking
    vote_record.voter = ctx.accounts.voter.key();
    vote_record.election_id = election_id;
    vote_record.candidate_index = candidate_index;
    vote_record.timestamp = current_time;
    vote_record.processed_in_ephemeral = true;

    msg!("Vote cast in ephemeral rollup");
    msg!("Voter: {}", ctx.accounts.voter.key());
    msg!("Election: {}, Candidate: {}", election_id, candidate_index);

    Ok(())
}

#[derive(Accounts)]
#[instruction(election_id: u64)]
pub struct CastVoteCtx<'info> {
    #[account(
        seeds = [
            b"election",
            election_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub election: Account<'info, Election>,
    #[account(
        init,
        payer = voter,
        space = ANCHOR_DISCRIMINATOR_SIZE + EphemeralVote::INIT_SPACE,
        seeds = [
            b"ephemeral_vote",
            voter.key().as_ref(),
            election_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub ephemeral_vote: Account<'info, EphemeralVote>,
    #[account(
        init,
        payer = voter,
        space = ANCHOR_DISCRIMINATOR_SIZE + VoteRecord::INIT_SPACE,
        seeds = [
            b"vote_record",
            voter.key().as_ref(),
            election_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>,
    #[account(mut)]
    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}