use std::{env, str::FromStr};

use actix_web::{HttpResponse, Responder, body::MessageBody, get, web};
use alloy::{
    primitives::{Address, address},
    providers::{
        Identity, Provider, ProviderBuilder, RootProvider,
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
    },
};

#[get("/execute")]
async fn get_block(
    provider: web::Data<
        FillProvider<
            JoinFill<
                Identity,
                JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
            >,
            RootProvider,
        >,
    >,
) -> HttpResponse {
    let addr = std::env::var("CONTRACT_ADDR").expect("Contract addr must be set in .env");
    let contract_address = Address::from_str(addr.as_str());

    match provider.get_block_number().await {
        Ok(block) => HttpResponse::Ok().body(format!("Current block: {}", block)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
