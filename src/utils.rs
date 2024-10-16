use mpl_token_metadata::accounts::Metadata;
use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_client::rpc_config::RpcTransactionConfig;
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding, UiTransactionTokenBalance,
};
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
    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::JsonParsed),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };
    client
        .get_transaction_with_config(&signature, config)
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
        signatures.first().map(|s| s.signature.to_string())
    } else {
        None
    }
}

#[derive(Debug)]
pub enum TypeTransaction {
    Buy,
    Sell,
}

pub fn calculate_transaction_info(
    pre_tb: &[UiTransactionTokenBalance],
    post_tb: &[UiTransactionTokenBalance],
    address: &str,
) -> (f64, String, TypeTransaction) {
    let mut seller = "".to_string();
    let mut buyer = "".to_string();
    let mut change_amount = 0.0;
    let mut mint = "";
    for post in post_tb {
        if let Some(pre) = pre_tb
            .iter()
            .find(|x| x.account_index == post.account_index)
        {
            let post_amount = post.ui_token_amount.ui_amount.unwrap_or(0.0);
            let pre_amount = pre.ui_token_amount.ui_amount.unwrap_or(0.0);
            let post_owner = match &post.owner {
                OptionSerializer::Some(o) => o.clone(),
                _ => "".to_string(),
            };
            let _pre_owner = match &pre.owner {
                OptionSerializer::Some(o) => o.clone(),
                _ => "".to_string(),
            };
            if post_amount - pre_amount > 0.0 {
                buyer = post_owner;
                change_amount = post_amount - pre_amount;
                mint = &post.mint;
                if !almost_equal(post_amount,pre_amount) {
                    break
                }
            } else {
                seller = post_owner;
                change_amount = pre_amount - post_amount;
                mint = &post.mint;
                if !almost_equal(post_amount,pre_amount) {
                    break
                }
            }
        }
    }
    if seller.is_empty() {
        if seller == address {
            (change_amount, mint.to_string(), TypeTransaction::Sell)
        } else {
            (change_amount, mint.to_string(), TypeTransaction::Buy)
        }
    } else if buyer == address {
        (change_amount, mint.to_string(), TypeTransaction::Buy)
    } else {
        (change_amount, mint.to_string(), TypeTransaction::Sell)
    }
}

pub fn almost_equal(a: f64, b: f64) -> bool {
    let epsilon = 1e-10;
    (a - b).abs() < epsilon
}
