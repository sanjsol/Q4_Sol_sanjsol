use anchor_lang::prelude::*;

mod state;
mod instructions;

pub use instructions::*;

declare_id!("4ivzaX2i3iF5yESaAK83RQR9fZ5mdbxESjVA4NMqscuY");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.initialize(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;

        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()?;

        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()?;

        Ok(())
    }
}
