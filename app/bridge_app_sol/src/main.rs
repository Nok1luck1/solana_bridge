pub mod sender;
pub mod utils;

use std::rc::Rc;

use anchor_client::{
    solana_sdk::{
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        system_program,
    },
    Client, Cluster, EventParser,
};
use anchor_lang::prelude::Pubkey;
use bridge::{instruction, OrderId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let payer = read_keypair_file("../../bridge/tests/keys/admin1.json")?;
    let client = Client::new(Cluster::Localnet, Rc::new(payer));
    //let ws_client = PubsubClient::new(Cluster::Localnet, Rc::new(payer));
    print!("connection ws ,{:?}", ws_client);
    let program: anchor_client::Program<Rc<Keypair>> = client.program(bridge::ID)?;
    let (admin_config_pda, _bump) = Pubkey::find_program_address(&[b"admin_config"], &bridge::ID);
    println!("{},pda admin config", admin_config_pda);
    let admin_config: bridge::AdminConfig = program.account(admin_config_pda).await?;
    //return empty because not inited yet
    println!("{:?},admin config admins", admin_config.admins);
    println!("{:?},admin config settet", admin_config.settet);
    println!("{:?},admin config bump", admin_config.bump);
    // let check = utils::get_order_id(&program).await?;
    // println!("{:?},{:?}", check.bump, check.counter);
    Ok(())
}
