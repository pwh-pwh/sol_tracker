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
    let tx_id =
        "2FM9dAMegHqt75rapiD5Ynpbgd4kT2t7g4QPXhhWLxCXCoGgxgSZwGLDYt3QHXZqDcmNivK7YJ89FXLrS42m2JNS";
    let meta = utils::get_transaction_details(&client, tx_id).unwrap();
    // dbg!(&meta);
    dbg!(meta.get_account_list());
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
