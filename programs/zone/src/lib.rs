use anchor_lang::prelude::*;

declare_id!("FuF87VDgECo9tETuve2obCmt38r7EZM2HC34D8ePo6fz");

#[program]
pub mod zone {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("counter Account Created");
        msg!("Current Count: {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {:?}", counter);

        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. Current count: {:?}", counter.count);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts, Debug)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,

    pub user: Signer<'info>,
}

#[derive(Debug)]
#[account]
pub struct Counter {
    count: u64,
}
