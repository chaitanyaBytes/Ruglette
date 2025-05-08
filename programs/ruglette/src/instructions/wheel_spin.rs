pub use anchor_lang::prelude::*;
use switchboard_on_demand::accounts::RandomnessAccountData;

use crate::{transfer, Bet, BetState, GameState, GameStatus, RoundState};
use crate::error::ErrorCodes;

#[derive(Accounts)]
pub struct WheelSpin<'info> {
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

    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,

    pub system_program: Program<'info, System>,  
}

impl<'info> WheelSpin<'info> {
    pub fn wheel_spin(&mut self, randomness_account: Pubkey) -> Result<()> {
        let clock = Clock::get()?;

        let randomness_data =
            RandomnessAccountData::parse(self.randomness_account_data.data.borrow())
                .unwrap();
        
        if randomness_data.seed_slot != clock.slot - 1 {
            msg!("seed_slot: {}", randomness_data.seed_slot);
            msg!("slot: {}", clock.slot);
            return Err(ErrorCodes::RandomnessAlreadyRevealed.into());
        };

        // Store flip commit
        self.round.vrf_account = randomness_account;
        self.round.status = GameStatus::WaitingForVRF;
        
        // Log the result
        msg!("wheel spin initiated, randomness requested.");
        Ok(())  
  }
}
