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
use anchor_spl::associated_token;
use anyhow::Ok;
use bridge::{instruction, order, AdminConfig, Order, OrderId};

pub async fn get_current_order_id(
    program: &anchor_client::Program<Rc<Keypair>>,
) -> Result<OrderId, anyhow::Error> {
    let (order_id_pda, _) = Pubkey::find_program_address(&[b"order_id"], &bridge::ID);
    println!("{:?}", order_id_pda);
    let order_id_account: bridge::OrderId = program.account(order_id_pda).await?;
    Ok(order_id_account)
}
pub async fn get_admin_config(
    program: &anchor_client::Program<Rc<Keypair>>,
) -> Result<AdminConfig, anyhow::Error> {
    let (admin_config_pda, _) = Pubkey::find_program_address(&[b"adminconfig"], &bridge::ID);
    println!("{:?}", admin_config_pda);
    let admin_config_account: bridge::AdminConfig = program.account(admin_config_pda).await?;
    Ok(admin_config_account)
}
pub async fn get_specific_order(
    program: &anchor_client::Program<Rc<Keypair>>,
    user: Pubkey,
    order_counter: u64,
) -> Result<Order, anyhow::Error> {
    let (order_pda, _) = Pubkey::find_program_address(
        &[b"order", user.as_ref(), &order_counter.to_le_bytes()],
        &bridge::ID,
    );
    println!("{:?}", order_pda);
    let order_account: bridge::Order = program.account(order_pda).await?;
    println!(
        "{:?},{:?},{:?},",
        order_account.id, order_account.maker, order_account.timestart
    );
    Ok(order_account)
}
pub async fn get_token_vault(
    program: &anchor_client::Program<Rc<Keypair>>,
    token_mint: Pubkey,
    token_program_id: Pubkey,
) -> Result<Pubkey, anyhow::Error> {
    let (admin_config_pda, _) = Pubkey::find_program_address(&[b"adminconfig"], &bridge::ID);
    let vault_account = associated_token::get_associated_token_address_with_program_id(
        &admin_config_pda,
        &token_mint,
        &token_program_id,
    );
    Ok(vault_account)
}
