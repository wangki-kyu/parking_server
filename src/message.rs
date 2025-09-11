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
#[derive(Deserialize)]
pub struct OcrSub {
    camera_id: String,
    time_stamp: u64,
    img: String,
}

#[derive(Deserialize)]
pub struct FeelInfoSub {
    license_plate: String,
    time_stamp: u64,
}

// pub message 정의
#[derive(Serialize)]
pub struct OcrPub {
    time_stamp: u64,
    ocr_data: Vec<u8>,
}

impl OcrPub {
    pub fn new(time_stamp: u64, ocr_data: Vec<u8>) -> Self {
        OcrPub {
            time_stamp,
            ocr_data,
        }
    }
}

#[derive(Serialize)]
pub struct FeelInfoPub {

}



