use std::rc::Rc;

use anchor_client::{
    solana_sdk::{
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        system_program,
    },
    Client, Cluster,
};
use bridge::{accounts, instruction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let payer = read_keypair_file("../../bridge/tests/keys/admin1.json")?;
    let client = Client::new(Cluster::Localnet, Rc::new(payer));
    let program = client.program(bridge::ID)?;
    //print!("{}", program.);
    Ok(())
}
