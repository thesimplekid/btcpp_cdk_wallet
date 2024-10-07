use std::io;
use std::io::Write;
use std::str::FromStr;

use anyhow::{bail, Result};
use cdk::amount::SplitTarget;
use cdk::nuts::{Conditions, CurrencyUnit, PublicKey, SpendingConditions, Token};
use cdk::wallet::multi_mint_wallet::WalletKey;
use cdk::wallet::types::SendKind;
use cdk::wallet::MultiMintWallet;
use cdk::Amount;
use clap::Args;

use crate::sub_commands::balance::mint_balances;

#[derive(Args)]
pub struct SendSubCommand {
    /// Token Memo
    #[arg(short, long)]
    memo: Option<String>,
    /// Pubkey to lock proofs to
    #[arg(short, long, action = clap::ArgAction::Append)]
    pubkey: Vec<String>,
    /// Token as V3 token
    #[arg(short, long)]
    v3: bool,
}

pub async fn send(
    multi_mint_wallet: &MultiMintWallet,
    sub_command_args: &SendSubCommand,
) -> Result<()> {
    let unit = CurrencyUnit::Sat;
    // Get and print balances of all wallets in multimint wallet
    let mints_amounts = mint_balances(multi_mint_wallet).await?;

    println!("Enter mint number to create token");

    let mut user_input = String::new();
    let stdin = io::stdin();
    io::stdout().flush().unwrap();
    stdin.read_line(&mut user_input)?;

    let mint_number: usize = user_input.trim().parse()?;

    if mint_number.gt(&(mints_amounts.len() - 1)) {
        bail!("Invalid mint number");
    }

    println!("Enter value of token in sats");

    let mut user_input = String::new();
    let stdin = io::stdin();
    io::stdout().flush().unwrap();
    stdin.read_line(&mut user_input)?;
    let token_amount = Amount::from(user_input.trim().parse::<u64>()?);

    if token_amount.gt(&mints_amounts[mint_number].1) {
        bail!("Not enough funds");
    }

    let mint_url = mints_amounts[mint_number].0.clone();

    let wallet = multi_mint_wallet
        .get_wallet(&WalletKey::new(mint_url, unit))
        .await
        .expect("Known wallet");

    // TODO: Use wallet.send to create token
    let token: Token = todo!();

    match sub_command_args.v3 {
        true => {
            let token = token;

            println!("{}", token.to_v3_string());
        }
        false => {
            println!("{}", token);
        }
    }

    Ok(())
}
