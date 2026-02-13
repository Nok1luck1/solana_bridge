use std::rc::Rc;

use anchor_client::{
    solana_sdk::{
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        system_program,
    },
    Client, Cluster,
};
use anchor_lang::prelude::Pubkey;
use bridge::{instruction, order, AdminConfig, OrderId};

pub async fn get_order_id(
    program: &anchor_client::Program<Rc<Keypair>>,
) -> Result<(OrderId), Box<dyn std::error::Error>> {
    let (order_id_pda, bump) = Pubkey::find_program_address(&[b"order_id"], &bridge::ID);
    println!("{:?}", order_id_pda);
    let order_id_account: bridge::OrderId = program.account(order_id_pda).await?;
    Ok(order_id_account)
}
pub async fn get_admin_config(
    program: &anchor_client::Program<Rc<Keypair>>,
) -> Result<(AdminConfig), Box<dyn std::error::Error>> {
    let (admin_config_pda, bump) = Pubkey::find_program_address(&[b"admin_config"], &bridge::ID);
    println!("{:?}", admin_config_pda);
    let admin_config_account: bridge::AdminConfig = program.account(admin_config_pda).await?;
    Ok(admin_config_account)
}
