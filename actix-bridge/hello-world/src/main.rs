pub(crate) mod constants;
pub mod eth;
pub mod routes;
pub mod solana;

use actix_web::{App, HttpServer, web};
use alloy::providers::ProviderBuilder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let provider = ProviderBuilder::new()
        .connect("https://bsc-dataseed.binance.org/")
        .await
        .expect("Failed to connect to BSC");

    let provider_data = web::Data::new(provider);
    HttpServer::new(move || {
        App::new()
            .app_data(provider_data.clone()) // Клонируется только Arc
            .route("/block", web::get().to(get_block))
            .route("/balance/{address}", web::get().to(get_balance))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
