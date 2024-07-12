use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
        signer::Signer,
    },
    Client, Cluster,
};
use anchor_lang::system_program;

use crate::PROGRAM_ID;

#[test]
fn test_initialize_market() {
    let program_id = PROGRAM_ID;
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    // BONK
    let token_account = Pubkey::from_str("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263").unwrap();

    let (market_pda, _bump) =
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
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}
