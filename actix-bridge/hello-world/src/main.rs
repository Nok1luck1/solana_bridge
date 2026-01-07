pub(crate) mod constants;
pub mod eth;
pub mod routes;
pub mod solana;

use actix_web::{App, HttpServer, web};
use alloy::providers::ProviderBuilder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bnbprovider = ProviderBuilder::new()
        .connect(eth::constant::bnbprovideraddr)
        .await
        .ok();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(bnbprovider.clone()))
            .service(routes::get_latest_block)
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
