#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::str::FromStr;
    use std::sync::Arc;
    use ethers::core::rand::thread_rng;
    use ethers::middleware::gas_oracle::GasNow;
    use ethers::middleware::MiddlewareBuilder;
    use ethers::providers::{Http, Middleware};
    use ethers::signers::{LocalWallet, Signer, Wallet};
    use ethers::types::{H256};
    use reth_primitives::{BlockId, BlockNumber};
    use reth_rpc::EthApiSpec;
    use reth_primitives::Block;
    use crate::RethMiddleware;
    use super::*;

    #[tokio::test]
    async fn test() {
        let signing_key = "0x8bcc27c22e14953faaf6ed2ee62f8f62d0cc670c07ec2680fe408f335760d7fb";
        let signer = signing_key.parse::<LocalWallet>().unwrap();
        let address = signer.address();
        let gas_oracle = GasNow::new();
        // Make sure that you have this one copied or accessible to Reth database
        let db_path = Path::new("./mdbx.dat");

        let rpc_endpoint = "https://ethereum-mainnet-archive.allthatnode.com";

        // We need tokio::spawn to create a new task for the blocking code
        // for preventing its new `Runtime` won't be dropped in an async context
        let provider_result = tokio::task::spawn_blocking(move || {
            ethers::providers::Provider::<Http>::try_from(rpc_endpoint)
                .unwrap()
                .wrap_into(|s| RethMiddleware::new(Arc::new(s), db_path))
        }).await.unwrap();

        println!("provider_result: {:?}", provider_result.reth_api.sync_status());
        println!("provider_result: {:?}", provider_result.reth_api.is_syncing());

        // genesis block hash: 0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3
        let block_genesis_0_hash = "\"0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3\"";

        // let block_number_1_hash = "\"0x88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6\"";
        let block_hash: H256 = serde_json::from_str(block_genesis_0_hash).unwrap();
        let block_id: BlockId = serde_json::from_str(block_genesis_0_hash).unwrap();
        // assert_eq!(block_id, BlockId::Hash(block_hash.into()));

        let block = provider_result.reth_api.state_at_block_id(block_id).unwrap();
        let genesis_block_number: BlockNumber = 0 as u64;
        let genesis_block_hash = block.block_hash(genesis_block_number).unwrap();

        println!("block: {:?}", genesis_block_hash.unwrap());
        assert_eq!("0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3", format!("{:?}", genesis_block_hash.unwrap()));

        // vitalik: 0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045
        // println!("block: {:?}", block.account_balance("0x0000000000000000000000000000000000000000".parse().unwrap()));
    }
}
