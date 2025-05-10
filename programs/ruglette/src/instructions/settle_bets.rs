#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

use crate::error::ErrorCodes;
use crate::{transfer, Bet, BetState, BetType, Color, GameState, GameStatus, RoundState};

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
        constraint = round.status == GameStatus::ResultReady @ ErrorCodes::ResultNotReady,
        close = player
    )]
    pub round: Account<'info, RoundState>,

    #[account(
        mut,
        seeds = [b"bet", player.key().as_ref(), round.key().as_ref()],
        bump = bets.bump,
        close = player
    )]
    pub bets: Box<Account<'info, BetState>>,

    #[account(
        seeds = [b"game", authority.key().as_ref()],
        bump = game.bump,
    )]
    pub game: Account<'info, GameState>,

    #[account(
        mut,
        seeds = [b"house_vault", game.to_account_info().key().as_ref()],
        bump = game.house_vault_bump
    )]
    pub house_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> SettleBets<'info> {
    pub fn settle_bets(&mut self) -> Result<()> {
        require!(
            self.round.status == GameStatus::ResultReady,
            ErrorCodes::ResultNotReady
        );

        let random_number = match self.round.winning_number {
            Some(number) => number,
            None => return Err(ErrorCodes::ResultNotReady.into()),
        };

        let mut payout_amount = 0;

        for bet in &self.bets.bets {
            payout_amount += calculate_payout(&bet, random_number)?;
        }

        self.round.payout_amount = Some(payout_amount);

        // handle payouts
        if payout_amount > 0 {
            let signer_seeds: &[&[&[u8]]] = &[&[
                b"house_vault",
                self.game.to_account_info().key.as_ref(),
                &[self.game.house_vault_bump],
            ]];

            let house_fee = payout_amount
                .checked_mul(
                    u64::try_from(self.game.house_fee_basis_points)
                        .map_err(|_| ProgramError::ArithmeticOverflow)?,
                )
                .ok_or(ProgramError::ArithmeticOverflow)?
                .checked_div(10_000)
                .ok_or(ProgramError::ArithmeticOverflow)?;

            transfer(
                self.system_program.to_account_info(),
                self.house_vault.to_account_info(),
                self.player.to_account_info(),
                payout_amount - house_fee,
                Some(signer_seeds),
            )?;
        }

        // Update round status
        self.round.status = GameStatus::BetsSettled;

        Ok(())
    }
}

pub fn calculate_payout(bet: &Bet, random_number: u8) -> Result<u64> {
    let won = match bet.bet_type {
        BetType::Odd => random_number > 0 && random_number % 2 == 1,
        BetType::Even => random_number > 0 && random_number % 2 == 0,
        BetType::Low => random_number >= 1 && random_number <= 18,
        BetType::High => random_number >= 19 && random_number <= 36,
        BetType::Red => {
            let color = BetType::get_color(random_number);
            color == Color::Red
        }
        BetType::Black => {
            let color = BetType::get_color(random_number);
            color == Color::Black
        }
        BetType::Straight => {
            // Single number bet
            if bet.targets.len() == 1 {
                random_number == bet.targets[0]
            } else {
                false // Invalid number of targets
            }
        }
        BetType::Split => {
            // Two adjacent numbers
            if bet.targets.len() == 2 {
                bet.targets.contains(&random_number)
            } else {
                false // Invalid number of targets
            }
        }
        BetType::Street => {
            // Three numbers in a horizontal line
            if bet.targets.len() == 3 {
                bet.targets.contains(&random_number)
            } else {
                false // Invalid number of targets
            }
        }
        BetType::Corner => {
            // Four numbers in a square
            if bet.targets.len() == 4 {
                bet.targets.contains(&random_number)
            } else {
                false // Invalid number of targets
            }
        }
        BetType::Sixline => {
            // Six numbers (two adjacent streets)
            if bet.targets.len() == 6 {
                bet.targets.contains(&random_number)
            } else {
                false // Invalid number of targets
            }
        }
        BetType::Dozen => {
            // Check which dozen was bet on and if the winning number is in that dozen
            // First dozen: 1-12, Second dozen: 13-24, Third dozen: 25-36
            if bet.targets.len() == 1 {
                match bet.targets[0] {
                    1 => random_number >= 1 && random_number <= 12,
                    2 => random_number >= 13 && random_number <= 24,
                    3 => random_number >= 25 && random_number <= 36,
                    _ => false, // Invalid dozen
                }
            } else {
                false // Invalid number of targets
            }
        }
        BetType::Column => {
            // Check which column was bet on and if the winning number is in that column
            // First column: 1,4,7,10,13,16,19,22,25,28,31,34
            // Second column: 2,5,8,11,14,17,20,23,26,29,32,35
            // Third column: 3,6,9,12,15,18,21,24,27,30,33,36
            if bet.targets.len() == 1 {
                match bet.targets[0] {
                    1 => random_number % 3 == 1,
                    2 => random_number % 3 == 2,
                    3 => random_number > 0 && random_number % 3 == 0,
                    _ => false, // Invalid column
                }
            } else {
                false // Invalid number of targets
            }
        }
    };

    if won {
        // Calculate payout: (bet amount * multiplier)
        let multiplier = u64::try_from(bet.bet_type.multiplier())
            .map_err(|_| ProgramError::ArithmeticOverflow)?;

        Ok(bet
            .amount
            .checked_mul(multiplier)
            .ok_or(ProgramError::ArithmeticOverflow)?)
    } else {
        Ok(0)
    }
}
