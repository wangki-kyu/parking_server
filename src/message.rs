use serde::{Deserialize, Serialize};

pub enum AsyncMessage {
    SubMessage(SubMessage),
}

pub enum SubMessage {
    OcrRequest(OcrRequest),
    FeelInfoRequest(FeelInfoRequest),
}

// Request message 정의
#[derive(Deserialize)]
pub struct OcrRequest {
    camera_id: String,
    time_stamp: u64,
    img: String,
}

#[derive(Deserialize)]
pub struct FeelInfoRequest {
    license_plate: String,
    time_stamp: u64,
}


