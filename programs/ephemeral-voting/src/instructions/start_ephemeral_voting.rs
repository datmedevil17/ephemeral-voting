use anchor_lang::prelude::*;
use crate::errors::ErrorCode::*;
use crate::states::Election;

pub fn start_ephemeral_voting(
    ctx: Context<StartEphemeralVotingCtx>,
    election_id: u64,
    duration_hours: u64,
) -> Result<()> {
    let election = &mut ctx.accounts.election;

    if election.eid != election_id {
        return Err(ElectionNotFound.into());
    }

    if election.creator != ctx.accounts.creator.key() {
        return Err(NotAuthorized.into());
    }

    if election.active {
        return Err(ElectionNotActive.into());
    }

    if election.candidate_count == 0 {
        return Err(TooManyCandidates.into()); // Reusing error for no candidates
    }

    let current_time = Clock::get()?.unix_timestamp as u64;
    
    election.start_time = current_time;
    election.end_time = current_time + (duration_hours * 3600);
    election.active = true;
    election.ephemeral_rollup_active = true;

    msg!("Ephemeral voting started for election: {}", election_id);
    msg!("Voting period: {} hours", duration_hours);
    msg!("End time: {}", election.end_time);

    Ok(())
}

#[derive(Accounts)]
#[instruction(election_id: u64)]
pub struct StartEphemeralVotingCtx<'info> {
    #[account(
        mut,
        seeds = [
            b"election",
            election_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub election: Account<'info, Election>,
    #[account(mut)]
    pub creator: Signer<'info>,
}