use std::{str::FromStr, thread::sleep};

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
        signer::Signer,
    },
    Client, ClientError, Cluster,
};
use anchor_lang::system_program;
use chrono::Utc;

use crate::PROGRAM_ID;

#[test]
fn test_settle_prediction() {
    let program_id = PROGRAM_ID;
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    // CATWIFHAT
    let token_account = Pubkey::from_str("7atgF8KQo4wJrD5ATGX7t1V2zVvykPJbFfNeVf1icFv7").unwrap();

    let (market_pda, market_bump) =
        Pubkey::find_program_address(&[b"market", token_account.as_ref()], &program_id);

    let tx = program
        .request()
        .accounts(zone::accounts::InitializeMarket {
            market: market_pda,
            authority: payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(zone::instruction::InitializeMarket {
            token_account,
            payout_multiplier: 200,
        })
        .send();

    assert!(tx.is_ok());

    let end = Utc::now() + chrono::Duration::microseconds(1);

    let tx = program
        .request()
        .accounts(zone::accounts::StartMarket { market: market_pda })
        .args(zone::instruction::StartMarket {
            end: end.timestamp(),
        })
        .send();

    assert!(tx.is_ok());

    sleep(std::time::Duration::new(5, 0));

    let (prediction_pda, _bump) = Pubkey::find_program_address(
        &[b"prediction", market_pda.as_ref(), payer.pubkey().as_ref()],
        &program_id,
    );
    let (vault_pda, _bump) = Pubkey::find_program_address(&[b"vault"], &program_id);

    match program
        .request()
        .accounts(zone::accounts::CreatePrediction {
            prediction: prediction_pda,
            user: payer.pubkey(),
            market: market_pda,
            system_program: system_program::ID,
            vault: vault_pda,
        })
        .args(zone::instruction::CreatePrediction {
            prediction: true,
            amount: 100,
            current_price: 100_000,
        })
        .send()
    {
        Ok(sig) => {
            println!("{sig}");
        }
        Err(e) => {
            println!("{e:?}");
        }
    }

    // assert!(tx.is_ok());

    match program
        .request()
        .accounts(zone::accounts::SettlePrediction {
            prediction: prediction_pda,
            user: payer.pubkey(),
            market: market_pda,
            system_program: system_program::ID,
            vault: vault_pda,
        })
        .args(zone::instruction::SettlePrediction {
            actual_price: 200_000,
            token_account,
            bump: market_bump,
        })
        .send()
    {
        Ok(sig) => {
            println!("{sig}");
        }
        Err(e) => {
            println!("{e:?}");
        }
    }

    // assert!(tx.is_ok());
}
