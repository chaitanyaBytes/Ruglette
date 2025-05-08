use anchor_lang::error_code;

#[error_code]
pub enum ErrorCodes {
    #[msg("Game is paused")]
    GamePaused,
    #[msg("The given seed is invalid")]
    InvalidSeed,
    #[msg("The Player is invalid")]
    InvalidPlayer,
    #[msg("Invalid Round state")]
    RoundNotAcceptingBets,
    #[msg("Not Waiting For Random Number")]
    NotWaitingForRandomNumber,
    #[msg("The Result is Not Ready Yet")]
    ResultNotReady,
    #[msg("Bet amount too small")]
    BetTooSmall,
    #[msg("Bet amount too large")]
    BetTooLarge,
    #[msg("Round not settled yet")]
    RoundNotSettled,

    NotEnoughFundsToPlay,
    RandomnessAlreadyRevealed,
    InvalidRandomnessAccount,
    RandomnessExpired,
    RandomnessNotResolved,
}
