use anchor_lang::prelude::*;

use crate::state::GameState;

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + GameState::INIT_SPACE,
        seeds = [b"game", authority.key().as_ref()],
        bump
    )]
    pub game: Account<'info, GameState>,

    #[account(
        mut,
        seeds = [b"house_vault", game.to_account_info().key().as_ref()],
        bump
    )]
    pub house_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeGame<'info> {
    pub fn initialize_game(
        &mut self,
        min_bet: u64,
        max_bet: u64,
        house_fee_basis_points: u16,
        is_paused: bool,
        bumps: &InitializeGameBumps,
    ) -> Result<()> {
        self.game.set_inner(GameState {
            authority: self.authority.key(),
            min_bet,
            max_bet,
            house_fee_basis_points,
            is_paused,
            bump: bumps.game,
            house_vault_bump: bumps.house_vault,
        });

        Ok(())
    }
}
