use alloy::providers::{ProviderBuilder, RootProvider};
use alloy::transports::http::{Client, Http};
use std::sync::OnceLock;

static PROVIDER: OnceLock<RootProvider<Http<Client>>> = OnceLock::new();

fn get_provider() -> &'static RootProvider<Http<Client>> {
    PROVIDER.get_or_init(|| {
        ProviderBuilder::new().on_http("https://bsc-dataseed.binance.org/".parse().unwrap())
    })
}
