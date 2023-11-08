use serde::Deserialize;
use web3::types::{Address, H256, U256, U64};

use super::get_asset_transfers_params::Category;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetTransfersResponse {
    pub page_key: Option<String>,
    pub transfers: Vec<AssetTransfer>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetTransfer {
    pub asset: Option<String>,
    pub block_num: U64,
    pub category: Category,
    pub erc1155_metadata: Option<Vec<Erc1155Metadata>>,
    pub erc721_token_id: Option<H256>,
    pub from: Address,
    pub hash: H256,
    pub raw_contract: RawContract,
    pub to: Address,
    pub token_id: Option<H256>,
    pub unique_id: String,
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawContract {
    pub address: Option<Address>,
    pub decimal: Option<U256>,
    pub value: Option<U256>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Erc1155Metadata {
    pub token_id: U256,
    pub value: U256,
}
