use aptos_sdk::{
    rest_client::{Client as ApiClient, PendingTransaction},
    types::{account_address::AccountAddress, LocalAccount}, crypto::ed25519::Ed25519Signature, bcs,
};
use anyhow::{Context, Result};

pub mod types;
mod module_client;
use module_client::ModuleClient;
use types::*;

pub struct AccountClient <'a> {
    api_client: &'a ApiClient,
    module_client: ModuleClient,
}

impl <'a> AccountClient<'a> {
    pub async fn new(api_client: &'a ApiClient) -> Result<AccountClient<'a>> {
        let chain_id = api_client
        .get_index()
        .await
        .context("Failed to get chain ID")?
        .inner()
        .chain_id;
        let module_client = ModuleClient::new(
            chain_id, 
            AccountAddress::ONE,
            "account"
        );
        Ok(Self {
            api_client,
            module_client,
        })
    }

    pub async fn offer_signer_capability(
        &self,
        from: &mut LocalAccount,
        signature: Ed25519Signature,
        to: AccountAddress,
        options: Option<TransactionOptions>,
    ) -> Result<PendingTransaction> {
        let options = options.unwrap_or_default();

        let signed_txn = self.module_client.build_signed_transaction(
            from,
            "offer_signer_capability",
            vec![],
            vec![
                bcs::to_bytes(&signature).unwrap(),
                vec![0u8],
                bcs::to_bytes(&from.public_key()).unwrap(),
                bcs::to_bytes(&to).unwrap(),
            ],
            options
        );

        Ok(self
            .api_client
            .submit(&signed_txn)
            .await
            .context("Failed to submit offer signer capability tx")?
            .into_inner()
        )
    }
}