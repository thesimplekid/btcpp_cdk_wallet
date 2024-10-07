use std::sync::Arc;

use anyhow::Result;
use cdk::cdk_database::{Error, WalletDatabase};
use cdk::mint_url::MintUrl;
use cdk::nuts::{CurrencyUnit, MintQuoteState};
use cdk::wallet::MultiMintWallet;
use clap::Args;
use serde::{Deserialize, Serialize};

use crate::get_single_mint_wallet;

#[derive(Args, Serialize, Deserialize)]
pub struct ListProofsSubCommand {
    /// Mint url
    mint_url: MintUrl,
}

pub async fn list_proofs(
    multi_mint_wallet: &MultiMintWallet,
    seed: &[u8],
    localstore: Arc<dyn WalletDatabase<Err = Error> + Sync + Send>,
    sub_command_args: &ListProofsSubCommand,
) -> Result<()> {
    let mint_url = sub_command_args.mint_url.clone();

    let wallet = get_single_mint_wallet(
        multi_mint_wallet,
        seed,
        localstore,
        mint_url.clone(),
        CurrencyUnit::Sat,
    )
    .await?;

    let proofs = wallet.get_proofs().await?;

    for proof in proofs {
        println!("Amount: {}, Secret: {}", proof.amount, proof.secret);
    }

    Ok(())
}
