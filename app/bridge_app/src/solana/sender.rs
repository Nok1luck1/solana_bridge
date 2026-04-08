use crate::solana::get_solana_provider;
use crate::utils;
use anchor_client::solana_sdk::{
    signature::{Keypair, Signature},
    signer::Signer,
};
use anchor_lang::prelude::Pubkey;

use anyhow::Ok;

use std::time::SystemTime;

pub async fn execute_order(
    _time_started: i64,
    _time_executed: i64,
    token0: Pubkey,
    token1: String,
    amount0: i64,
    amount1: i64,
    sender: String,
    receiver: Pubkey,
) -> Result<Signature , anyhow::Error> {
    let admin_keypair: Keypair =
        Keypair::from_base58_string(std::env::var("ADMIN_KEYPAIR").unwrap().as_str());
    let program = get_solana_provider();
    let timeend = SystemTime::now().elapsed().unwrap().as_secs() as i64;
    let (order_id_pda, order_id) = utils::get_current_order_id().await?;
    let (admin_config_pda, _admin_config) = utils::get_admin_config().await?;
    let (order_execution, _bump_exec) = Pubkey::find_program_address(
        &[
            b"order_execution",
            admin_keypair.pubkey().as_array().as_ref(),
            &order_id.counter.to_le_bytes(),
        ],
        &bridge::ID,
    );
    let user_ata = utils::get_user_ata(token0, receiver).await?;
    let vault_ata = utils::get_token_vault(token0, user_ata).await?;
    let _send_transaction = program
        .await
        .request()
        .accounts(bridge::accounts::ExecuteOrder {
            order_id: order_id_pda,
            order_execution,
            token_1_mint: token0,
            receiver_token_account: user_ata,
            vault_token_program: vault_ata,
            admin: admin_config_pda,
            admin_config: admin_config_pda,
            token_program: token0,
            system_program: bridge::ID,
            associated_token_program: anchor_spl::associated_token::ID,
        })
        .args(bridge::instruction::OrderForExecution {
            receiver: receiver,
            token0amount: amount0 as u64,
            token1amount: amount1 as u64,
            token0: token1,
            sender: sender,
            timestart: timeend,
        })
        .signer(admin_keypair)
        .send()
        .await?;
    Ok(_send_transaction )
}
