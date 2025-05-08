#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

use crate::error::ErrorCodes;
use crate::{transfer, Bet, BetState, GameState, GameStatus, RoundState};

#[derive(Accounts)]
pub struct SettleBets<'info> {
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
        mut,
        seeds = [b"bet", payer.key().as_ref(), round.key().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, BetState>,

    #[account(
        seeds = [b"game", authority.key().as_ref()],
        bump = game.bump,
        constraint = !game.is_paused @ ErrorCodes::GamePaused
    )]
    pub game: Account<'info, GameState>,

    #[account(
        mut,
        seeds = [b"house_vault", game.key().as_ref()],
        bump
    )]
    pub house_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> SettleBets<'info> {
    pub fn settle_bets(&mut self) {

    }
}