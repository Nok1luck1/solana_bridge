use crate::entity;
use crate::orders;
use entity::orders::Entity as OrdersEntity;
use sea_orm::IntoActiveModel;
use sea_orm::{
    prelude::Decimal, ActiveModelTrait, ActiveValue, Database, DatabaseConnection, DbErr, Set,
};
use sea_orm::{EntityTrait, QueryFilter};
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
pub async fn save_order(
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
    hashsol: String,
    hashevm: String,
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
