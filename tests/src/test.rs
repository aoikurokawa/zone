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
use sequential_test::sequential;
use solana_program::native_token::LAMPORTS_PER_SOL;

use crate::TestSetup;

#[allow(dead_code)]
const WIF_TOKEN_ADDRESS: &str = "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm";

#[allow(dead_code)]
const BONK_TOKEN_ADDRESS: &str = "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263";

#[allow(dead_code)]
const WATER_TOKEN_ADDRESS: &str = "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263";

#[allow(dead_code)]
const CATWIFHAT_TOKEN_ADDRESS: &str = "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263";

#[test]
fn test_initialize() {
    let setup = TestSetup::new();

    // Success pattern
    let success_res = setup.initialize(0);
    assert!(success_res.is_ok());

    // Fail pattern (Same vault)
    let fail_res = setup.initialize(0);
    assert!(fail_res.is_err());
}

#[test]
fn test_initialize_market() {
    // WIF
    let token_account = Pubkey::from_str(WIF_TOKEN_ADDRESS).unwrap();
    let setup = TestSetup::new();

    let _ = setup.initialize(1);

    // Success pattern
    let success_res = setup.initialize_market(token_account);
    assert!(success_res.is_ok());

    // Fail pattern (Already initialized)
    let fail_res = setup.initialize_market(token_account);
    assert!(fail_res.is_err());
}

#[test]
fn test_start_market() {
    // BONK
    let token_account = Pubkey::from_str(BONK_TOKEN_ADDRESS).unwrap();
    let setup = TestSetup::new();
    let end = Utc::now() + chrono::Duration::days(1);

    let _ = setup.initialize(2);
    let _ = setup.initialize_market(token_account);

    // Success pattern
    let success_res = setup.start_market(token_account, end);
    match success_res {
        Ok(sig) => {
            println!("{sig}");
        }
        Err(e) => {
            println!("{e:?}");
        }
    }
    // assert!(success_res.is_ok());


    // Fail pattern (Already started the market)
    let fail_res = setup.start_market(token_account, end);
    assert!(fail_res.is_err());
}

#[test]
fn test_create_prediction() {
    // WATER
    let token_account = Pubkey::from_str(WATER_TOKEN_ADDRESS).unwrap();
    let setup = TestSetup::new();
    let vault_num = 3;
    let end = Utc::now() + chrono::Duration::days(1);

    let _ = setup.initialize(vault_num);
    let _ = setup.initialize_market(token_account);
    let _ = setup.start_market(token_account, end);

    // Success pattern
    let success_res = setup.create_prediction(vault_num, token_account);
    assert!(success_res.is_ok());

    // Fail pattern (Already created)
    let fail_res = setup.create_prediction(vault_num, token_account);
    assert!(fail_res.is_err());
}
