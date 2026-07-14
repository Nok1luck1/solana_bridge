use crate::entity;
use crate::entity::users;
use crate::errors::FormatError;
use crate::orders;
use crate::orders::Column;
use crate::types::OrderFormatter;
use entity::orders::Entity as OrdersEntity;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use std::sync::Arc;

use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection, DbErr, Set};
use tokio::sync::OnceCell;

static DB_POOL: OnceCell<Arc<DatabaseConnection>> = OnceCell::const_new();

pub async fn init_db_pool() {
    let pool = Database::connect(std::env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("Failed to create database pool");

    DB_POOL
        .set(Arc::new(pool))
        .expect("DB_POOL already initialized");
}

pub fn get_pool() -> Arc<DatabaseConnection> {
    DB_POOL
        .get()
        .expect("DB_POOL not initialized! Call init_db_pool() first")
        .clone()
}
pub async fn create_order(
    pool: &DatabaseConnection,
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
    let (maker_id, receiver_id): (i32, i32) = if fromevm {
        (
            get_user_id_by_address_evm(&pool, maker).await? as i32,
            get_user_id_by_address_solana(&pool, receiver).await? as i32,
        )
    } else {
        (
            get_user_id_by_address_solana(&pool, maker).await? as i32,
            get_user_id_by_address_evm(&pool, receiver).await? as i32,
        )
    };
    let create_order = orders::ActiveModel {
        id: NotSet,
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
    };
    let check = create_order.insert(pool).await?;
    println!("Inserted: {:?}", check);
    Ok(())
}
pub async fn create_user(
    pool: &DatabaseConnection,
    address: String,
    is_evm: bool,
) -> Result<(), DbErr> {
    if is_evm {
        let _create_user = users::ActiveModel {
            id: NotSet,
            address_evm: Set(address),
            address_solana: NotSet,
            blocked: Set(false),
        };
        let _insert = _create_user.insert(pool).await?;
    } else {
        let _create_user = users::ActiveModel {
            id: NotSet,
            address_evm: NotSet,
            address_solana: Set(address),
            blocked: Set(false),
        };
        let _insert = _create_user.insert(pool).await?;
    }

    Ok(())
}
pub async fn update_user_address(
    pool: &DatabaseConnection,
    user_id: i64,
    address_evm: String,
    address_sol: String,
) -> Result<(), DbErr> {
    if let Some(user) = users::Entity::find_by_id(user_id as i32).one(pool).await? {
        let mut active = user.into_active_model();
        active.address_evm = Set(address_evm.clone());
        active.address_solana = Set(address_sol.clone());
        active.update(pool).await?;
    }
    Ok(())
}
pub async fn update_order_with_hash_evm(
    pool: &DatabaseConnection,
    order_id: i32,
    hashevm: String,
) -> Result<(), DbErr> {
    if let Some(order) = OrdersEntity::find_by_id(order_id).one(pool).await? {
        let mut active = order.into_active_model();
        active.tx_hash_evm = Set(Some(hashevm));
        active.update(pool).await?;
    }
    Ok(())
}
pub async fn update_order_with_hash_sol(
    pool: &DatabaseConnection,
    order_id: i32,
    hashsolan: String,
) -> Result<(), DbErr> {
    if let Some(order) = OrdersEntity::find_by_id(order_id).one(pool).await? {
        let mut active = order.into_active_model();
        active.tx_hash_solana = Set(Some(hashsolan));
        active.update(pool).await?;
        println!("added hash solana for order {:?}", &order_id);
    }
    Ok(())
}
pub async fn get_users_made_orders(
    pool: &DatabaseConnection,
    user_address: String,
    in_evm: bool,
    make_or_receive: bool,
    limit: u64,
    offset: u64,
) -> Result<Vec<OrderFormatter>, DbErr> {
    let get_user_id: i64 = if in_evm {
        get_user_id_by_address_evm(&pool, user_address).await?
    } else {
        get_user_id_by_address_solana(&pool, user_address).await?
    };
    let orders_by_type = if make_or_receive {
        OrdersEntity::find()
            .filter(Column::Maker.eq(get_user_id))
            .offset(offset)
            .limit(limit)
            .all(pool)
            .await?
    } else {
        OrdersEntity::find()
            .filter(Column::Receiver.eq(get_user_id))
            .offset(offset)
            .limit(limit)
            .all(pool)
            .await?
    };
    let result: Vec<OrderFormatter> = orders_by_type
        .into_iter()
        .map(|order| OrderFormatter::from_db_to_formatet(order))
        .collect();
    Ok(result)
}
pub async fn get_user_id_by_address_evm(
    pool: &DatabaseConnection,
    user_address_evm: String,
) -> Result<i64, DbErr> {
    let evm_user_id = users::Entity::find()
        .filter(users::Column::AddressEvm.eq(user_address_evm))
        .one(pool)
        .await?
        .expect(&FormatError::DBError.to_string())
        .id as i64;

    Ok(evm_user_id)
}
pub async fn get_user_id_by_address_solana(
    pool: &DatabaseConnection,
    user_address_sol: String,
) -> Result<i64, DbErr> {
    let solana_user_id = users::Entity::find()
        .filter(users::Column::AddressEvm.eq(user_address_sol))
        .one(pool)
        .await?
        .expect(&FormatError::DBError.to_string())
        .id as i64;

    Ok(solana_user_id)
}
pub async fn check_blocked(pool: &DatabaseConnection, user_id: u64) -> Result<bool, DbErr> {
    let blocked = users::Entity::find()
        .filter(users::Column::Id.eq(user_id))
        .one(pool)
        .await?
        .expect(&FormatError::DBError.to_string())
        .blocked;
    Ok(blocked)
}
pub async fn block_user(pool: &DatabaseConnection, user_id: u64) -> Result<users::Model, DbErr> {
    if let Some(user) = users::Entity::find_by_id(user_id as i32).one(pool).await? {
        let mut active = user.into_active_model();
        active.blocked = Set(false);
        let updated_user = active.update(pool).await?;
        Ok(updated_user)
    } else {
        Err(DbErr::RecordNotFound(format!("User {} not found", user_id)))
    }
}
pub async fn get_spicific_order(
    pool: &DatabaseConnection,
    order_id: i32,
) -> Result<orders::Model, DbErr> {
    let order = orders::Entity::find()
        .filter(orders::Column::Id.eq(order_id))
        .one(pool)
        .await?
        .expect(&FormatError::DBError.to_string());
    Ok(order)
}
pub async fn get_all_evm_order(
    pool: &DatabaseConnection,
    offset: u64,
    limit: u64,
) -> Result<Vec<OrderFormatter>, DbErr> {
    let orders = OrdersEntity::find()
        .filter(orders::Column::Fromevmtosol.eq(true))
        .offset(offset)
        .limit(limit)
        .all(pool)
        .await?
        .into_iter()
        .map(|orders| OrderFormatter::from_db_to_formatet(orders))
        .collect();
    Ok(orders)
}
pub async fn get_all_solana_order(
    pool: &DatabaseConnection,
    offset: u64,
    limit: u64,
) -> Result<Vec<OrderFormatter>, DbErr> {
    let orders = OrdersEntity::find()
        .filter(orders::Column::Fromevmtosol.eq(false))
        .offset(offset)
        .limit(limit)
        .all(pool)
        .await?
        .into_iter()
        .map(|orders| OrderFormatter::from_db_to_formatet(orders))
        .collect();
    Ok(orders)
}
