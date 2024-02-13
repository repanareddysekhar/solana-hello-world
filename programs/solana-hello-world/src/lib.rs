use anchor_lang::prelude::*;

declare_id!("5RToMja5PmpcFjLvtZrtxJozY5xoA3AK6Qdj3aj62wXU");

#[program]
pub mod solana_hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
