use std::env;
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use tokio::sync::OnceCell;

pub static DB_POOL: OnceCell<bb8::Pool<ConnectionManager>> = OnceCell::const_new();

pub async fn init_db_pool() -> anyhow::Result<()> {
    let con_str = env::var("DB_CON_STR")?;
    let mgr = bb8_tiberius::ConnectionManager::build(con_str.as_str())?;

    let pool = Pool::builder()
        .max_size(2)
        .build(mgr)
        .await?;

    DB_POOL.set(pool)?;

    Ok(())
}

pub fn get_db_pool() -> &'static Pool<ConnectionManager> {
    DB_POOL.get().expect("DB Pool is not initialized")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    init_db_pool().await?;  // 어플리케이션 시작 시 초기화

    let pool = get_db_pool();
    let mut conn = pool.get().await?;

    // 쿼리 실행
    let query = format!(
        "SELECT TOP 5 parking_id, license_plate, entry_time FROM Parking.dbo.ParkingRecords ORDER BY entry_time DESC"
    );

    println!("\n쿼리 실행 중: {}", query);

    let stream = conn.query(
        query,
        &[]
    ).await?;

    let row = stream.into_row().await?;

    println!("{:?}", row);

    Ok(())
}