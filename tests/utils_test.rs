use crate::common::common::get_rpc_client;
use sol_tracker::{utils, EncodedConfirmedTransactionMetaInfo};
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

mod common;

#[test]
fn test_get_metadata_by_mint() {
    let client = get_rpc_client();
    let mint = Pubkey::from_str("4uP8C8AWoXJQjU41RoVyjGneCwJtzzyHY1R4n7yPpump").unwrap();
    let metadata = utils::get_metadata_by_mint(&client, &mint).unwrap();
    // 打印结果
    println!("{:?}", metadata);
}

#[test]
fn test_get_transaction_details() {
    let client = get_rpc_client();
    // let tx_id =
    //     "2YFvE4hLZXpbzaK43tyBnWAAk67Agq7pJ24AetDH6F4x8rLmMU6iTBHvUTWNAGdhHeuN5Bk61UL89U7Ks3id3SX";
    //3xQpd3xSthQ1S589shuqpaVmgBoug9eu4g1g6mi9yJF4QjF1WrSHKPG3KrBUYDcre7WM6rJqqXKbKTy1ivLX9ZX3
    //2FM9dAMegHqt75rapiD5Ynpbgd4kT2t7g4QPXhhWLxCXCoGgxgSZwGLDYt3QHXZqDcmNivK7YJ89FXLrS42m2JNS
    let tx_id =
        "5uWAsNnrLaUudAqYDDm4qLBsLCgPy9pBYEStntdfaqSTVfnDirQFE3QzQXhUr63gaypK66BX694pwkub6Zfv68HM";
    let meta = utils::get_transaction_details(&client, tx_id).unwrap();
    // dbg!(&meta);
    dbg!(meta.get_account_list());
    dbg!(meta.get_pre_token_balance_info());
    dbg!(meta.get_post_token_balance_info());
    let pret = meta.get_pre_token_balance_info();
    let postt = meta.get_post_token_balance_info();
    let info = utils::calculate_transaction_info(
        &pret.unwrap(),
        &postt.unwrap(),
        "CANTSsRNWR2ykW4YejwMPgLBJ1GbR1FYoX47yJconumj",
    );
    dbg!(info);
}

#[test]
fn test_get_lastest_transaction_signature() {
    let client = get_rpc_client();
    let sign = utils::get_lastest_transaction_signature(
        &client,
        &Pubkey::from_str("CANTSsRNWR2ykW4YejwMPgLBJ1GbR1FYoX47yJconumj").unwrap(),
    );
    println!("{:?}", sign);
}
