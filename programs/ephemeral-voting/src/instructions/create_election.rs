use anchor_lang::prelude::*;
use crate::constants::*;
use crate::errors::ErrorCode::*;
use crate::states::{Election, ProgramState};

pub fn create_election(
    ctx: Context<CreateElectionCtx>,
    title: String,
    description: String,
    duration_hours: u64,
) -> Result<()> {
    let election = &mut ctx.accounts.election;
    let state = &mut ctx.accounts.program_state;

    if title.len() > MAX_ELECTION_TITLE_LENGTH {
        return Err(ElectionTitleTooLong.into());
    }

    if description.len() > MAX_ELECTION_DESCRIPTION_LENGTH {
        return Err(ElectionDescriptionTooLong.into());
    }

    if duration_hours < 1 {
        return Err(ElectionDurationTooShort.into());
    }

    let current_time = Clock::get()?.unix_timestamp as u64;
    
    state.election_count += 1;
    
    election.eid = state.election_count;
    election.creator = ctx.accounts.creator.key();
    election.title = title;
    election.description = description;
    election.start_time = current_time;
    election.end_time = current_time + (duration_hours * 3600);
    election.candidate_count = 0;
    election.total_votes = 0;
    election.active = true;
    election.results_finalized = false;
    election.delegated_to_ephemeral = false;
    election.ephemeral_rollup_pda = Pubkey::default();
    election.winner_candidate_index = 255; // Invalid index initially
    election.created_at = current_time;

    msg!("Election created: {}", election.eid);
    msg!("Duration: {} hours", duration_hours);

    Ok(())
}

#[derive(Accounts)]
pub struct CreateElectionCtx<'info> {
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(
        init,
        payer = creator,
        space = ANCHOR_DISCRIMINATOR_SIZE + Election::INIT_SPACE,
        seeds = [
            b"election",
            (program_state.election_count + 1).to_le_bytes().as_ref()
        ],
        bump
    )]
    pub election: Account<'info, Election>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
