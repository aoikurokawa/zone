use anchor_lang::prelude::*;

declare_id!("FuF87VDgECo9tETuve2obCmt38r7EZM2HC34D8ePo6fz");

#[program]
pub mod zone {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
