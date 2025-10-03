use std::env;
use dotenvy::dotenv;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
use once_cell::sync::Lazy;

static CONN_STR: Lazy<String> = Lazy::new(|| {
    env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
        env::var("DB_CON_STR").unwrap().into()
    })
});

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. .env 파일 로드
    dotenv().ok();
    println!("연결 시도 중...");

    let config = Config::from_ado_string(&CONN_STR)?;

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    // Tiberius 클라이언트 생성
    let mut client = Client::connect(config, tcp.compat_write()).await?;
    println!("MSSQL 연결 성공!");

    // 쿼리 실행
    let query = format!(
        "SELECT TOP 5 parking_id, license_plate, entry_time FROM Parking.dbo.ParkingRecords ORDER BY entry_time DESC"
    );

    println!("\n쿼리 실행 중: {}", query);

    let stream = client.query(query, &[]).await?;
    let row = stream.into_row().await?.unwrap();

    println!("{:?}", row);

    Ok(())
}