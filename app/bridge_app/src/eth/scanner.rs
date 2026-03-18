use crate::eth::{self};
use alloy::primitives::FixedBytes;
use alloy::sol_types::SolEvent;
use alloy::{
    eips::BlockNumberOrTag,
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::Filter,
};
use futures::StreamExt;
use std::{error::Error, str::FromStr};

pub async fn scan_for_orders() -> Result<Option<FixedBytes<32>>, Box<dyn Error>> {
    let rpc_url = std::env::var("WEBSOCKET_EVM").expect("WS url missing");
    let ws_connect = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().connect_ws(ws_connect).await?;
    let contract_addr_env = std::env::var("BRIDGE_EVM_ADDR").expect("env bridge addr missing");
    let contract = Address::from_str(&contract_addr_env.as_str()).unwrap();
    let filter = Filter::new()
        .address(contract)
        .event("OrderCreated(bytes32)")
        .from_block(BlockNumberOrTag::Latest);
    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();
    while let Some(log) = stream.next().await {
        println!("Geting some orders {log:?}");

        match eth::constant::Bridge::OrderCreated::decode_raw_log(
            log.topics().into_iter(),
            &log.data().data,
        ) {
            Ok(decoded) => {
                let order_id: alloy::primitives::FixedBytes<32> = decoded.orderId;
                println!("Decoded OrderCreated:");
                println!("order_id: 0x{}", hex::encode(order_id));
                return Ok(Some(order_id));
            }
            Err(e) => {
                println!("Decode failed: {e}");
            }
        }
    }
    Ok(None)
}
