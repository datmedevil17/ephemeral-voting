use anchor_lang::prelude::*;
use crate::errors::ErrorCode::*;
use crate::states::{Election, Candidate};

pub fn get_election_results(
    ctx: Context<GetElectionResultsCtx>,
    election_id: u64,
) -> Result<()> {
    let election = &ctx.accounts.election;

    if election.eid != election_id {
        return Err(ElectionNotFound.into());
    }

    if !election.results_finalized {
        return Err(ResultsAlreadyFinalized.into()); // Reusing error
    }

    msg!("Election Results for Election {}", election_id);
    msg!("Title: {}", election.title);
    msg!("Total Votes: {}", election.total_votes);
    msg!("Winner: Candidate {}", election.winner_candidate_index);
    msg!("Finalized: {}", election.results_finalized);

    // Output candidate results if provided
    for (i, candidate_account) in ctx.remaining_accounts.iter().enumerate() {
        let candidate_data = Account::<Candidate>::try_from(candidate_account)?;
        msg!("Candidate {}: {} - {} votes ({}%)", 
             i, 
             candidate_data.name, 
             candidate_data.vote_count,
             candidate_data.percentage as f64 / 100.0
        );
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(election_id: u64)]
pub struct GetElectionResultsCtx<'info> {
    #[account(
        seeds = [
            b"election",
            election_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub election: Account<'info, Election>,
}