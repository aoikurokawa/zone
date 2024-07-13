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
