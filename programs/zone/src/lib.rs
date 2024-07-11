use anchor_lang::prelude::*;

declare_id!("7UyLYeoNwWeh3LgMUnWFKPc1Ebwr8Afzsz8hVjgavoRa");

pub mod constants {
    pub const VAULT_SEED: &[u8] = b"vault";
    pub const STAKE_INFO_SEED: &[u8] = b"stake_info";
    pub const TOKEN_SEED: &[u8] = b"token";
}

#[program]
pub mod zone {
    use anchor_lang::{context::Context, Key, ToAccountInfo};
    use solana_program::{clock::Clock, pubkey::Pubkey, sysvar::Sysvar};

    use crate::{
        CreatePrediction, Initialize, InitializeMarket, SettlePrediction, StartMarket,
        ZoneErrorCode,
    };

    pub fn initialize(_ctx: Context<Initialize>) -> anchor_lang::Result<()> {
        Ok(())
    }

    pub fn initialize_market(
        ctx: Context<InitializeMarket>,
        token_account: Pubkey,
        payout_multiplier: u64,
    ) -> anchor_lang::Result<()> {
        let market = &mut ctx.accounts.market;

        market.authority = ctx.accounts.authority.key();
        market.token_account = token_account;
        market.payout_multiplier = payout_multiplier;

        Ok(())
    }

    pub fn start_market(ctx: Context<StartMarket>, end: i64) -> anchor_lang::Result<()> {
        let market = &mut ctx.accounts.market;
        let clock = Clock::get()?;

        market.start = clock.unix_timestamp;
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

        if market.start < clock.unix_timestamp {
            return Err(ZoneErrorCode::NotStarted.into());
        }

        let new_prediction = &mut ctx.accounts.prediction;
        new_prediction.user = ctx.accounts.user.key();
        new_prediction.market = ctx.accounts.market.key();
        new_prediction.prediction = prediction;
        new_prediction.amount = amount;
        new_prediction.market_price = current_price;

        let transfer_sol = anchor_lang::solana_program::system_instruction::transfer(
            &new_prediction.user,
            &new_prediction.market,
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_sol,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.market.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn settle_prediction(
        ctx: Context<SettlePrediction>,
        actual_price: u64,
    ) -> anchor_lang::Result<()> {
        let market = &mut ctx.accounts.market;
        let clock = Clock::get()?;

        if market.end < clock.unix_timestamp {
            return Err(ZoneErrorCode::NotFinished.into());
        } else {
            let prediction = &mut ctx.accounts.prediction;
            let reward = (prediction.amount * market.payout_multiplier) / 100;

            let (from, to) = if (actual_price > prediction.market_price && prediction.prediction)
                || (actual_price < prediction.market_price && !prediction.prediction)
            {
                (prediction.market, prediction.user)
            } else {
                (prediction.user, prediction.market)
            };

            let transfer_sol =
                anchor_lang::solana_program::system_instruction::transfer(&from, &to, reward);

            anchor_lang::solana_program::program::invoke(
                &transfer_sol,
                &[
                    ctx.accounts.user.to_account_info(),
                    ctx.accounts.market.to_account_info(),
                ],
            )?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct InitializeMarket<'info> {
    #[account(init, payer = authority, space = 8 + std::mem::size_of::<Market>())]
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
    #[account(init_if_needed, payer = user, space = 8 + std::mem::size_of::<Prediction>())]
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
    prediction: Account<'info, Prediction>,

    #[account(mut)]
    user: Signer<'info>,

    #[account(mut)]
    market: Account<'info, Market>,
}

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
    #[msg("Tokens are already staked")]
    IsStaked,

    #[msg("Tokens not staked")]
    NotStaked,

    #[msg("No Tokens to stake")]
    NoTokens,

    #[msg("Market has not started yet")]
    NotStarted,

    #[msg("Market has not finished yet")]
    NotFinished,
}
