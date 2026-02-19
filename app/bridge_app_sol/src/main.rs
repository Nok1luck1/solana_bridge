pub mod sender;
pub mod utils;

use std::rc::Rc;

use anchor_client::{
    solana_sdk::signature::{read_keypair_file, Keypair},
    Client, Cluster,
};
use anchor_lang::prelude::Pubkey;
use bridge::{instruction, OrderId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let payer = read_keypair_file("../../bridge/tests/keys/admin1.json")?;
    let client = Client::new(Cluster::Localnet, Rc::new(payer));
    //let ws_client = PubsubClient::
    // print!("connection ws ,{:?}", ws_client);
    let program: anchor_client::Program<Rc<Keypair>> = client.program(bridge::ID)?;
    let admin = utils::get_admin_config(&program).await?;
    let order_id = utils::get_current_order_id(&program).await?;

    Ok(())
}
