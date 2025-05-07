use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GameState {
    /// The authority who can update game settings
    pub authority: Pubkey,
    /// Minimum bet amount in lamports
    pub min_bet: u64,
    /// Maximum bet amount in lamports
    pub max_bet: u64,
    /// House fee percentage (e.g., 270 for 2.7%)
    pub house_fee_basis_points: u16,
    // /// Total volume of bets placed
    // pub total_volume: u64,
    // /// Total fees collected
    // pub total_fees: u64,
    /// Whether the game is paused
    pub is_paused: bool,
    /// Bump for PDA derivation
    pub bump: u8,
    /// The bump seed for the House Vault PDA (stores collected fees)
    pub house_vault_bump: u8,
}
