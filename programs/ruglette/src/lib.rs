#![allow(unexpected_cfgs)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod types;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use types::*;
pub use utils::*;

declare_id!("GDHvCgbLXGheTMSDh7byyEPRLWhGJPSXxeCHKkd3JuSn");

#[program]
pub mod ruglette {
    use super::*;

    pub fn initialize_game(
        ctx: Context<InitializeGame>,
        min_bet: u64,
        max_bet: u64,
        house_fee_basis_points: u16,
        is_paused: bool,
    ) -> Result<()> {
        ctx.accounts.initialize_game(
            min_bet,
            max_bet,
            house_fee_basis_points,
            is_paused,
            &ctx.bumps,
        )
    }

    pub fn initialize_round(ctx: Context<InitializeRound>, start_time: i64) -> Result<()> {
        ctx.accounts.initialize_round(start_time, &ctx.bumps)
    }

    pub fn place_bet(ctx: Context<PlaceBet>, bets: Vec<Bet>) -> Result<()> {
        ctx.accounts.place_bet(bets, &ctx.bumps)
    }

    pub fn wheel_spin(ctx: Context<WheelSpin>, randomness_account: Pubkey) -> Result<()> {
        ctx.accounts.wheel_spin(randomness_account)
    }

    pub fn verify_randomness(ctx: Context<VerifyRandomness>) -> Result<()> {
        ctx.accounts.verify_randomness()
    }

    pub fn settle_bets(ctx: Context<SettleBets>) -> Result<()> {
        ctx.accounts.settle_bets()
    }
}
