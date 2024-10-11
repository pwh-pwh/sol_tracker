use mpl_token_metadata::accounts::Metadata;
use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding};
use std::str::FromStr;

pub fn get_metadata_by_mint(
    client: &RpcClient,
    mint: &Pubkey,
) -> Result<Metadata, Box<dyn std::error::Error>> {
    let (pub_key, _) = Metadata::find_pda(mint);
    let account_data = client.get_account_data(&pub_key)?;
    Metadata::safe_deserialize(&account_data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

pub fn get_transaction_details(
    client: &RpcClient,
    tx_id: &str,
) -> Result<EncodedConfirmedTransactionWithStatusMeta, Box<dyn std::error::Error>> {
    let signature = Signature::from_str(tx_id).unwrap();
    client
        .get_transaction(&signature, UiTransactionEncoding::JsonParsed)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

pub fn get_lastest_transaction_signature(client: &RpcClient, address: &Pubkey) -> Option<String> {
    let config = GetConfirmedSignaturesForAddress2Config {
        before: None,
        until: None,
        limit: Some(1),
        commitment: Some(CommitmentConfig::confirmed()),
    };
    let result = client.get_signatures_for_address_with_config(address, config);
    if let Ok(signatures) = result {
        Some(signatures.first().unwrap().signature.clone())
    } else {
        None
    }
}
