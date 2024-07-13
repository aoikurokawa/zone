use std::{str::FromStr, sync::Arc};

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::{read_keypair_file, Keypair},
        signer::Signer,
    },
    Client, Cluster, Program,
};
use solana_program::pubkey::Pubkey;

#[allow(unused_imports)]
mod test;
#[allow(unused_imports)]
mod test_settle_prediction;

pub const PROGRAM_ID: &str = "7UyLYeoNwWeh3LgMUnWFKPc1Ebwr8Afzsz8hVjgavoRa";

pub struct TestSetup {
    pub payer: Arc<Keypair>,
    pub client: Client<Arc<Keypair>>,
    pub program_id: Pubkey,
    pub program: Program<Arc<Keypair>>,
}

impl TestSetup {
    pub fn new() -> Self {
        let program_id = PROGRAM_ID;
        let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
        let payer = read_keypair_file(&anchor_wallet).unwrap();
        let payer = Arc::new(payer);

        let client = Client::new_with_options(
            Cluster::Localnet,
            payer.clone(),
            CommitmentConfig::confirmed(),
        );
        let program_id = Pubkey::from_str(program_id).unwrap();
        let program = client.program(program_id).unwrap();

        Self {
            payer,
            client,
            program_id,
            program,
        }
    }

    pub fn get_vault_pda(&self) -> Pubkey {
        let (vault_pda, _bump) = Pubkey::find_program_address(&[b"vault"], &self.program_id);

        vault_pda
    }

    pub fn get_market_pda(&self, token_account: Pubkey) -> Pubkey {
        let (market_pda, _bump) =
            Pubkey::find_program_address(&[b"market", token_account.as_ref()], &self.program_id);

        market_pda
    }

    pub fn get_prediction_pda(&self, token_account: Pubkey) -> Pubkey {
        let market_pda = self.get_market_pda(token_account);

        let (prediction_pda, _bump) = Pubkey::find_program_address(
            &[
                b"prediction",
                market_pda.as_ref(),
                self.payer.pubkey().as_ref(),
            ],
            &self.program_id,
        );

        prediction_pda
    }
}
