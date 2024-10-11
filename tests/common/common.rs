use solana_client::rpc_client::RpcClient;

pub fn get_rpc_client() ->RpcClient {
    let rpc_url = "https://mainnet.helius-rpc.com/?api-key=4282ccbd-d6c6-4101-8ef8-3959b134f908";
    RpcClient::new(rpc_url)
}