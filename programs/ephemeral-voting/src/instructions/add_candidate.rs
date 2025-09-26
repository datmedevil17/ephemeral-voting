use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::ErrorCode::*;
use crate::states::{Election, Candidate};

pub fn add_candidate(
    ctx: Context<AddCandidateCtx>,
    election_id: u64,
    name: String,
    description: String,
) -> Result<()> {
    let election = &mut ctx.accounts.election;
    let candidate = &mut ctx.accounts.candidate;

    if election.eid != election_id {
        return Err(ElectionNotFound.into());
    }

    if election.creator != ctx.accounts.creator.key() {
        return Err(NotAuthorized.into());
    }

    if name.len() > MAX_CANDIDATE_NAME_LENGTH {
        return Err(CandidateNameTooLong.into());
    }

    if election.candidate_count >= MAX_CANDIDATES_PER_ELECTION as u8 {
        return Err(TooManyCandidates.into());
    }

    candidate.election_id = election_id;
    candidate.candidate_index = election.candidate_count;
    candidate.name = name;
    candidate.description = description;
    candidate.vote_count = 0;
    candidate.percentage = 0;

    election.candidate_count += 1;

    msg!("Candidate added: {} (index: {})", candidate.name, candidate.candidate_index);

    Ok(())
}

#[derive(Accounts)]
#[instruction(election_id: u64)]
pub struct AddCandidateCtx<'info> {
    #[account(
        mut,
        seeds = [
            b"election",
            election_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub election: Account<'info, Election>,
    #[account(
        init,
        payer = creator,
        space = ANCHOR_DISCRIMINATOR_SIZE + Candidate::INIT_SPACE,
        seeds = [
            b"candidate",
            election_id.to_le_bytes().as_ref(),
            election.candidate_count.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub candidate: Account<'info, Candidate>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}