use std::str::FromStr;
use solana_program::pubkey::Pubkey;
use sol_tracker::utils;
use crate::common::common::get_rpc_client;

mod common;

#[test]
fn test_get_metadata_by_mint() {
    let client = get_rpc_client();
    let mint = Pubkey::from_str("EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm").unwrap();
    let metadata = utils::get_metadata_by_mint(&client, &mint).unwrap();
// 打印结果
    println!("{:?}", metadata);
}