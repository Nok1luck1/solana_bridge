use crate::solana::utils;
use anchor_client::{
    solana_sdk::signature::{read_keypair_file, Keypair},
    Client, Cluster,
};
use futures::{SinkExt, StreamExt};
use std::rc::Rc;
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::geyser::{
    subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
    SubscribeRequestFilterTransactions, SubscribeRequestPing,
};

pub async fn checking() -> Result<(), Box<dyn std::error::Error>> {
    let mut grpc_client = GeyserGrpcClient::build_from_static("http://127.0.0.1:10000")
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
    println!("Subscribed! Waiting for transactions...");

    while let Some(msg) = stream.next().await {
        match msg {
            Ok(update) => match update.update_oneof {
                Some(UpdateOneof::Transaction(tx_update)) => {
                    if let Some(tx_info) = tx_update.transaction {
                        let signature = bs58::encode(&tx_info.signature).into_string();
                        println!("New transaction: {}", signature);

                        if let Some(tx) = tx_info.transaction {
                            if let Some(msg) = tx.message {
                                println!("  Accounts: {}", msg.account_keys.len());
                                for ix in &msg.instructions {
                                    println!("  Instruction data (hex): {}", hex::encode(&ix.data));
                                }
                            }
                        }

                        if let Some(meta) = tx_info.meta {
                            for log in &meta.log_messages {
                                if log.contains("Program log:") {
                                    println!("  Log: {}", log);
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

    Ok(())
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
