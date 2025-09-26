use anchor_lang::prelude::*;
use crate::errors::ErrorCode::*;
use crate::states::Election;
use ephemeral_rollups_sdk::cpi::DelegateConfig;
use ephemeral_rollups_sdk::anchor::delegate;

pub fn delegate_voting(
    ctx: Context<DelegateVotingCtx>,
    election_id: u64,
    delegate_config: DelegateConfig,
) -> Result<()> {
    let election = &mut ctx.accounts.election;

    if election.eid != election_id {
        return Err(ElectionNotFound.into());
    }

    if election.creator != ctx.accounts.creator.key() {
        return Err(NotAuthorized.into());
    }

    if election.delegated_to_ephemeral {
        return Err(DelegationFailed.into()); // Already delegated
    }

    if election.candidate_count == 0 {
        return Err(TooManyCandidates.into()); // No candidates added
    }

    // Delegate the election to ephemeral rollup
    let cpi_program = ctx.accounts.ephemeral_rollups_program.to_account_info();
    let cpi_accounts = delegate::DelegateAccounts {
        owner: ctx.accounts.creator.to_account_info(),
        magic_context: ctx.accounts.magic_context.to_account_info(),
        magic_program: ctx.accounts.magic_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    delegate::delegate(cpi_ctx, delegate_config)?;

    election.delegated_to_ephemeral = true;
    election.ephemeral_rollup_pda = ctx.accounts.magic_context.key();

    msg!("Election {} delegated to ephemeral rollup", election_id);
    msg!("Ephemeral PDA: {}", election.ephemeral_rollup_pda);

    Ok(())
}

#[derive(Accounts)]
#[instruction(election_id: u64)]
pub struct DelegateVotingCtx<'info> {
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
    /// CHECK: Ephemeral rollups magic context account
    #[account(mut)]
    pub magic_context: UncheckedAccount<'info>,
    /// CHECK: Magic program from ephemeral rollups
    pub magic_program: UncheckedAccount<'info>,
    /// CHECK: Ephemeral rollups program
    pub ephemeral_rollups_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}