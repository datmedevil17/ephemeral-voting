use anchor_lang::prelude::*;
use crate::errors::ErrorCode::*;
use crate::states::{Election, Candidate, ProgramState};
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;

pub fn finalize_results(
    ctx: Context<FinalizeResultsCtx>,
    election_id: u64,
    vote_tallies: Vec<u64>,
    total_votes: u64,
) -> Result<()> {
    let election = &mut ctx.accounts.election;
    let state = &mut ctx.accounts.program_state;

    if election.eid != election_id {
        return Err(ElectionNotFound.into());
    }

    if !election.delegated_to_ephemeral {
        return Err(ElectionNotDelegated.into());
    }

    let current_time = Clock::get()?.unix_timestamp as u64;
    if current_time <= election.end_time {
        return Err(ElectionNotEndedYet.into());
    }

    if election.results_finalized {
        return Err(ResultsAlreadyFinalized.into());
    }

    if vote_tallies.len() != election.candidate_count as usize {
        return Err(InvalidCandidateIndex.into());
    }

    // Commit and undelegate from ephemeral rollup
    let cpi_program = ctx.accounts.ephemeral_rollups_program.to_account_info();
    let cpi_accounts = commit_and_undelegate_accounts::CommitAndUndelegateAccounts {
        magic_context: ctx.accounts.magic_context.to_account_info(),
        magic_program: ctx.accounts.magic_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // Commit the results from ephemeral rollup
    commit_and_undelegate_accounts::commit_and_undelegate(cpi_ctx)?;

    // Store the final tallies and determine winner
    election.total_votes = total_votes;
    election.results_finalized = true;
    election.active = false;
    election.delegated_to_ephemeral = false;

    let mut max_votes = 0u64;
    let mut winner_index = 0u8;

    // Update candidate accounts with final vote counts
    for (i, &votes) in vote_tallies.iter().enumerate() {
        if votes > max_votes {
            max_votes = votes;
            winner_index = i as u8;
        }

        // Calculate percentage (multiply by 10000 for 2 decimal precision)
        let percentage = if total_votes > 0 {
            ((votes * 10000) / total_votes) as u16
        } else {
            0
        };

        // Update candidate account if provided in remaining accounts
        if i < ctx.remaining_accounts.len() {
            let candidate_account = &ctx.remaining_accounts[i];
            let mut candidate_data = Account::<Candidate>::try_from(candidate_account)?;
            candidate_data.vote_count = votes;
            candidate_data.percentage = percentage;
            candidate_data.exit(&crate::ID)?;
        }

        msg!("Candidate {}: {} votes ({}%)", i, votes, percentage as f64 / 100.0);
    }

    election.winner_candidate_index = winner_index;
    state.total_votes_cast += total_votes;
    state.total_elections_finalized += 1;

    msg!("Election {} finalized with ephemeral rollup", election_id);
    msg!("Winner: Candidate {}", winner_index);
    msg!("Total votes: {}", total_votes);
    msg!("Results committed from ephemeral rollup");

    Ok(())
}

#[derive(Accounts)]
#[instruction(election_id: u64)]
pub struct FinalizeResultsCtx<'info> {
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(
        mut,
        seeds = [
            b"election",
            election_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub election: Account<'info, Election>,
    /// CHECK: Ephemeral rollups magic context account
    #[account(mut)]
    pub magic_context: UncheckedAccount<'info>,
    /// CHECK: Magic program from ephemeral rollups
    pub magic_program: UncheckedAccount<'info>,
    /// CHECK: Ephemeral rollups program
    pub ephemeral_rollups_program: UncheckedAccount<'info>,
    pub authority: Signer<'info>, // Can be election creator or admin
}