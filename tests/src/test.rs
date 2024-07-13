use std::str::FromStr;

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
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::TestSetup;

#[allow(dead_code)]
const WIF_TOKEN_ADDRESS: &str = "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm";

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
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}
