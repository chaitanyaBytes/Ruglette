pub use anchor_lang::prelude::*;

use crate::BetType;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Bet {
    pub bet_type: BetType,
    pub amount: u64,
}

#[account]
#[derive(InitSpace)]
pub struct BetState {
    /// The round this bet belongs to
    pub round: Pubkey,
    /// Type of bets placed
    #[max_len(10)]
    pub bets: Vec<Bet>,
    /// Timestamp when bet was placed
    pub timestamp: i64,
    /// Bump for PDA derivation
    pub bump: u8,
}
