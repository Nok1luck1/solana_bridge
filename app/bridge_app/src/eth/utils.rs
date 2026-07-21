use crate::db::database;
use crate::errors::FormatError;
use crate::eth::Bridge;
use crate::eth::ERC20;
use crate::state;
use alloy::primitives::FixedBytes;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::providers::{fillers::JoinFill, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::Signature;
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
    let bridge_balance = token
        .balanceOf(contract_address.expect(&FormatError::BlockchainError.to_string()))
        .call()
        .await?;
    Ok(bridge_balance)
}

pub async fn execute_order_evm(
    address_receiver: Address,
    token_deposited: String,
    token_to_distribute: Address,
    address_sender: String,
    amount_deposited: u64,
    amount_to_distribute: U256,
) -> Result<FixedBytes<32>, Box<dyn Error>> {
    let provider = connect_static_evm_provider().await;
    let addr = std::env::var("BRIDGE_EVM_ADDR").expect(&FormatError::ParseError.to_string());
    let contract_address = Address::from_str(addr.as_str());
    let bridge_contract = Bridge::new(
        contract_address.expect(&FormatError::BlockchainError.to_string()),
        &provider,
    );
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
    let _distribute_tx = disctribute_token.send().await?.get_receipt().await?;

    Ok(_distribute_tx.transaction_hash)
}
pub async fn check_is_admin(address_admin: String) -> Result<bool, Box<dyn Error>> {
    let provider = connect_static_evm_provider().await;
    let addr = std::env::var("BRIDGE_EVM_ADDR").expect("Contract addr must be set in .env");
    let contract_address = Address::from_str(addr.as_str())?;
    let bridge_contract = Bridge::new(contract_address, &provider);
    let is_admin: bool = bridge_contract
        .hasRole(
            FixedBytes::ZERO,
            Address::from_str(address_admin.as_str())
                .expect(&FormatError::BlockchainError.to_string()),
        )
        .call()
        .await?;
    Ok(is_admin)
}
pub async fn check_interactions_with_program(
    user_addr: Address,
) -> Result<(bool, bool), Box<dyn Error>> {
    let pool = state::get_pool();
    let is_user_exists =
        database::get_user_id_by_address_evm(&pool, user_addr.to_string()).await? > 0;
    let provider = connect_static_evm_provider().await;
    let addr = std::env::var("BRIDGE_EVM_ADDR").expect("Contract addr must be set in .env");
    let contract_address = Address::from_str(addr.as_str());
    let _bridge_contract = Bridge::new(
        contract_address.expect(&FormatError::BlockchainError.to_string()),
        &provider,
    );
    let is_admin = check_is_admin(user_addr.to_string()).await?;
    Ok((is_user_exists, is_admin))
}
pub async fn get_order_info(order_id: U256) -> Result<Bridge::Order, Box<dyn Error>> {
    let provider = connect_static_evm_provider().await;
    let addr = std::env::var("BRIDGE_EVM_ADDR").expect("Contract addr must be set in .env");
    let contract_address = Address::from_str(addr.as_str());
    let bridge_contract = Bridge::new(
        contract_address.expect(&FormatError::BlockchainError.to_string()),
        &provider,
    );
    let order_info: Bridge::Order = bridge_contract.getOrderInfo(order_id.into()).call().await?;
    println!("getInfo about order {order_id}");
    Ok(order_info)
}
pub async fn get_address_nonce(address: String) -> Result<u64, Box<dyn Error>> {
    let provider = connect_static_evm_provider().await;
    let nonce = provider
        .get_transaction_count(Address::from_str(address.as_str())?)
        .pending()
        .await?;
    Ok(nonce)
}
pub async fn verify_message(
    message: String,
    signature: String,
    address: String,
) -> Result<bool, Box<dyn Error>> {
    let signature_owner =
        Signature::recover_address_from_msg(&Signature::from_str(&signature).unwrap(), message)
            .expect(&FormatError::DecoderError.to_string())
            .to_string();
    if Address::from_str(signature_owner.as_str()).unwrap()
        == Address::from_str(address.as_str()).unwrap()
    {
        return Ok(true);
    }
    Ok(false)
}
