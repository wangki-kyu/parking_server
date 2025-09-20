mod mqtt;
pub mod message;
mod ocr;
mod car;

use std::{env, process};
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::mpsc::error::SendError;
use tokio::task::spawn_blocking;
use mqtt::run_mqtt;
use message::SubMessage;
use message::AsyncMessage;
use crate::message::{OcrPub, PubMessage};
use ocr::run_ocr_task;


struct AsyncTxBundle {
    tx_pub: UnboundedSender<PubMessage>,
    tx_ocr: UnboundedSender<String>,
}

impl AsyncTxBundle {
    fn new(tx_pub: UnboundedSender<PubMessage>, tx_ocr: UnboundedSender<String>) -> Self {
        Self {
            tx_pub,
            tx_ocr,
        }
    }
}

#[tokio::main]
async fn main() {
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtt://localhost:1883".to_string());

    // create sub channel
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<AsyncMessage>();
    // create pub channel
    let (tx_pub, mut rx_pub) = tokio::sync::mpsc::unbounded_channel::<PubMessage>();
    // create ocr channel
    let (tx_ocr, mut rx_ocr) = tokio::sync::mpsc::unbounded_channel::<String>();    // 추후에 bytes로 변경될 수 있음.

    let tx_pub_cloned = tx_pub.clone();
    let async_tx_bundle = AsyncTxBundle::new(tx_pub.clone(), tx_ocr.clone());
    tokio::join!(
        run_mqtt(host, tx, rx_pub),
        run_async_task(rx, async_tx_bundle),
        run_ocr_task(rx_ocr, tx_pub_cloned),
    );
}

async fn run_async_task(mut rx: UnboundedReceiver<AsyncMessage>, tx_bundle: AsyncTxBundle) {
    // async worker
    let _ = tokio::spawn(async move {
        println!("start async receiver");
        loop {
            let Some(msg) = rx.recv().await else {
                println!("what!?");
                continue;
            };

            // clone tx
            let clone_tx_pub = tx_bundle.tx_pub.clone();
            let tx_ocr_clone = tx_bundle.tx_ocr.clone();

            match msg {
                AsyncMessage::SubMessage(req) => {
                    match req {
                        SubMessage::OcrRequest(ocr) => {
                            // ocr ?
                            match tx_ocr_clone.send(ocr.camera_id) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("{}", e);
                                }
                            }
                        }
                        SubMessage::FeelInfoRequest(fee_info) => {
                            // 1. ocr 처리 
                            // 2. ocr 에서 mqtt pub으로 번호를 보내주면 
                            // 정산 처리 관련 함수를 호출 해준다.
                            
                            println!("feel_info msg recv");
                        }
                    }
                }
                _ => {}
            }
        }

    }).await;
}

// ------- 번호판 -------
// 1. 번호판을 Request 받음
// 2. OCR 객체인식을 하여 차량 번호 전송.

// ------- 차량 데이터 요청 -------
// request: 차량 데이터 요청
// response: 차량 데이터 응답

// ------- 정산 -----------
//`