use crate::OrderFormatter;
use anchor_client::solana_sdk::signature::{read_keypair_file, Keypair};
use anchor_client::Client;
use anchor_client::Cluster;
use anchor_lang::prelude::Pubkey;
use anchor_spl::associated_token::{self, get_associated_token_address};
use anchor_spl::token::TokenAccount;
use anyhow::Ok;
use bridge::{AdminConfig, OrderId};
use std::sync::Arc;
use tokio::sync::OnceCell;

static SOLANA_CLIENT: OnceCell<anchor_client::Program<Arc<Keypair>>> = OnceCell::const_new();

pub async fn get_solana_provider() -> &'static anchor_client::Program<Arc<Keypair>> {
    SOLANA_CLIENT
        .get_or_init(|| async {
            let _payer = read_keypair_file("../../../bridge/tests/keys/admin1.json").unwrap();
            let _client = Client::new(Cluster::Localnet, Arc::new(_payer));
            _client.program(bridge::ID).unwrap()
        })
        .await
}

pub async fn get_current_order_id() -> Result<(Pubkey, OrderId), anyhow::Error> {
    let program = get_solana_provider();
    let (order_id_pda, _) = Pubkey::find_program_address(&[b"order_id"], &bridge::ID);
    println!("{:?}", order_id_pda);
    let order_id_account: bridge::OrderId = program.await.account(order_id_pda).await?;
    println!(
        "{:?},{:?},order counter and bump",
        order_id_account.counter, order_id_account.bump
    );
    Ok((order_id_pda, order_id_account))
}
pub async fn get_admin_config() -> Result<(Pubkey, AdminConfig), anyhow::Error> {
    let program = get_solana_provider();
    let (admin_config_pda, _) = Pubkey::find_program_address(&[b"adminconfig"], &bridge::ID);
    println!("{:?}", admin_config_pda);
    let admin_config_account: bridge::AdminConfig = program.await.account(admin_config_pda).await?;
    println!(
        "{:?},{:?},admins and setttet",
        admin_config_account.admins, admin_config_account.settet
    );
    Ok((admin_config_pda, admin_config_account))
}
pub async fn get_specific_order(
    user: Pubkey,
    order_counter: u64,
) -> Result<(Pubkey, OrderFormatter), anyhow::Error> {
    let program = get_solana_provider();
    let (order_pda, _) = Pubkey::find_program_address(
        &[b"order", user.as_ref(), &order_counter.to_le_bytes()],
        &bridge::ID,
    );
    println!("{:?}", order_pda);
    let order_account: bridge::Order = program.await.account(order_pda).await?;
    println!(
        "{:?},{:?},{:?},",
        order_account.id, order_account.maker, order_account.timestart
    );
    let order_form = OrderFormatter::new(
        order_account.timestart,
        0,
        order_account.token0.to_string(),
        order_account.token1,
        order_account.token0amount,
        order_account.token1amount,
        order_account.maker.to_string(),
        order_account.receiver,
    );
    Ok((order_pda, order_form))
}
pub async fn get_token_vault(
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
//Getting ATA for user pubkey
pub async fn get_user_ata(token_mint: Pubkey, user: Pubkey) -> Result<Pubkey, anyhow::Error> {
    let user_ata = get_associated_token_address(&user, &token_mint);
    Ok(user_ata)
}
pub async fn get_vault_balance(token_mint: Pubkey) -> Result<u64, anyhow::Error> {
    let program = get_solana_provider().await;
    let (pubk, _) = get_admin_config().await?;
    let vault_ata = get_associated_token_address(&pubk, &token_mint);
    let vault_acc: TokenAccount = program.account(vault_ata).await?;
    Ok(vault_acc.amount)
}
