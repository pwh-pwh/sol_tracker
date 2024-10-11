use mpl_token_metadata::accounts::Metadata;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

pub fn get_metadata_by_mint(
    client: &RpcClient,
    mint: &Pubkey,
) -> Result<Metadata, Box<dyn std::error::Error>> {
    let (pub_key, _) = Metadata::find_pda(mint);
    let account_data = client.get_account_data(&pub_key)?;
    Metadata::safe_deserialize(&mut account_data.as_ref())
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
