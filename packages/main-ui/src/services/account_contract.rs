use std::sync::Arc;

use ethers::prelude::*;

#[derive(Debug, Clone)]
pub struct AccountContract {
    pub contract: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
    pub nft_address: String,
}
