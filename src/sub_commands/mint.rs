use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use cdk::amount::SplitTarget;
use cdk::cdk_database::{Error, WalletDatabase};
use cdk::mint_url::MintUrl;
use cdk::nuts::{CurrencyUnit, MintQuoteState};
use cdk::wallet::MultiMintWallet;
use cdk::Amount;
use clap::Args;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::get_single_mint_wallet;

#[derive(Args, Serialize, Deserialize)]
pub struct MintSubCommand {
    /// Mint url
    mint_url: MintUrl,
    /// Amount
    amount: u64,
}

pub async fn mint(
    multi_mint_wallet: &MultiMintWallet,
    seed: &[u8],
    localstore: Arc<dyn WalletDatabase<Err = Error> + Sync + Send>,
    sub_command_args: &MintSubCommand,
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

    // TODO: Get mint quote
    let mint_quote = wallet.mint_quote(10.into(), None).await?;

    println!("{:#?}", mint_quote);

    loop {
        // TODO: Check mint quote status
        let state = wallet.mint_quote_state(&mint_quote.id).await?;

        if state.state == MintQuoteState::Paid {
            break;
        }
    }

    // TODO: Mint once quote has been paid
    let mint = wallet.mint(&mint_quote.id, SplitTarget::None, None).await?;

    println!("{}", mint);

    Ok(())
}
