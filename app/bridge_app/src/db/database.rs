use crate::entity;
use crate::entity::orders::Relation::Users1;
use crate::entity::users;
use crate::errors;
use crate::orders;
use crate::orders::Column;
use crate::types::OrderFormatter;
use alloy::primitives::Address;
use anchor_lang::prelude::Pubkey;
use entity::orders::Entity as OrdersEntity;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;

use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
use sea_orm::{
    prelude::Decimal, ActiveModelTrait, ActiveValue, Database, DatabaseConnection, DbErr, Set,
};
use tokio::sync::OnceCell;
use yellowstone_grpc_proto::geyser::subscribe_request_filter_accounts_filter_memcmp::Data;

static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn connect_static_db() -> &'static DatabaseConnection {
    DB.get_or_init(|| async {
        Database::connect(std::env::var("DATABASE_URL").expect("DB URL NOT SET"))
            .await
            .expect("DB connect failed")
    })
    .await
}
pub async fn create_order(
    id: i32,
    fromevm: bool,
    maker: String,
    receiver: String,
    token0: String,
    token1: String,
    amount0: i64,
    amount1: i64,
    timestart: i64,
    timeend: i64,
    _hashsol: String,
    _hashevm: String,
) -> Result<(), DbErr> {
    let database = connect_static_db().await;
    let (maker_id, receiver_id): (i32, i32) = if fromevm {
        (
            get_user_id_by_address_evm(maker).await? as i32,
            get_user_id_by_address_solana(receiver).await? as i32,
        )
    } else {
        (
            get_user_id_by_address_solana(maker).await? as i32,
            get_user_id_by_address_evm(receiver).await? as i32,
        )
    };
    let create_order = orders::ActiveModel {
        fromevmtosol: Set(fromevm),
        maker: Set(maker_id),
        receiver: Set(receiver_id),
        token0: Set(token0),
        token1: Set(token1),
        token0amount: Set(amount0),
        token1amount: Set(amount1),
        timestart: Set(timestart),
        timeendl: Set(timeend),
        tx_hash_solana: ActiveValue::NotSet,
        tx_hash_evm: ActiveValue::NotSet,
        id: Set(id),
    };
    let check = create_order.insert(database).await?;
    println!("Inserted: {:?}", check);
    Ok(())
}
pub async fn update_order_with_hash_evm(order_id: i32, hashevm: String) -> Result<(), DbErr> {
    let database = connect_static_db().await;
    if let Some(order) = OrdersEntity::find_by_id(order_id).one(database).await? {
        let mut active = order.into_active_model();
        active.tx_hash_evm = Set(Some(hashevm));
        active.update(database).await?;
        println!("added hash evm for order {:?}", order_id);
    }
    Ok(())
}
pub async fn update_order_with_hash_sol(order_id: i32, hashsolan: String) -> Result<(), DbErr> {
    let database = connect_static_db().await;
    if let Some(order) = OrdersEntity::find_by_id(order_id).one(database).await? {
        let mut active = order.into_active_model();
        active.tx_hash_solana = Set(Some(hashsolan));
        active.update(database).await?;
        println!("added hash solana for order {:?}", order_id);
    }
    Ok(())
}
pub async fn get_users_orders(limit: u64, offset: u64) -> Result<Vec<OrderFormatter>, DbErr> {
    let database = connect_static_db().await;
    let orders = OrdersEntity::find()
        .order_by_asc(Column::Id)
        .offset(offset)
        .limit(limit)
        .all(database)
        .await?;
    let result: Vec<OrderFormatter> = orders
        .into_iter()
        .map(|order| OrderFormatter::from_db_to_formatet(order))
        .collect();
    Ok(result)
}
pub async fn get_user_id_by_address_evm(user_address_evm: String) -> Result<i64, DbErr> {
    let database = connect_static_db().await;

    let evm_user = users::Entity::find()
        .filter(users::Column::AddressEvm.eq(user_address_evm))
        .one(database)
        .await?
        .unwrap();

    Ok(evm_user.id as i64)
}
pub async fn get_user_id_by_address_solana(user_address_sol: String) -> Result<i64, DbErr> {
    let database = connect_static_db().await;

    let solana_user = users::Entity::find()
        .filter(users::Column::AddressEvm.eq(user_address_sol))
        .one(database)
        .await?
        .unwrap();

    Ok(solana_user.id as i64)
}
// pub async fn get_user_orders(limit: u64, offset: u64) -> Result<Vec<OrderFormatter>, DbErr> {
//     let database = connect_static_db().await;
//     let orders = OrdersEntity::find()
//         .order_by_asc(Column::Address)
//         .offset(offset)
//         .limit(limit)
//         .all(database)
//         .await?;
//     let result: Vec<OrderFormatter> = orders
//         .into_iter()
//         .map(|order| OrderFormatter::from_db_to_formatet(order))
//         .collect();
//     Ok(result)
// }//Add new table into database that will create users list
