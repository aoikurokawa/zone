use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

declare_id!("FuF87VDgECo9tETuve2obCmt38r7EZM2HC34D8ePo6fz");

pub mod constants {
    pub const VAULT_SEED: &[u8] = b"vault";
    pub const STAKE_INFO_SEED: &[u8] = b"stake_info";
    pub const TOKEN_SEED: &[u8] = b"token";
}

#[program]
pub mod zone {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let stake_info = &mut ctx.accounts.stake_info;

        if stake_info.is_staked {
            return Err(ErrorCode::IsStaked.into());
        }

        if amount <= 0 {
            return Err(ErrorCode::NoTokens.into());
        }

        let clock = Clock::get()?;

        stake_info.stake_at_slot = clock.slot;
        stake_info.is_staked = true;

        let stake_amount = if let Some(amount) =
            amount.checked_mul(10u64.pow(ctx.accounts.mint.decimals as u32))
        {
            amount
        } else {
            0
        };

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user_token_account.to_account_info(),
                    to: ctx.accounts.stake_account.to_account_info(),
                    authority: ctx.accounts.signer.to_account_info(),
                },
            ),
            stake_amount,
        )?;

        Ok(())
    }

    pub fn initialize_market(ctx: Context<InitializeMarket>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        market.authority = ctx.accounts.authority.key();

        Ok(())
    }

    pub fn destake(ctx: Context<DeStake>) -> Result<()> {
        let stake_info = &mut ctx.accounts.stake_info;

        if !stake_info.is_staked {
            return Err(ErrorCode::NotStaked.into());
        }

        let clock = Clock::get()?;
        let slot_passed = clock.slot - stake_info.stake_at_slot;

        let stake_amount = ctx.accounts.stake_account.amount;

        let reward = if let Some(num) =
            slot_passed.checked_mul(10u64.pow(ctx.accounts.mint.decimals as u32))
        {
            num
        } else {
            0
        };

        let bump = ctx.bumps.token_vault_account;
        let signer: &[&[&[u8]]] = &[&[constants::VAULT_SEED, &[bump]]];

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.token_vault_account.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.token_vault_account.to_account_info(),
                },
                signer,
            ),
            reward,
        )?;

        let staker = ctx.accounts.signer.key();
        let bump = ctx.bumps.stake_account;
        let signer: &[&[&[u8]]] = &[&[constants::TOKEN_SEED, staker.as_ref(), &[bump]]];

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.stake_account.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.stake_account.to_account_info(),
                },
                signer,
            ),
            stake_amount,
        )?;

        stake_info.is_staked = false;
        stake_info.stake_at_slot = clock.slot;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        seeds = [constants::VAULT_SEED],
        bump,
        payer = signer,
        token::mint = mint,
        token::authority = token_vault_account,
    )]
    pub token_vault_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeMarket<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 1)]
    market: Account<'info, Market>,

    #[account(mut)]
    authority: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [constants::STAKE_INFO_SEED, signer.key.as_ref()],
        bump,
        payer = signer,
        space = 8 + std::mem::size_of::<StakeInfo>(),
    )]
    pub stake_info: Account<'info, StakeInfo>,

    #[account(
        init_if_needed,
        seeds = [constants::TOKEN_SEED, signer.key.as_ref()],
        bump,
        payer = signer,
        token::mint = mint,
        token::authority = stake_account,
    )]
    pub stake_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeStake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::VAULT_SEED],
        bump
    )]
    pub token_vault_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [constants::STAKE_INFO_SEED, signer.key.as_ref()],
        bump,
    )]
    pub stake_info: Account<'info, StakeInfo>,

    #[account(
        mut,
        seeds = [constants::TOKEN_SEED, signer.key.as_ref()],
        bump
    )]
    pub stake_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[account]
struct Market {
    authority: Pubkey,
}

#[account]
pub struct StakeInfo {
    // 8
    pub stake_at_slot: u64,

    // 1
    pub is_staked: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Tokens are already staked")]
    IsStaked,

    #[msg("Tokens not staked")]
    NotStaked,

    #[msg("No Tokens to stake")]
    NoTokens,
}
