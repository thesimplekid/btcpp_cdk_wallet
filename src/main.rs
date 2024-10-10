use std::{fs, sync::Arc};

use bip39::Mnemonic;
use cdk::{
    cdk_database::{self, WalletDatabase},
    mint_url::MintUrl,
    nuts::CurrencyUnit,
    wallet::{multi_mint_wallet::WalletKey, MultiMintWallet, Wallet},
};
use clap::{Parser, Subcommand};
use rand::Rng;
use tracing_subscriber::EnvFilter;

mod sub_commands;

const DEFAULT_WORK_DIR: &str = ".cdk-cli";

/// Simple CLI application to interact with cashu
#[derive(Parser)]
#[command(name = "cashu-tool")]
#[command(author = "thesimplekid <tsk@thesimplekid.com>")]
#[command(version = "0.1.0")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Mint(sub_commands::mint::MintSubCommand),
    /// Send
    Send(sub_commands::send::SendSubCommand),
    Balance,
    /// Receive token
    Receive(sub_commands::receive::ReceiveSubCommand),
    /// Pay bolt11 invoice
    Pay(sub_commands::melt::MeltSubCommand),
    /// Decode a token
    DecodeToken(sub_commands::decode_token::DecodeTokenSubCommand),
    ListProof(sub_commands::list_proofs::ListProofsSubCommand),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Cli = Cli::parse();
    let default_filter = "warn";

    let sqlx_filter = "sqlx=warn";

    let env_filter = EnvFilter::new(format!("{},{}", default_filter, sqlx_filter));

    // Parse input
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    fs::create_dir_all(&DEFAULT_WORK_DIR)?;

    let localstore = create_localstore().await;

    let mut rng = rand::thread_rng();
    let random_bytes: [u8; 32] = rng.gen();
    let mnemonic = Mnemonic::from_entropy(&random_bytes)?;

    let multi_mint_wallet =
        create_multimint_wallet(&mnemonic.to_seed_normalized(""), localstore.clone()).await?;

    match args.command {
        Commands::Mint(sub_command_args) => {
            sub_commands::mint::mint(
                &multi_mint_wallet,
                &mnemonic.to_seed_normalized(""),
                localstore,
                &sub_command_args,
            )
            .await?
        }
        Commands::Balance => sub_commands::balance::balance(&multi_mint_wallet).await?,
        Commands::Send(sub_command_args) => {
            sub_commands::send::send(&multi_mint_wallet, &sub_command_args).await?
        }
        Commands::Receive(sub_command_args) => {
            sub_commands::receive::receive(&multi_mint_wallet, &sub_command_args).await?
        }
        Commands::Pay(sub_command_args) => {
            sub_commands::melt::pay(&multi_mint_wallet, &sub_command_args).await?
        }
        Commands::DecodeToken(sub_command_args) => {
            sub_commands::decode_token::decode_token(&sub_command_args)?
        }
        Commands::ListProof(sub_command_args) => {
            sub_commands::list_proofs::list_proofs(
                &multi_mint_wallet,
                &mnemonic.to_seed_normalized(""),
                localstore,
                &sub_command_args,
            )
            .await?
        }
    }

    Ok(())
}

async fn create_localstore() -> Arc<dyn WalletDatabase<Err = cdk_database::Error> + Send + Sync> {
    todo!()
}

async fn create_multimint_wallet(
    seed: &[u8],
    localstore: Arc<dyn WalletDatabase<Err = cdk_database::Error> + Sync + Send>,
) -> anyhow::Result<MultiMintWallet> {
    let mut wallets: Vec<Wallet> = Vec::new();

    // TODO: Get mints from localstore
    let mints = todo!();

    // TODO: For mint in store create wallet
    for (mint, _) in mints {
        let wallet = todo!();
        wallets.push(wallet);
    }

    Ok(MultiMintWallet::new(wallets))
}

pub async fn get_single_mint_wallet(
    multi_mint_wallet: &MultiMintWallet,
    seed: &[u8],
    localstore: Arc<dyn WalletDatabase<Err = cdk_database::Error> + Sync + Send>,
    mint_url: MintUrl,
    unit: CurrencyUnit,
) -> anyhow::Result<Wallet> {
    let wallet = match multi_mint_wallet
        .get_wallet(&WalletKey::new(mint_url.clone(), unit))
        .await
    {
        Some(wallet) => wallet.clone(),
        None => {
            let wallet = Wallet::new(
                &mint_url.to_string(),
                CurrencyUnit::Sat,
                localstore,
                seed,
                None,
            )?;

            multi_mint_wallet.add_wallet(wallet.clone()).await;
            wallet
        }
    };

    Ok(wallet)
}
