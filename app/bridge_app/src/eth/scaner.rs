use crate::eth::Bridge;
use alloy::{
    primitives::{Address, U256},
    providers::{
        Identity, Provider, RootProvider,
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
    },
};
use std::{error::Error, str::FromStr};

pub async fn scan_for_orders(
    block_start: u64,
    block_latest: u64,
    provider: &alloy::providers::fillers::FillProvider<
        JoinFill<
            alloy::providers::fillers::JoinFill<
                alloy::providers::Identity,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::GasFiller,
                    alloy::providers::fillers::JoinFill<
                        alloy::providers::fillers::BlobGasFiller,
                        alloy::providers::fillers::JoinFill<
                            alloy::providers::fillers::NonceFiller,
                            alloy::providers::fillers::ChainIdFiller,
                        >,
                    >,
                >,
            >,
            alloy::providers::fillers::WalletFiller<alloy::network::EthereumWallet>,
        >,
        alloy::providers::RootProvider,
    >,
) -> Result<(), Box<dyn Error>> {
    let addr = std::env::var("CONTRACT_ADDR").expect("Contract addr must be set in .env");
    let contract_address = Address::from_str(addr.as_str());
    let bridge_contract = Bridge::new(contract_address.unwrap(), &provider);
    let events = bridge_contract
        .OrderCreated_filter()
        .from_block(block_start)
        .to_block(block_latest)
        .query()
        .await?;
    for (event, log) in events {
        println!(
            "Orderid : {:?}, in Block : {:?},in timestamp: {:?}",
            event.orderId, log.block_number, log.block_timestamp
        );
    }
    Ok(())
}
