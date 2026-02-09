use crate::eth::ERC20;
use alloy::{
    primitives::{Address, U256},
    providers::{
        Identity, Provider, RootProvider,
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
    },
    signers::k256::elliptic_curve::bigint,
};
use std::{error::Error, str::FromStr};

pub async fn check_balance(
    token_addr: Address,
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
) -> Result<U256, Box<dyn Error>> {
    let token = ERC20::new(token_addr, provider);
    let addr = std::env::var("CONTRACT_ADDR").expect("Contract addr must be set in .env");
    let contract_address = Address::from_str(addr.as_str());
    let bridge_balance = token.balanceOf(contract_address.unwrap()).call().await?;
    Ok(bridge_balance)
}
