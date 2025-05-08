#![allow(unexpected_cfgs)]
pub use anchor_lang::prelude::*;

use crate::error::ErrorCodes;
use crate::{transfer, Bet, BetState, GameState, GameStatus, RoundState};

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    pub authority: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"round", player.key().as_ref(), round.start_time.to_le_bytes().as_ref()],
        bump = round.bump,
        constraint = round.player == player.key() @ ErrorCodes::InvalidPlayer,
        constraint = round.status == GameStatus::AcceptingBets @ ErrorCodes::RoundNotAcceptingBets  
    )]
    pub round: Account<'info, RoundState>,

    #[account(
        init, 
        payer = player,
        space = 8 + BetState::INIT_SPACE,
        seeds = [b"bet", payer.key().as_ref(), round.key().as_ref()],
        bump
    )]
    pub bets: Account<'info, BetState>,

    #[account(
        seeds = [b"game", authority.key().as_ref()],
        bump = game.bump,
        constraint = !game.is_paused @ ErrorCodes::GamePaused
    )]
    pub game: Account<'info, GameState>,

    #[account(
        mut,
        seeds = [b"house_vault", game.key().as_ref()],
        bump = game.house_vault_bump
    )]
    pub house_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn place_bet(&mut self, bets: Vec<Bet>, bumps: &PlaceBetBumps) -> Result<()> {

        // calculate total amount
        let mut total_bet_amount: u64 = 0;
        for bet in &bets {
            // validate bet amounts
            require!(bet.amount >= self.game.min_bet, ErrorCodes::BetTooSmall);
            require!(bet.amount <= self.game.max_bet, ErrorCodes::BetTooLarge);

            total_bet_amount += bet.amount;
        }

        let clock = Clock::get()?;
        self.bets.set_inner(BetState { 
            round: self.round.key(), 
            bets, 
            timestamp: clock.unix_timestamp, 
            bump: bumps.bets 
        });
        
        // update the round state
        self.round.total_wagered += total_bet_amount;
        
        transfer(
            self.system_program.to_account_info(), 
            self.player.to_account_info(), 
            self.house_vault.to_account_info(), 
            self.round.total_wagered, 
            None
        )
    }
}
