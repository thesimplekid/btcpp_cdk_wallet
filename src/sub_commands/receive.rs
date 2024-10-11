use std::str::FromStr;

use anyhow::Result;
use cdk::nuts::SecretKey;
use cdk::wallet::multi_mint_wallet::MultiMintWallet;
use clap::Args;

#[derive(Args)]
pub struct ReceiveSubCommand {
    /// Cashu Token
    token: String,
    /// Signing Key
    #[arg(short, long, action = clap::ArgAction::Append)]
    signing_key: Vec<String>,
}

pub async fn receive(
    multi_mint_wallet: &MultiMintWallet,
    sub_command_args: &ReceiveSubCommand,
) -> Result<()> {
    let mut signing_keys = Vec::new();

    if !sub_command_args.signing_key.is_empty() {
        let mut s_keys: Vec<SecretKey> = sub_command_args
            .signing_key
            .iter()
            .map(|s| {
                if s.starts_with("nsec") {
                    let nostr_key = nostr_sdk::SecretKey::from_str(s).expect("Invalid secret key");

                    SecretKey::from_str(&nostr_key.to_secret_hex())
                } else {
                    SecretKey::from_str(s)
                }
            })
            .collect::<Result<Vec<SecretKey>, _>>()?;
        signing_keys.append(&mut s_keys);
    };

    let token_str = &sub_command_args.token;

    // TODO: Call receive on multi mint wallet

    let amount = multi_mint_wallet.receive(&token_str, &[], &[]).await?;

    println!("Received: {}", amount);

    Ok(())
}
