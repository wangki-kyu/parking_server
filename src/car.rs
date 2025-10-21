use std::collections::HashMap;
use anyhow::{anyhow, Ok};
use chrono::{FixedOffset, TimeZone, Utc};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::message::FeeInfoPub;

/// key: 번호판
/// value: CarInfo
/// 차량이 주차장에 들어왔을 때 추가
/// 정산 요청이 왔을 때 정산 응답 보내준 뒤 삭제
static CAR_INFO_MAP: Lazy<Mutex<HashMap<String, CarInfo>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

const FEE_PRICE_PER_MIN: u64 = 50;

#[derive(Debug, Default)]
struct CarInfo {
    enter_time: Option<i64>,
    exit_time: Option<i64>,
}

impl CarInfo {
    
}

/// 차량 추가 로직 
pub async fn add_car(license_plate: String) {
    let now = Utc::now();
    let time_stamp = now.timestamp();
    let utc_datetime = Utc.timestamp_opt(time_stamp, 0).unwrap();

    // 2. 한국 시간대로 변환
    let kst_offset = FixedOffset::east_opt(9*3600).unwrap();
    let kst_datetime = utc_datetime.with_timezone(&kst_offset);
    let formatted_kst = kst_datetime.format("%Y년 %m월 %d일 %H:%M").to_string();

    let car_info = CarInfo {
        enter_time: Some(time_stamp),
        exit_time: None,
    };

    println!("차량 번호: {}, 출입 시간: {}", license_plate, formatted_kst);

    let mut map = CAR_INFO_MAP.lock().await;
    map.insert(license_plate, car_info);
}

/// 차량 정산 로직 
pub async fn calculate_parking_fee(license_plate: String) -> anyhow::Result<FeeInfoPub> {
    let mut map = CAR_INFO_MAP.lock().await;
    println!("{:?}", map);
    let Some(car_info) = map.get(&license_plate) else {
        return Err(anyhow!("fail to get value by key [{}]", license_plate));
    };

    let Some(entry_time) = car_info.enter_time else {
        return Err(anyhow!("there is no enter time car [{}]", license_plate));
    };

    // calc fee 
    let exit_time = Utc::now().timestamp();
    let total_in_time = exit_time - entry_time;
    
    let fee = total_in_time * FEE_PRICE_PER_MIN as i64;
    println!("총 주차 시간: {} 분입니다. 총 요금은 {}원 입니다.", total_in_time / 60 , fee);
    let fee_info_pub = FeeInfoPub {
        license_plate,
        exit_time,
        entry_time,
        fee: fee as u64, 
        is_paid: false,
        discount_applied: "NONE".to_string(),
    };

    Ok(fee_info_pub)
}

#[cfg(test)]
mod car_test {
    use std::time::Duration;

    use super::*;

    #[tokio::test]
    async fn calc_fee_test() {
        let car_plate = "123가7890".to_string();
        add_car(car_plate.clone()).await;
        tokio::time::sleep(Duration::from_secs(70)).await;
        let _ = calculate_parking_fee(car_plate.clone()).await;
    }
}