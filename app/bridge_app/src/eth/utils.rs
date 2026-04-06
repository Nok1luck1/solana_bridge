use crate::eth::Bridge;
use crate::eth::ERC20;
use alloy::primitives::{Address, FixedBytes, U256};
use alloy::providers::{fillers::JoinFill, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use std::error::Error;
use std::str::FromStr;
use tokio::sync::OnceCell;
use tracing::error;

static EVM_PROVIDER: OnceCell<
    alloy::providers::fillers::FillProvider<
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
> = OnceCell::const_new();

pub async fn connect_static_evm_provider() -> &'static alloy::providers::fillers::FillProvider<
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
> {
    EVM_PROVIDER
        .get_or_init(|| async {
            let provider_end_point =
                std::env::var("PROVIDER_ETH_ENDPOINT").expect("PROVIDER DID NOT SET");

            let private_key = std::env::var("PRIVATE_KEY_ETH").expect("PRIVATE KEY NOT SET");

            let signer: PrivateKeySigner = private_key.parse().expect("private key parse error");

            ProviderBuilder::new()
                .wallet(signer)
                .connect(provider_end_point.as_str())
                .await
                .expect("Connection evm error")
        })
        .await
}

pub async fn check_balance(token_addr: Address) -> Result<U256, Box<dyn Error>> {
    let provider = connect_static_evm_provider().await;
    let token = ERC20::new(token_addr, provider);
    let addr = std::env::var("BRIDGE_EVM_ADDR").expect("Contract addr must be set in .env");
    let contract_address = Address::from_str(addr.as_str());
    let bridge_balance = token.balanceOf(contract_address.unwrap()).call().await?;
    Ok(bridge_balance)
}

pub async fn execute_order_evm(
    address_receiver: Address,
    token_deposited: String,
    token_to_distribute: Address,
    address_sender: String,
    amount_deposited: u64,
    amount_to_distribute: U256,
) -> Result<(), Box<dyn Error>> {
    let provider = connect_static_evm_provider().await;
    let addr = std::env::var("BRIDGE_EVM_ADDR").expect("Contract addr must be set in .env");
    let contract_address = Address::from_str(addr.as_str());
    let bridge_contract = Bridge::new(contract_address.unwrap(), &provider);
    let current_available_balance: U256 = check_balance(token_to_distribute).await?;
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
pub async fn get_order_info(order_id: FixedBytes<32>) -> Result<Bridge::Order, Box<dyn Error>> {
    let provider = connect_static_evm_provider().await;
    let addr = std::env::var("BRIDGE_EVM_ADDR").expect("Contract addr must be set in .env");
    let contract_address = Address::from_str(addr.as_str());
    let bridge_contract = Bridge::new(contract_address.unwrap(), &provider);
    let order_info: Bridge::Order = bridge_contract.getOrderInfo(order_id).call().await?;
    println!("getInfo about order {order_id}");
    Ok(order_info)
}
