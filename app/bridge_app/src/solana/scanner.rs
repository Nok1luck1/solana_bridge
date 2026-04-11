use crate::{solana::decoder, types::OrderFormatter};

use futures::{SinkExt, StreamExt};
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::geyser::{
    subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
    SubscribeRequestFilterTransactions, SubscribeRequestPing,
};
pub async fn scan_for_order_sol(
) -> Result<Option<(OrderFormatter, Vec<u8>)>, Box<dyn std::error::Error>> {
    let grpc_address: &'static str = Box::leak(
        std::env::var("GRPC_ADDRESS")
            .unwrap_or_else(|_| "http://127.0.0.1:10000".to_owned())
            .into_boxed_str(),
    );

    let mut grpc_client = GeyserGrpcClient::build_from_static(grpc_address)
        .connect()
        .await?;

    let transactions = make_transactions_filter(&bridge::ID.to_string());

    let request = SubscribeRequest {
        transactions,
        commitment: Some(CommitmentLevel::Confirmed as i32),
        ..Default::default()
    };

    println!("must be started");
    let (mut subscribe_tx, mut stream) = grpc_client.subscribe_with_request(Some(request)).await?;
    println!("Subscribed,,, Waiting for transactions");

    while let Some(msg) = stream.next().await {
        match msg {
            Ok(update) => match update.update_oneof {
                Some(UpdateOneof::Transaction(tx_update)) => {
                    if let Some(tx_info) = tx_update.transaction {
                        let signature = bs58::encode(&tx_info.signature).into_string();
                        println!("New transaction in program {}", signature);
                        if let Some(meta) = tx_info.meta {
                            for log in &meta.log_messages {
                                if let Some(data_b64) = log.strip_prefix("Program data: ") {
                                    match base64::decode(data_b64.trim()) {
                                        Ok(raw_bytes) => {
                                            let check = decoder::decode(&raw_bytes);
                                            return Ok(check);
                                        }
                                        Err(e) => println!("Base64 decode error: {}", e),
                                    }
                                }
                            }
                        }
                    }
                }
                Some(UpdateOneof::Ping(_)) => {
                    subscribe_tx
                        .send(SubscribeRequest {
                            ping: Some(SubscribeRequestPing { id: 1 }),
                            ..Default::default()
                        })
                        .await?;
                }
                _ => {}
            },
            Err(e) => {
                eprintln!("Stream error: {:?}", e);
                break;
            }
        }
    }

    Ok(None)
}

fn make_transactions_filter(
    program_id: &str,
) -> std::collections::HashMap<String, SubscribeRequestFilterTransactions> {
    let mut map = std::collections::HashMap::new();
    map.insert(
        "my_program_filter".to_string(),
        SubscribeRequestFilterTransactions {
            vote: Some(false),
            failed: Some(false),
            signature: None,
            account_include: vec![program_id.to_string()],
            account_exclude: vec![],
            account_required: vec![],
        },
    );
    map
}
