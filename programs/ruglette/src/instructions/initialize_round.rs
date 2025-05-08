#![allow(unexpected_cfgs)]
pub use anchor_lang::prelude::*;

use crate::{RoundState, GameState, GameStatus};
use crate::error::ErrorCodes;

#[derive(Accounts)]
#[instruction(start_time: i64)]
pub struct InitializeRound<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    pub authority: SystemAccount<'info>,

    #[account(
        init, 
        payer = player,
        space = 8 + RoundState::INIT_SPACE,
        seeds = [b"round", player.key().as_ref(), start_time.to_le_bytes().as_ref()],
        bump
    )]
    pub round: Account<'info, RoundState>,
    
    #[account(
        seeds = [b"game", authority.key().as_ref()],
        bump = game.bump
    )]
    pub game: Account<'info, GameState>,
    
    pub system_program: Program<'info, System>
}

impl<'info> InitializeRound<'info> {
    pub fn initialize_round(&mut self, start_time: i64, bumps: &InitializeRoundBumps) -> Result<()> {
        // Check if game is paused
    require!(!self.game.is_paused, ErrorCodes::GamePaused);
    
    // Optional validation
    require!(seed > 0, ErrorCodes::InvalidSeed);

        self.round.set_inner(RoundState {
            player: self.player.key(),
            start_time,
            winning_number: None,
            total_wagered: 0,
            payout_amount: None,
            status: GameStatus::AcceptingBets,
            vrf_account: Pubkey::default(),
            result_buffer: [0; 32],
            bump: bumps.round,
        });
        Ok(())
    }
}