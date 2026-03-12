use crate::eth::ERC20;
use alloy::eips::eip4844::c_kzg::Bytes32;
use alloy::primitives::{Address, FixedBytes, U256};
use alloy::providers::fillers::JoinFill;
use std::error::Error;
use std::str::FromStr;
use tracing::error;

use crate::eth::Bridge;

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

pub async fn distribute_reward(
    address_receiver: Address,
    token_deposited: String,
    token_to_distribute: Address,
    address_sender: String,
    amount_deposited: u64,
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
    let disctribute_token = bridge_contract.distributeReward(
        address_receiver,
        token_deposited,
        token_to_distribute,
        address_sender,
        U256::from(amount_deposited),
        U256::from(amount_to_distribute),
    );
    let _distribute_tx = disctribute_token.send().await?;
    Ok(())
}
pub async fn get_order_info(
    order_id: FixedBytes<32>,
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
) -> Result<Bridge::Order, Box<dyn Error>> {
    let addr = std::env::var("CONTRACT_ADDR").expect("Contract addr must be set in .env");
    let contract_address = Address::from_str(addr.as_str());
    let bridge_contract = Bridge::new(contract_address.unwrap(), &provider);
    let order_info: Bridge::Order = bridge_contract.getOrderInfo(order_id).call().await?;
    //println!("getInfo about order {order_id},{order_info:?}");
    Ok(order_info)
}
