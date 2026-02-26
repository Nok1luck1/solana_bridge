use anchor_client::{
    solana_sdk::{
        signature::{read_keypair_file, Keypair},
        signer::Signer,
    },
    Client, Cluster,
};
use anchor_lang::prelude::Pubkey;
use anchor_spl::associated_token::{self, get_associated_token_address};
use anyhow::Ok;
use bridge::{instruction, order, AdminConfig, Order, OrderId};
use std::rc::Rc;

pub async fn get_current_order_id(
    program: &anchor_client::Program<Rc<Keypair>>,
) -> Result<(Pubkey, OrderId), anyhow::Error> {
    let (order_id_pda, _) = Pubkey::find_program_address(&[b"order_id"], &bridge::ID);
    println!("{:?}", order_id_pda);
    let order_id_account: bridge::OrderId = program.account(order_id_pda).await?;
    println!(
        "{:?},{:?},order counter and bump",
        order_id_account.counter, order_id_account.bump
    );
    Ok((order_id_pda, order_id_account))
}
pub async fn get_admin_config(
    program: &anchor_client::Program<Rc<Keypair>>,
) -> Result<(Pubkey, AdminConfig), anyhow::Error> {
    let (admin_config_pda, _) = Pubkey::find_program_address(&[b"adminconfig"], &bridge::ID);
    println!("{:?}", admin_config_pda);
    let admin_config_account: bridge::AdminConfig = program.account(admin_config_pda).await?;
    println!(
        "{:?},{:?},admins and setttet",
        admin_config_account.admins, admin_config_account.settet
    );
    Ok((admin_config_pda, admin_config_account))
}
pub async fn get_specific_order(
    program: &anchor_client::Program<Rc<Keypair>>,
    user: Pubkey,
    order_counter: u64,
) -> Result<(Pubkey, bridge::Order), anyhow::Error> {
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
    Ok((order_pda, order_account))
}
pub async fn get_token_vault(
    _program: &anchor_client::Program<Rc<Keypair>>,
    token_mint: Pubkey,
    associated_token_account: Pubkey,
) -> Result<Pubkey, anyhow::Error> {
    let (admin_config_pda, _) = Pubkey::find_program_address(&[b"adminconfig"], &bridge::ID);
    let vault_account = associated_token::get_associated_token_address_with_program_id(
        &admin_config_pda,
        &token_mint,
        &associated_token_account,
    );
    Ok(vault_account)
}
pub async fn get_user_ata(token_mint: Pubkey, user: Pubkey) -> Result<Pubkey, anyhow::Error> {
    let user_ata = get_associated_token_address(&user, &token_mint);
    Ok(user_ata)
}
