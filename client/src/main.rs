use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, signature::read_keypair_file, signer::Signer,
    },
    Client, Cluster,
};
use anchor_lang::system_program;
use chrono::DateTime;
use clap::{Parser, Subcommand};
use solana_program::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the vault
    Initialize {
        /// SOL
        vault_amount: u64,
    },

    /// Initialize the market
    InitializeMarket {
        /// BONK: DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263
        /// MOTHER: 3S8qX1MsMqRbiwKg2cQyx7nis1oHMgaCuc9c4VfvVdPN
        token_address: String,

        /// Multiplier for payout (e.g., 200 for 2x)
        payout_multiplier: u64,
    },

    /// Start the market
    StartMarket {
        /// BONK: DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263
        /// MOTHER: 3S8qX1MsMqRbiwKg2cQyx7nis1oHMgaCuc9c4VfvVdPN
        token_address: String,

        /// 2024-06-13 13:03:00
        end: String,
    },

    /// Bet YES or NO
    Bet {
        /// BONK: DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263
        /// MOTHER: 3S8qX1MsMqRbiwKg2cQyx7nis1oHMgaCuc9c4VfvVdPN
        token_address: String,

        /// HIGH => >= 1
        /// LOW => 0
        prediction: u8,

        /// SOL
        amount: u64,

        /// Market price at prediction time
        current_price: u64,
    },
}

fn main() {
    // devnet
    let program_id = "2BqXsVFG5Woo6VVg6pK4RM7g6W7YZwCSM9wYou8kzu6F";

    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let (vault_pda, _bump) = Pubkey::find_program_address(&[b"vault"], &program_id);

    let cli = Cli::parse();
    match &cli.command {
        Commands::Initialize { vault_amount } => {
            let sig = program
                .request()
                .accounts(zone::accounts::Initialize {
                    vault: vault_pda,
                    authority: payer.pubkey(),
                    system_program: system_program::ID,
                })
                .args(zone::instruction::Initialize {
                    amount: vault_amount * LAMPORTS_PER_SOL,
                })
                .send()
                .expect("Failed to send initialize transaction");

            println!("Successfully initialized: https://solscan.io/tx/{sig}?cluster=devnet");
        }
        Commands::InitializeMarket {
            token_address,
            payout_multiplier,
        } => {
            let token_account = Pubkey::from_str(&token_address).unwrap();

            let (market_pda, _bump) =
                Pubkey::find_program_address(&[b"market", token_account.as_ref()], &program_id);

            let sig = program
                .request()
                .accounts(zone::accounts::InitializeMarket {
                    market: market_pda,
                    authority: payer.pubkey(),
                    system_program: system_program::ID,
                })
                .args(zone::instruction::InitializeMarket {
                    token_account,
                    payout_multiplier: *payout_multiplier,
                })
                .send()
                .expect("Failed to send initialize market transaction");

            println!("Successfully initialized market: https://solscan.io/tx/{sig}?cluster=devnet");
        }
        Commands::StartMarket { token_address, end } => {
            let token_account = Pubkey::from_str(&token_address).unwrap();

            let (market_pda, _bump) =
                Pubkey::find_program_address(&[b"market", token_account.as_ref()], &program_id);

            let end = format!("{end} +0000");
            let end = DateTime::parse_from_str(&end, "%Y-%m-%d %H:%M:%S %z").unwrap();
            // let end = Utc::now() + chrono::Duration::days(1);

            let sig = program
                .request()
                .accounts(zone::accounts::StartMarket { market: market_pda })
                .args(zone::instruction::StartMarket {
                    end: end.timestamp(),
                })
                .send()
                .expect("Failed to send start market transaction");

            println!("Successfully start market: https://solscan.io/tx/{sig}?cluster=devnet");
        }
        Commands::Bet {
            token_address,
            prediction,
            amount,
            current_price,
        } => {
            let token_account = Pubkey::from_str(&token_address).unwrap();
            let (market_pda, _bump) =
                Pubkey::find_program_address(&[b"market", token_account.as_ref()], &program_id);

            let (prediction_pda, _bump) = Pubkey::find_program_address(
                &[b"prediction", market_pda.as_ref(), payer.pubkey().as_ref()],
                &program_id,
            );

            let prediction = if *prediction == 0 { false } else { true };

            let sig = program
                .request()
                .accounts(zone::accounts::CreatePrediction {
                    prediction: prediction_pda,
                    user: payer.pubkey(),
                    market: market_pda,
                    system_program: system_program::ID,
                    vault: vault_pda,
                })
                .args(zone::instruction::CreatePrediction {
                    prediction,
                    amount: amount * LAMPORTS_PER_SOL,
                    current_price: *current_price,
                })
                .send()
                .expect("Failed to send create prediction transaction");

            println!("Successfully create prediction: https://solscan.io/tx/{sig}?cluster=devnet");
        }
    }
}
