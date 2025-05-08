pub use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct WheelSpin<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
}
