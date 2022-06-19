use anchor_lang::prelude::*;

declare_id!("B2U6Lz4j6guHFf5DmXRgy5SXkuZnucfhp74J1Vm5YRQv");

#[program]
pub mod solana_svelte_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
