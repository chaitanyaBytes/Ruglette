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
    #[msg("Bet amount too small")]
    BetTooSmall,
    #[msg("Bet amount too large")]
    BetTooLarge,
}
