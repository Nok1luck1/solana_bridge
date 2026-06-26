use crate::entity;
use crate::orders;
use crate::orders::Column;
use crate::types::OrderFormatter;
use entity::orders::Entity as OrdersEntity;
use sea_orm::EntityTrait;

use sea_orm::IntoActiveModel;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
use sea_orm::{
    prelude::Decimal, ActiveModelTrait, ActiveValue, Database, DatabaseConnection, DbErr, Set,
};
use tokio::sync::OnceCell;

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
    let create_order = orders::ActiveModel {
        fromevmtosol: Set(fromevm),
        maker: Set(maker),
        receiver: Set(receiver),
        token0: Set(token0),
        token1: Set(token1),
        token0amount: Set(amount0),
        token1amount: Set(amount1),
        timestart: Set(Decimal::from(timestart)),
        timeendl: Set(Decimal::from(timeend)),
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
        active.tx_hash_evm = Set(hashevm);
        active.update(database).await?;
        println!("added hash evm for order {:?}", order_id);
    }
    Ok(())
}
pub async fn update_order_with_hash_sol(order_id: i32, hashsolan: String) -> Result<(), DbErr> {
    let database = connect_static_db().await;
    if let Some(order) = OrdersEntity::find_by_id(order_id).one(database).await? {
        let mut active = order.into_active_model();
        active.tx_hash_solana = Set(hashsolan);
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
