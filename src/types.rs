use serde::{Deserialize, Serialize};
use aptos_sdk::types::account_address::AccountAddress;

pub struct TransactionOptions {
    pub max_gas_amount: u64,

    pub gas_unit_price: u64,

    /// This is the number of seconds from now you're willing to wait for the
    /// transaction to be committed.
    pub timeout_sec: u64,

    pub coin_type: String,
}

impl Default for TransactionOptions {
    fn default() -> Self {
        Self {
            max_gas_amount: 5_000,
            gas_unit_price: 100,
            timeout_sec: 10,
            coin_type: "0x1::aptos_coin::AptosCoin".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignerCapabilityOfferProofChallengeV2 {
    pub account_address: AccountAddress,
    pub module_name: String,
    pub struct_name: String,
    pub sequence_number: u64,
    pub source_address: AccountAddress,
    pub recipient_address: AccountAddress,
}
