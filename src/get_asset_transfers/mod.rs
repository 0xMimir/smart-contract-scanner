use web3::{api::Namespace, helpers, Result, Transport};

pub use self::{
    get_asset_transfers_params::GetAssetTransfersParams,
    get_asset_transfers_response::GetAssetTransfersResponse,
};

mod get_asset_transfers_params;
mod get_asset_transfers_response;

#[derive(Clone)]
/// Only forks on alchemy nodes
///
/// Further docs: https://docs.alchemy.com/reference/alchemy-getassettransfers
pub struct GetAssetTransfers<T: Transport> {
    transport: T,
}

impl<T> Namespace<T> for GetAssetTransfers<T>
where
    T: Transport,
    Self: Clone,
{
    fn new(transport: T) -> Self {
        Self { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

impl<T> GetAssetTransfers<T>
where
    T: Transport,
{
    const METHOD: &str = "alchemy_getAssetTransfers";

    pub async fn query(
        &self,
        params: GetAssetTransfersParams,
    ) -> Result<GetAssetTransfersResponse> {
        let params = vec![params.serialize()];
        let value = self.transport.execute(Self::METHOD, params).await?;
        println!("{}", value);
        helpers::decode(value)
    }
}
