use anchor_lang::prelude::*;
use switchboard_on_demand::accounts::RandomnessAccountData;

use crate::error::ErrorCodes;
use crate::{GameStatus, RoundState};

#[derive(Accounts)]
pub struct VerifyRandomness<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    pub authority: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"round", player.key().as_ref(), round.start_time.to_le_bytes().as_ref()],
        bump = round.bump,
        constraint = round.player == player.key() @ ErrorCodes::InvalidPlayer,
        constraint = round.status == GameStatus::WaitingForVRF @ ErrorCodes::NotWaitingForRandomNumber  
    )]
    pub round: Account<'info, RoundState>,

    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> VerifyRandomness<'info> {
    pub fn verify_randomness(&mut self) -> Result<()> {
        let clock: Clock = Clock::get()?;

        // Verify that the provided randomness account matches the stored one
        if self.randomness_account_data.key() != self.round.vrf_account {
            return Err(ErrorCodes::InvalidRandomnessAccount.into());
        }

        // call the switchboard on-demand parse function to get the randomness data
        let randomness_data =
            RandomnessAccountData::parse(self.randomness_account_data.data.borrow())
                .unwrap();

        if randomness_data.seed_slot != self.round.commit_slot {
            return Err(ErrorCodes::RandomnessExpired.into());
        }

        // call the switchboard on-demand get_value function to get the revealed random value
        let revealed_random_value = randomness_data
            .get_value(&clock)
            .map_err(|_| ErrorCodes::RandomnessNotResolved)?;

        // Use the revealed random value to determine the flip results
        let randomness_result = revealed_random_value[0] % 37;
        
        // Update and log the result
        self.round.result_buffer = revealed_random_value;
        self.round.winning_number = Some(randomness_result);
        self.round.status = GameStatus::ResultReady;
        
        Ok(())
    }
}
