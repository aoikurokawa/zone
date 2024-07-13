use std::{str::FromStr, thread::sleep};

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        signer::Signer,
    },
    Client, Cluster,
};
use anchor_lang::system_program;
use chrono::Utc;
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::TestSetup;

#[allow(dead_code)]
const WIF_TOKEN_ADDRESS: &str = "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm";
#[allow(dead_code)]
const BONK_TOKEN_ADDRESS: &str = "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263";

#[test]
fn test_initialize() {
    let setup = TestSetup::new();

    let sig = setup
        .program
        .request()
        .accounts(zone::accounts::Initialize {
            vault: setup.get_vault_pda(),
            authority: setup.payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(zone::instruction::Initialize {
            amount: 100 * LAMPORTS_PER_SOL,
        })
        .send();

    assert!(sig.is_ok());
}

#[test]
fn test_fail_initialize() {
    let setup = TestSetup::new();

    let sig = setup
        .program
        .request()
        .accounts(zone::accounts::Initialize {
            vault: setup.get_vault_pda(),
            authority: setup.payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(zone::instruction::Initialize {
            amount: 100 * LAMPORTS_PER_SOL,
        })
        .send();

    assert!(sig.is_err());
}

#[test]
fn test_initialize_market() {
    let setup = TestSetup::new();

    // WIF
    let token_account = Pubkey::from_str(WIF_TOKEN_ADDRESS).unwrap();

    let sig = setup
        .program
        .request()
        .accounts(zone::accounts::InitializeMarket {
            market: setup.get_market_pda(token_account),
            authority: setup.payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(zone::instruction::InitializeMarket {
            token_account,
            payout_multiplier: 200,
        })
        .send();

    assert!(sig.is_ok());
}

#[test]
fn test_fail_initialize_market() {
    let setup = TestSetup::new();

    // WIF
    let token_account = Pubkey::from_str(WIF_TOKEN_ADDRESS).unwrap();

    let sig = setup
        .program
        .request()
        .accounts(zone::accounts::InitializeMarket {
            market: setup.get_market_pda(token_account),
            authority: setup.payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(zone::instruction::InitializeMarket {
            token_account,
            payout_multiplier: 200,
        })
        .send();

    assert!(sig.is_err());
}

#[test]
fn test_start_market() {
    let setup = TestSetup::new();

    // BONK
    let token_account = Pubkey::from_str(BONK_TOKEN_ADDRESS).unwrap();

    let sig = setup
        .program
        .request()
        .accounts(zone::accounts::InitializeMarket {
            market: setup.get_market_pda(token_account),
            authority: setup.payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(zone::instruction::InitializeMarket {
            token_account,
            payout_multiplier: 200,
        })
        .send();

    assert!(sig.is_ok());

    let end = Utc::now() + chrono::Duration::days(1);

    let sig = setup
        .program
        .request()
        .accounts(zone::accounts::StartMarket {
            market: setup.get_market_pda(token_account),
        })
        .args(zone::instruction::StartMarket {
            end: end.timestamp(),
        })
        .send();

    assert!(sig.is_ok());
}

#[test]
fn test_fail_start_market() {
    let setup = TestSetup::new();

    // BONK
    let token_account = Pubkey::from_str(BONK_TOKEN_ADDRESS).unwrap();

    let end = Utc::now() + chrono::Duration::days(1);

    let sig = setup
        .program
        .request()
        .accounts(zone::accounts::StartMarket {
            market: setup.get_market_pda(token_account),
        })
        .args(zone::instruction::StartMarket {
            end: end.timestamp(),
        })
        .send();

    assert!(sig.is_err());
}

#[test]
fn test_create_prediction() {
    let setup = TestSetup::new();

    // WATER
    let token_account = Pubkey::from_str("B6h248NJkAcBAkaCnji889a26tCiGXGN8cxhEJ4dX391").unwrap();

    let tx = setup
        .program
        .request()
        .accounts(zone::accounts::InitializeMarket {
            market: setup.get_market_pda(token_account),
            authority: setup.payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(zone::instruction::InitializeMarket {
            token_account,
            payout_multiplier: 200,
        })
        .send();

    assert!(tx.is_ok());

    let end = Utc::now() + chrono::Duration::days(1);

    let tx = setup
        .program
        .request()
        .accounts(zone::accounts::StartMarket {
            market: setup.get_market_pda(token_account),
        })
        .args(zone::instruction::StartMarket {
            end: end.timestamp(),
        })
        .send();

    assert!(tx.is_ok());

    let tx = setup
        .program
        .request()
        .accounts(zone::accounts::CreatePrediction {
            prediction: setup.get_prediction_pda(token_account),
            user: setup.payer.pubkey(),
            market: setup.get_market_pda(token_account),
            system_program: system_program::ID,
            vault: setup.get_vault_pda(),
        })
        .args(zone::instruction::CreatePrediction {
            prediction: true,
            amount: 100,
            current_price: 100_000,
        })
        .send();

    assert!(tx.is_ok());
}

#[test]
fn test_fail_create_prediction() {
    let setup = TestSetup::new();

    // WATER
    let token_account = Pubkey::from_str("B6h248NJkAcBAkaCnji889a26tCiGXGN8cxhEJ4dX391").unwrap();

    let tx = setup
        .program
        .request()
        .accounts(zone::accounts::CreatePrediction {
            prediction: setup.get_prediction_pda(token_account),
            user: setup.payer.pubkey(),
            market: setup.get_market_pda(token_account),
            system_program: system_program::ID,
            vault: setup.get_vault_pda(),
        })
        .args(zone::instruction::CreatePrediction {
            prediction: true,
            amount: 100,
            current_price: 100_000,
        })
        .send();

    assert!(tx.is_err());
}

#[test]
fn test_settle_prediction() {
    let setup = TestSetup::new();

    // CATWIFHAT
    let token_account = Pubkey::from_str("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB265").unwrap();

    let tx = setup
        .program
        .request()
        .accounts(zone::accounts::InitializeMarket {
            market: setup.get_market_pda(token_account),
            authority: setup.payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(zone::instruction::InitializeMarket {
            token_account,
            payout_multiplier: 200,
        })
        .send();

    assert!(tx.is_ok());

    let end = Utc::now() + chrono::Duration::microseconds(1);

    let tx = setup
        .program
        .request()
        .accounts(zone::accounts::StartMarket {
            market: setup.get_market_pda(token_account),
        })
        .args(zone::instruction::StartMarket {
            end: end.timestamp(),
        })
        .send();

    assert!(tx.is_ok());

    sleep(std::time::Duration::new(5, 0));

    let tx = setup
        .program
        .request()
        .accounts(zone::accounts::CreatePrediction {
            prediction: setup.get_prediction_pda(token_account),
            user: setup.payer.pubkey(),
            market: setup.get_market_pda(token_account),
            system_program: system_program::ID,
            vault: setup.get_vault_pda(),
        })
        .args(zone::instruction::CreatePrediction {
            prediction: true,
            amount: 10 * LAMPORTS_PER_SOL,
            current_price: 100_000,
        })
        .send();

    assert!(tx.is_ok());

    let tx = setup
        .program
        .request()
        .accounts(zone::accounts::SettlePrediction {
            prediction: setup.get_prediction_pda(token_account),
            user: setup.payer.pubkey(),
            market: setup.get_market_pda(token_account),
            system_program: system_program::ID,
            vault: setup.get_vault_pda(),
        })
        .args(zone::instruction::SettlePrediction {
            actual_price: 20_000,
        })
        .send();

    assert!(tx.is_ok());
}

#[test]
fn test_fail_settle_prediction() {
    let setup = TestSetup::new();

    // CATWIFHAT
    let token_account = Pubkey::from_str("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB265").unwrap();

    let tx = setup
        .program
        .request()
        .accounts(zone::accounts::SettlePrediction {
            prediction: setup.get_prediction_pda(token_account),
            user: setup.payer.pubkey(),
            market: setup.get_market_pda(token_account),
            system_program: system_program::ID,
            vault: setup.get_vault_pda(),
        })
        .args(zone::instruction::SettlePrediction {
            actual_price: 20_000,
        })
        .send();

    assert!(tx.is_ok());
}
