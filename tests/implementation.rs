use smart_contract_scanner::{
    env::{get, load_env},
    get_asset_transfers::{GetAssetTransfers, GetAssetTransfersParams},
    web3::create_web3_client,
};

#[tokio::test]
async fn implementation_test() {
    load_env();
    let node_url = get("NODE_URL").expect("NODE_URL not found in env");
    let web = create_web3_client(&node_url);

    let api: GetAssetTransfers<_> = web.api();

    let response = api
        .query(GetAssetTransfersParams::new())
        .await
        .expect("Error querying");

    assert!(response.page_key.is_some(), "Next page token should exist");
    assert!(!response.transfers.is_empty(), "Empty transfers");
}
