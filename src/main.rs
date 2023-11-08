use smart_contract_scanner::{
    env,
    get_asset_transfers::{GetAssetTransfers, GetAssetTransfersParams},
    web3::create_web3_client,
};

#[tokio::main]
async fn main() {
    env::load_env();

    let node_url = env::get("NODE_URL").unwrap();
    let web3 = create_web3_client(&node_url);
    let namespace: GetAssetTransfers<_> = web3.api();

    let params = GetAssetTransfersParams::default();
    namespace.query(params).await.unwrap();
}
