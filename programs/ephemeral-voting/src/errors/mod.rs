use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Election title too long")]
    ElectionTitleTooLong,
    #[msg("Election description too long")]
    ElectionDescriptionTooLong,
    #[msg("Candidate name too long")]
    CandidateNameTooLong,
    #[msg("Election not found")]
    ElectionNotFound,
    #[msg("Election not active")]
    ElectionNotActive,
    #[msg("Election already ended")]
    ElectionAlreadyEnded,
    #[msg("Election not ended yet")]
    ElectionNotEndedYet,
    #[msg("Already voted in this election")]
    AlreadyVoted,
    #[msg("Invalid candidate index")]
    InvalidCandidateIndex,
    #[msg("Not authorized")]
    NotAuthorized,
    #[msg("Program already initialized")]
    AlreadyInitialized,
    #[msg("Too many candidates")]
    TooManyCandidates,
    #[msg("Election duration too short")]
    ElectionDurationTooShort,
    #[msg("Results already finalized")]
    ResultsAlreadyFinalized,
    #[msg("Delegation failed")]
    DelegationFailed,
    #[msg("Election not delegated to ephemeral rollup")]
    ElectionNotDelegated,
}
