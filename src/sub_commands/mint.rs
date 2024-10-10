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
    let quote = wallet
        .mint_quote(Amount::from(sub_command_args.amount), None)
        .await?;

    println!("{:#?}", quote);

    // loop {
    //     // TODO: Check mint quote status
    // }

    loop {
        let status = wallet.mint_quote_state(&quote.id).await?;

        if status.state == MintQuoteState::Paid {
            break;
        }

        sleep(Duration::from_secs(2)).await;
    }

    // TODO: Mint once quote has been paid
    let receive_amount = wallet.mint(&quote.id, SplitTarget::default(), None).await?;

    println!("Received {receive_amount} from mint {mint_url}");

    Ok(())
}
