use std::str::FromStr;
use std::thread;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use sol_tracker::{utils, EncodedConfirmedTransactionMetaInfo};

fn main() {
    //pumpFun address: 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
    //Raydium address: 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
    loop {
        let rpc_url = "https://mainnet.helius-rpc.com/?api-key=8aea2ad5-ddf6-4a78-9132-0e03995c5369";
        let client = RpcClient::new(rpc_url);
        let sign = utils::get_lastest_transaction_signature(
            &client,
            &Pubkey::from_str("2PX2PYdsFJ2ki6wUbYSR8Wfn4LYwvMFwyAy3SY3zCshx").unwrap(),
        );
        let sign = sign.unwrap();
        let meta = utils::get_transaction_details(&client, &sign).unwrap();
        let pret = meta.get_pre_token_balance_info();
        let postt = meta.get_post_token_balance_info();
        let info = utils::calculate_transaction_info(
            &pret.unwrap(),
            &postt.unwrap(),
            "2PX2PYdsFJ2ki6wUbYSR8Wfn4LYwvMFwyAy3SY3zCshx",
        );
        let mint = &info.1;
        dbg!(meta.get_account_pre_balance());
        dbg!(meta.get_account_post_balance());
        let metadata = utils::get_metadata_by_mint(&client, &Pubkey::from_str(mint).unwrap()).unwrap();
        dbg!(&metadata.symbol.trim_end_matches('\0'));
        dbg!(metadata.symbol);
        dbg!(info);
        thread::sleep(std::time::Duration::from_secs(5));
    }

}
