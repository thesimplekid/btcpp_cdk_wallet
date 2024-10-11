use std::collections::BTreeMap;

use anyhow::Result;
use cdk::mint_url::MintUrl;
use cdk::nuts::CurrencyUnit;
use cdk::wallet::multi_mint_wallet::MultiMintWallet;
use cdk::Amount;

pub async fn balance(multi_mint_wallet: &MultiMintWallet) -> Result<()> {
    mint_balances(multi_mint_wallet).await?;
    Ok(())
}

pub async fn mint_balances(multi_mint_wallet: &MultiMintWallet) -> Result<Vec<(MintUrl, Amount)>> {
    // TODO: Get balances of wallets in multimint wallet
    let wallets: BTreeMap<MintUrl, Amount> =
        multi_mint_wallet.get_balances(&CurrencyUnit::Sat).await?;

    let mut wallets_vec = Vec::new();

    // TODO: Print balance of each mint in multimint wallet
    for (i, (mint_url, amount)) in wallets.iter().enumerate() {
        let mint_url = mint_url.clone();
        println!("{i}: {mint_url} {amount}");
        wallets_vec.push((mint_url, *amount))
    }

    Ok(wallets_vec)
}
