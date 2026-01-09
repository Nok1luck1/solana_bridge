use actix_web::{HttpResponse, Responder, get, web};
use alloy::providers::{
    Identity, Provider, ProviderBuilder, RootProvider,
    fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
};

#[get("/eth")]
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
    match provider.get_block_number().await {
        Ok(block) => HttpResponse::Ok().body(format!("Current block: {}", block)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
