use std::str::FromStr;

use rust_aptos_account_client::{AccountClient, types::SignerCapabilityOfferProofChallengeV2};
use once_cell::sync::Lazy;
use aptos_sdk::{types::{ account_address::AccountAddress, LocalAccount}, rest_client::{Client, FaucetClient}, bcs};
use anyhow::{Context, Result};
use url::Url;

static NODE_URL: Lazy<Url> = Lazy::new(||  Url::from_str("https://fullnode.devnet.aptoslabs.com").unwrap());
static FAUCET_URL: Lazy<Url> = Lazy::new(|| Url::from_str("https://faucet.devnet.aptoslabs.com").unwrap());

#[tokio::main]
async fn main() -> Result<()> {
    let rest_client = Client::new(NODE_URL.clone());
    let faucet_client = FaucetClient::new(FAUCET_URL.clone(), NODE_URL.clone());
    let account_client = AccountClient::new(&rest_client).await?;
    
    let mut alice = LocalAccount::generate(&mut rand::rngs::OsRng);
    let bob = LocalAccount::generate(&mut rand::rngs::OsRng);
    
    faucet_client.fund(alice.address(), 1_000_000).await.context("Failed to fund Alice's account")?;
    faucet_client.fund(bob.address(), 1_000_000).await.context("Failed to fund Bob's account")?;


    let proof_struct = SignerCapabilityOfferProofChallengeV2 {
        account_address: AccountAddress::ONE,
        module_name: String::from("account"),
        struct_name: String::from("SignerCapabilityOfferProofChallengeV2"),
        sequence_number: alice.sequence_number(),
        source_address: alice.address(),
        recipient_address: bob.address(),
    };

    let proof_struct_bytes = bcs::to_bytes(&proof_struct)?;
    let signature = alice.private_key().sign_arbitrary_message(&proof_struct_bytes);

    let tx_hash = account_client.offer_signer_capability(
        &mut alice,
        signature,
        bob.address(),
        None,
    ).await?;

    rest_client.wait_for_transaction(&tx_hash).await?;
    Ok(())
}
