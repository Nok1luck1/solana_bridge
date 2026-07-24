use serde::{Deserialize, Serialize};

pub fn detect_network(address: &str) -> Option<Network> {
    match address.len() {
        42 if address.starts_with("0x") && address[2..].chars().all(|c| c.is_ascii_hexdigit()) => {
            Some(Network::Ethereum)
        }
        32..=44 if bs58::decode(address).into_vec().is_ok() => Some(Network::Solana),

        _ => None,
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    Ethereum,
    Solana,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
}
