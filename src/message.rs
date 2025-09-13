use serde::{Deserialize, Serialize};

pub enum AsyncMessage {
    SubMessage(SubMessage),
    PubMesage(PubMessage),
}

pub enum SubMessage {
    OcrRequest(OcrSub),
    FeelInfoRequest(FeelInfoSub),
}

pub enum PubMessage {
    OcrPub(OcrPub),
    FeelInfoPub(FeelInfoPub),
}

// sub message 정의
#[derive(Deserialize, Debug)]
pub struct OcrSub {
    pub camera_id: String,
    pub time_stamp: u64,
    pub img: String,
}

#[derive(Deserialize, Debug)]
pub struct FeelInfoSub {
    pub license_plate: String,
    pub time_stamp: u64,
}

// pub message 정의
#[derive(Serialize, Debug)]
pub struct OcrPub {
    pub time_stamp: u64,
    pub ocr_data: Vec<u8>,
}

impl OcrPub {
    pub fn new(time_stamp: u64, ocr_data: Vec<u8>) -> Self {
        OcrPub {
            time_stamp,
            ocr_data,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct FeelInfoPub {
    license_plate: String,
    timestamp: u64,
}



