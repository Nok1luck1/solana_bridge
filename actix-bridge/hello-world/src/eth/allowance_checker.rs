use actix_web::{HttpResponse, Responder, body::MessageBody, get, web};
use alloy::{
    primitives::Address,
    providers::{
        Identity, Provider, RootProvider,
        fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
    },
};

use crate::eth::ERC20;

#[get("/check_allowance")]
async fn check_allowance(
    token_addr: Address,
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
    let token = ERC20::new(token_addr, provider.clone());

    match provider.get_block_number().await {
        Ok(block) => HttpResponse::Ok().body(format!("Current block: {}", block)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
