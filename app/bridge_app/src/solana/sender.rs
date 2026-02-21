use anchor_client::{
    solana_sdk::{
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        system_program,
    },
    Client, Cluster,
};
use anchor_lang::{accounts, prelude::Pubkey};
use anchor_spl::associated_token;
use anyhow::Ok;
use bridge::{instruction, order};
use std::time::SystemTime;
use std::{rc::Rc, time::UNIX_EPOCH};

async fn execute_order(
    admin: Keypair,
    program: &anchor_client::Program<Rc<Keypair>>,
    token_mint: Pubkey,
    associated_token_account: Pubkey,
    amount0: u64,
    amount1: u64,
    token0: String,
    maker: String,
    receiver: Pubkey,
) -> Result<(), anyhow::Error> {
    let timeend = SystemTime::now().elapsed().unwrap().as_secs() as i64;
    let (order_id_pda, order_id) = utils::get_current_order_id(&program).await?;
    let (admin_config_pda, admin_config) = utils::get_admin_config(&program).await?;
    let (order_execution, bump_exec) = Pubkey::find_program_address(
        &[
            b"order_execution",
            admin.pubkey().as_array().as_ref(),
            &order_id.counter.to_le_bytes(),
        ],
        &bridge::ID,
    );
    let user_ata = utils::get_user_ata(token_mint, receiver).await?;
    let vault_ata = utils::get_token_vault(program, token_mint, associated_token_account).await?;
    let send_transaction = program
        .request()
        .accounts(bridge::accounts::ExecuteOrder {
            order_id: order_id_pda, // PDA order_id
            order_execution,        // PDA order_execution
            token_1_mint: token_mint,
            receiver_token_account: user_ata,
            vault_token_program: vault_ata,
            admin: admin.pubkey(),
            admin_config: admin_config_pda,
            token_program: token_mint,
            system_program: bridge::ID,
            associated_token_program: anchor_spl::associated_token::ID,
        })
        .args(bridge::instruction::OrderForExecution {
            receiver,
            token0amount: amount0,
            token1amount: amount1,
            token0,
            sender: maker,
            timestart: timeend,
        })
        .signer(admin)
        .send();
    Ok(())
}
