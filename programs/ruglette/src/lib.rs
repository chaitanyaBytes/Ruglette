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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
