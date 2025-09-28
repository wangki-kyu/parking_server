use serde::{Deserialize, Serialize};

pub enum AsyncMessage {
    SubMessage(SubMessage),
    PubMesage(PubMessage),
}

pub enum SubMessage {
    OcrRequest(OcrSub),
    FeelInfoRequest(FeeInfoSub),
}

pub enum PubMessage {
    OcrPub(OcrPub),
    FeelInfoPub(FeeInfoPub),
}

// sub message 정의
#[derive(Deserialize, Debug)]
pub struct OcrSub {
    pub camera_id: String,
    pub timestamp: i64,
    pub img: String,
}

// todo 필드 변경 필요 
#[derive(Deserialize, Debug)]
pub struct FeeInfoSub {
    pub license_plate: String,
    pub entry_time: i64,
    pub exit_time: i64,
    pub fee: u64,
    pub is_paid: bool,
    pub discount_applied: String
}

// pub message 정의 
#[derive(Serialize, Debug, Default)]
pub struct FeeInfoPub {
    pub license_plate: String,
    pub entry_time: i64,
    pub exit_time: i64,
    pub fee: u64,
    pub is_paid: bool,
    pub discount_applied: String,
}

#[derive(Serialize, Debug)]
pub struct OcrPub {
    pub success: bool,
    pub license_plate: String,
    pub accuracy: f64,
    pub request_timestamp: u64,
}

impl OcrPub {
    pub fn new(success: bool, license_plate: String, accuracy: f64, request_timestamp: u64) -> Self {
        Self {
            success,
            license_plate,
            accuracy,
            request_timestamp,
        }
    }
}







