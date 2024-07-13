use std::{str::FromStr, sync::Arc};

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::{read_keypair_file, Keypair, Signature},
        signer::Signer,
    },
    Client, ClientError, Cluster, Program,
};
use anchor_lang::system_program;
use chrono::{DateTime, Utc};
use solana_program::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};

#[allow(unused_imports)]
mod test;

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

    pub fn get_vault_pda(&self, vault_num: u8) -> Pubkey {
        let (vault_pda, _bump) =
            Pubkey::find_program_address(&[b"vault", &[vault_num]], &self.program_id);

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

    pub fn initialize(&self, vault_num: u8) -> Result<Signature, ClientError> {
        self.program
            .request()
            .accounts(zone::accounts::Initialize {
                vault: self.get_vault_pda(vault_num),
                authority: self.payer.pubkey(),
                system_program: system_program::ID,
            })
            .args(zone::instruction::Initialize {
                amount: 100 * LAMPORTS_PER_SOL,
                vault_num,
            })
            .send()
    }

    pub fn initialize_market(&self, token_account: Pubkey) -> Result<Signature, ClientError> {
        self.program
            .request()
            .accounts(zone::accounts::InitializeMarket {
                market: self.get_market_pda(token_account),
                authority: self.payer.pubkey(),
                system_program: system_program::ID,
            })
            .args(zone::instruction::InitializeMarket {
                token_account,
                payout_multiplier: 200,
            })
            .send()
    }

    pub fn start_market(
        &self,
        token_account: Pubkey,
        end: DateTime<Utc>,
    ) -> Result<Signature, ClientError> {
        self.program
            .request()
            .accounts(zone::accounts::StartMarket {
                market: self.get_market_pda(token_account),
            })
            .args(zone::instruction::StartMarket {
                end: end.timestamp(),
            })
            .send()
    }

    pub fn create_prediction(
        &self,
        vault_num: u8,
        token_account: Pubkey,
    ) -> Result<Signature, ClientError> {
        self.program
            .request()
            .accounts(zone::accounts::CreatePrediction {
                prediction: self.get_prediction_pda(token_account),
                user: self.payer.pubkey(),
                market: self.get_market_pda(token_account),
                system_program: system_program::ID,
                vault: self.get_vault_pda(vault_num),
            })
            .args(zone::instruction::CreatePrediction {
                prediction: true,
                amount: 100,
                current_price: 100_000,
            })
            .send()
    }

    pub fn settle_prediction(
        &self,
        vault_num: u8,
        token_account: Pubkey,
    ) -> Result<Signature, ClientError> {
        self.program
            .request()
            .accounts(zone::accounts::SettlePrediction {
                prediction: self.get_prediction_pda(token_account),
                user: self.payer.pubkey(),
                market: self.get_market_pda(token_account),
                system_program: system_program::ID,
                vault: self.get_vault_pda(vault_num),
            })
            .args(zone::instruction::SettlePrediction {
                actual_price: 20_000,
            })
            .send()
    }
}

impl Default for TestSetup {
    fn default() -> Self {
        TestSetup::new()
    }
}
