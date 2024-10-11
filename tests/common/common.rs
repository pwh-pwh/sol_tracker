use solana_client::rpc_client::RpcClient;

pub fn get_rpc_client() -> RpcClient {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    RpcClient::new(rpc_url)
}
