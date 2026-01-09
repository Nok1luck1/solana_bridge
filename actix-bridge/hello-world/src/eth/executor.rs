use alloy::{
    primitives::{Address, U256},
    providers::{
        Identity, RootProvider,
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
    },
    rpc::types::Log,
};
use std::error::Error;
use std::str::FromStr;
use tracing::error;

use crate::eth::{Bridge, check_balance};

pub async fn distribute_reward(
    token_to_distribute: Address,
    amount_to_distribute: U256,
    provider: alloy::providers::fillers::FillProvider<
        alloy::providers::fillers::JoinFill<
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
    let current_available_balance: U256 = check_balance(token_to_distribute, &provider).await?;
    if current_available_balance < amount_to_distribute {
        error!("Bridge does not have specific amount to distribute");
        std::process::exit(1);
    }
    Ok(())
}
