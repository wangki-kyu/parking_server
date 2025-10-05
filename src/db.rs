use std::env;
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use tokio::sync::OnceCell;

pub static DB_POOL: OnceCell<bb8::Pool<ConnectionManager>> = OnceCell::const_new();

pub enum DBMessage {
    InsertCar(InsertCarData),
    UpdateCar(UpdateCarData),
}

pub struct InsertCarData {
    pub license_plate: String,
    pub entry_time: i64,
}

pub struct UpdateCarData {
    pub license_plate: String,
    pub exit_time: i64,
    pub is_paid: bool,
    pub discount_applied: String
}


pub async fn init_db_pool() -> anyhow::Result<()> {
    let con_str = env::var("DB_CON_STR")?;
    let mgr = bb8_tiberius::ConnectionManager::build(con_str.as_str())?;

    let pool = Pool::builder()
        .max_size(2)
        .build(mgr)
        .await?;

    DB_POOL.set(pool)?;

    println!("success db init!");

    Ok(())
}

pub fn get_db_pool() -> &'static Pool<ConnectionManager> {
    DB_POOL.get().expect("DB Pool is not initialized")
}

// insert car data
pub async fn insert_entry_car(data: InsertCarData) {
    let mut conn = get_db_pool().get().await.unwrap();
    let query = r#"
        INSERT INTO PARKING.dbo.ParkingRecords (license_plate, entry_time, is_paid, is_discounted)
        VALUES (@p1, @p2, 0, 0)
    "#;

    // 단일 쿼리이므로 트랜잭션을 처리하지 않는다.
    conn.execute(query, &[&data.license_plate, &data.entry_time]).await.unwrap();

}

// update car data
pub async fn update_exit_car(data: UpdateCarData) {
    let mut conn = get_db_pool().get().await.unwrap();
    let query = r#"
        UPDATE PARKING.dbo.ParkingRecords
        SET
            exit_time = @p1,
            is_paid = @p2,
            is_discounted = @p3
        WHERE
            license_plate = @p4
    "#;

    // 단일 쿼리이므로 트랜잭션을 처리하지 않는다.
    conn.execute(query, &[&data.exit_time, &data.is_paid, &data.discount_applied, &data.license_plate]).await.unwrap();
}