use anchor_client::{
    solana_sdk::{
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        system_program,
    },
    Client, Cluster,
};
use anchor_lang::prelude::Pubkey;
use bridge::{order, program::Bridge, OrderId};

pub fn get_order_id(program: Program<Rc<Keypair>>) -> OrderId {
    let (order_id_pda, bump) = Pubkey::find_program_address(&[b"order_id"], &bridge::ID);
    println!("{:?}", order_id_pda);
    let order_id_account: bridge::OrderId = program.account(order_id_pda)?;
    order_id_account
}
