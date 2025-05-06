use anchor_lang::prelude::*;

#[account]
pub struct GameState {
    pub player: Pubkey,
    pub bet_amount: u64,
}
