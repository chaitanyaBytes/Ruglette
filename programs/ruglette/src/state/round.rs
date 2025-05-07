pub use anchor_lang::prelude::*;

use crate::GameStatus;

#[account]
#[derive(InitSpace)]
pub struct RoundState {
    /// The player who placed the bet
    pub player: Pubkey,
    /// Timestamp when round started
    pub start_time: i64,
    /// The winning number (0-36) once determined
    pub winning_number: Option<u8>,
    /// Total amount wagered in this round
    pub total_wagered: u64,
    /// status of the round
    pub status: GameStatus,
    /// Switchboard VRF account
    pub vrf_account: Pubkey,
    /// Result buffer
    pub result_buffer: [u8; 32],
    /// Bump for PDA derivation
    pub bump: u8,
}
