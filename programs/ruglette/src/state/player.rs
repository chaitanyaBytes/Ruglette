pub use anchor_lang::prelude::*;

#[account]
pub struct PlayerState {
    /// Player's wallet
    pub owner: Pubkey,
    /// Total amount wagered lifetime
    pub lifetime_wagered: u64,
    /// Total amount won lifetime
    pub lifetime_won: u64,
    /// Number of bets placed
    pub total_bets: u64,
    /// Number of wins
    pub total_wins: u64,
    /// Last bet timestamp
    pub last_bet_time: i64,
    /// Bump for PDA derivation
    pub bump: u8,
}
