use anchor_lang::prelude::*;

declare_id!("2BqXsVFG5Woo6VVg6pK4RM7g6W7YZwCSM9wYou8kzu6F");

pub mod constants {
    pub const VAULT_SEED: &[u8] = b"vault";
    pub const MARKET_SEED: &[u8] = b"market";
    pub const PREDICTION_SEED: &[u8] = b"prediction";
}

#[program]
pub mod zone {
    use anchor_lang::{
        context::{Context, CpiContext},
        system_program, Key, ToAccountInfo,
    };
    use solana_program::{clock::Clock, msg, pubkey::Pubkey, sysvar::Sysvar};

    use crate::{
        CreatePrediction, Initialize, InitializeMarket, SettlePrediction, StartMarket,
        ZoneErrorCode,
    };

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> anchor_lang::Result<()> {
        // deposit funds to vault
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.authority.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, amount)?;

        Ok(())
    }

    pub fn initialize_market(
        ctx: Context<InitializeMarket>,
        token_account: Pubkey,
        payout_multiplier: u64,
    ) -> anchor_lang::Result<()> {
        msg!("Initialize market");

        let market = &mut ctx.accounts.market;

        market.authority = ctx.accounts.authority.key();
        market.token_account = token_account;
        market.payout_multiplier = payout_multiplier;

        Ok(())
    }

    pub fn start_market(ctx: Context<StartMarket>, end: i64) -> anchor_lang::Result<()> {
        msg!("Start market");

        let market = &mut ctx.accounts.market;
        let clock = Clock::get()?;

        if market.start != 0 {
            return Err(ZoneErrorCode::AlreadyStarted.into());
        }

        clock.unix_timestamp;
        market.end = end;

        Ok(())
    }

    pub fn create_prediction(
        ctx: Context<CreatePrediction>,
        prediction: bool,
        amount: u64,
        current_price: u64,
    ) -> anchor_lang::Result<()> {
        let market = &mut ctx.accounts.market;
        let clock = Clock::get()?;

        if market.start > clock.unix_timestamp {
            return Err(ZoneErrorCode::NotStarted.into());
        }

        let new_prediction = &mut ctx.accounts.prediction;
        new_prediction.user = ctx.accounts.user.key();
        new_prediction.market = ctx.accounts.market.key();
        new_prediction.prediction = prediction;
        new_prediction.amount = amount;
        new_prediction.market_price = current_price;

        // Transfer the amount to the market escrow account
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, amount)?;

        Ok(())
    }

    pub fn settle_prediction(
        ctx: Context<SettlePrediction>,
        actual_price: u64,
    ) -> anchor_lang::Result<()> {
        let market = &mut ctx.accounts.market;
        let clock = Clock::get()?;

        if market.end > clock.unix_timestamp {
            return Err(ZoneErrorCode::NotFinished.into());
        } else {
            let prediction = &mut ctx.accounts.prediction;
            let reward = (prediction.amount * market.payout_multiplier) / 100;

            if (actual_price > prediction.market_price && prediction.prediction)
                || (actual_price < prediction.market_price && !prediction.prediction)
            {
                **ctx
                    .accounts
                    .vault
                    .to_account_info()
                    .try_borrow_mut_lamports()? -= reward;
                **ctx
                    .accounts
                    .user
                    .to_account_info()
                    .try_borrow_mut_lamports()? += reward;
            } else {
                let cpi_context = CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.user.to_account_info(),
                        to: ctx.accounts.vault.to_account_info(),
                    },
                );
                system_program::transfer(cpi_context, reward)?;
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init_if_needed, seeds = [crate::constants::VAULT_SEED], bump, payer = authority, space = 8 + 1)]
    vault: Account<'info, Vault>,

    #[account(mut)]
    authority: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(token_account: Pubkey)]
pub struct InitializeMarket<'info> {
    #[account(
        init,
        seeds = [crate::constants::MARKET_SEED, token_account.as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<Market>())
    ]
    market: Account<'info, Market>,

    #[account(mut)]
    authority: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StartMarket<'info> {
    #[account(mut)]
    market: Account<'info, Market>,
}

#[derive(Accounts)]
pub struct CreatePrediction<'info> {
    #[account(mut)]
    vault: Account<'info, Vault>,

    #[account(
        init,
        seeds = [crate::constants::PREDICTION_SEED, market.key().as_ref(), user.key.as_ref()],
        bump,
        payer = user,
        space = 8 + std::mem::size_of::<Prediction>())
    ]
    prediction: Account<'info, Prediction>,

    #[account(mut)]
    user: Signer<'info>,

    #[account(mut)]
    market: Account<'info, Market>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SettlePrediction<'info> {
    #[account(mut)]
    vault: Account<'info, Vault>,

    #[account(mut)]
    prediction: Account<'info, Prediction>,

    #[account(mut)]
    user: Signer<'info>,

    #[account(mut)]
    market: Account<'info, Market>,

    system_program: Program<'info, System>,
}

#[account]
pub struct Vault {}

#[account]
pub struct Market {
    authority: Pubkey,
    token_account: Pubkey,
    start: i64,
    end: i64,
    payout_multiplier: u64, // Multiplier for payout (e.g., 200 for 2x)
}

#[account]
pub struct Prediction {
    user: Pubkey,
    market: Pubkey,
    prediction: bool,  // True for higher, False for lower
    market_price: u64, // Market price at prediction time
    amount: u64,       // Amount wagered
}

#[error_code]
pub enum ZoneErrorCode {
    #[msg("Market has already started")]
    AlreadyStarted,

    #[msg("Market has not started yet")]
    NotStarted,

    #[msg("Market has not finished yet")]
    NotFinished,

    #[msg("Not enough SOL")]
    NotEnoughSol,
}
