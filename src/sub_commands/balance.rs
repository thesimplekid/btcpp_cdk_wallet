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

    let mut wallets_vec = Vec::new();

    // TODO: Print balance of each mint in multimint wallet

    Ok(wallets_vec)
}
