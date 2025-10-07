use serde::{Deserialize, Serialize};

pub enum AsyncMessage {
    SubMessage(SubMessage),
    PubMesage(PubMessage),
}

pub enum SubMessage {
    OcrRequest(OcrSub),
    FeeInfoRequest(FeeInfoSub),
}

#[derive(Debug)]
pub enum PubMessage {
    OcrPub(OcrPub),
    FeeInfoPub(FeeInfoPub),
}

// sub message 정의
#[derive(Deserialize, Debug)]
pub struct OcrSub {
    pub gate_id: i32,
    pub timestamp: i64,
    pub img: String,
}

// todo 필드 변경 필요 
#[derive(Deserialize, Debug)]
pub struct FeeInfoSub {
    pub license_plate: String,
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
    pub gate_id: i32,
    pub success: bool,
    pub license_plate: String,
    pub accuracy: f64,
    pub request_timestamp: u64,
}

impl OcrPub {
    pub fn new(gate_id: i32, success: bool, license_plate: String, accuracy: f64, request_timestamp: u64) -> Self {
        Self {
            gate_id,
            success,
            license_plate,
            accuracy,
            request_timestamp,
        }
    }
}







