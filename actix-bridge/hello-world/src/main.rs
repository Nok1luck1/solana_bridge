pub mod eth;
pub mod routes;
pub mod solana;

use actix_web::{App, HttpServer, web};
use alloy::{providers::ProviderBuilder, signers::local::PrivateKeySigner};

use crate::{eth::check_allowance, routes::get_block};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let provider_end_point = std::env::var("PROVIDER_BNB_ENDPOINT").expect("PROVIDER DID NOT SET");
    let private_key = std::env::var("PRIVATE_KEY_BNB").expect("private missing");
    let signer: PrivateKeySigner = private_key.parse().expect("private key parse error");
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(provider_end_point.as_str())
        .await
        .expect("Failed to connect to BSC");

    let provider_data = web::Data::new(provider);
    HttpServer::new(move || {
        App::new()
            .app_data(provider_data.clone())
            .service(get_block)
            .service(check_allowance)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
