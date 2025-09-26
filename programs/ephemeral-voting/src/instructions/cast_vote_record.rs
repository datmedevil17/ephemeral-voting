use anchor_lang::prelude::*;
use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::errors::ErrorCode::*;
use crate::states::{Election, VoteRecord};

pub fn cast_vote_record(
    ctx: Context<CastVoteRecordCtx>,
    election_id: u64,
    ephemeral_vote_hash: [u8; 32],
) -> Result<()> {
    let election = &ctx.accounts.election;
    let vote_record = &mut ctx.accounts.vote_record;

    if election.eid != election_id {
        return Err(ElectionNotFound.into());
    }

    if !election.active || !election.ephemeral_rollup_active {
        return Err(ElectionNotActive.into());
    }

    let current_time = Clock::get()?.unix_timestamp as u64;
    if current_time > election.end_time {
        return Err(ElectionAlreadyEnded.into());
    }

    // This creates a vote record that tracks the user voted
    // The actual vote is cast in the ephemeral rollup
    vote_record.voter = ctx.accounts.voter.key();
    vote_record.election_id = election_id;
    vote_record.voted = true;
    vote_record.timestamp = current_time;
    vote_record.ephemeral_vote_hash = ephemeral_vote_hash;

    msg!("Vote record created for voter: {}", ctx.accounts.voter.key());
    msg!("Ephemeral vote hash: {:?}", ephemeral_vote_hash);

    Ok(())
}

#[derive(Accounts)]
#[instruction(election_id: u64)]
pub struct CastVoteRecordCtx<'info> {
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