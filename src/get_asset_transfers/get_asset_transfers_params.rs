use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;
use web3::{
    helpers,
    types::{Address, BlockNumber},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetTransfersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_block: Option<BlockNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_block: Option<BlockNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_addresses: Option<Vec<Address>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<Category>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_zero_value: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_key: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    External,
    Internal,
    Erc20,
    Erc721,
    Erc1155,
    SpecialNft,
}

impl GetAssetTransfersParams {
    pub(crate) fn serialize(self) -> Value {
        helpers::serialize(&self)
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for GetAssetTransfersParams {
    fn default() -> Self {
        Self {
            category: Some(vec![Category::Erc721]),
            from_block: None,
            to_block: None,
            from_address: None,
            to_address: None,
            contract_addresses: None,
            with_metadata: None,
            exclude_zero_value: None,
            max_count: None,
            page_key: None,
        }
    }
}
