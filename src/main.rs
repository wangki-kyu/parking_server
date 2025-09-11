mod mqtt;
pub mod message;

use std::{env, process};
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::spawn_blocking;
use mqtt::run_mqtt;
use message::SubMessage;
use message::AsyncMessage;
use crate::message::AsyncMessage::PubMesage;
use crate::message::{OcrPub, PubMessage};

// The topics to which we subscribe.
const TOPICS: &[&str] = &["test/#", "hello"];

#[tokio::main]
async fn main() {
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtt://localhost:1883".to_string());

    // create sub channel
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<AsyncMessage>();
    // create pub channel
    let (tx_pub, mut rx_pub) = tokio::sync::mpsc::unbounded_channel::<PubMessage>();

    tokio::join!(
        run_mqtt(host, tx, rx_pub),
        run_async_task(rx, tx_pub),
    );
}

                        async fn run_async_task(mut rx: UnboundedReceiver<AsyncMessage>, pub_tx: UnboundedSender<PubMessage>) {
    // async worker
    let _ = tokio::spawn(async move {
        println!("start async receiver");
        loop {
            let Some(msg) = rx.recv().await else {
                println!("what!?");
                continue;
            };

            // clone tx
            let clone_tx_pub = pub_tx.clone();

            match msg {
                AsyncMessage::SubMessage(req) => {
                    match req {
                        SubMessage::OcrRequest(ocr) => {
                            println!("ocr msg recv");
                            let data = "kimwoojun".to_string();
                            let ocr_message = OcrPub::new(123123, data.into_bytes());

                            clone_tx_pub.send(PubMessage::OcrPub(ocr_message)).unwrap();
                        }
                        SubMessage::FeelInfoRequest(feel_info) => {
                            println!("feel_info msg recv");
                        }
                    }
                }
                _ => {}
            }
        }
        while let Some(msg) = rx.recv().await {

        }
    }).await;
}

async fn ocr_task() {
    // todo here ..
    println!("ocr processing,,,");
}

// ------- 번호판 -------
// 1. 번호판을 Request 받음
// 2. OCR 객체인식을 하여 차량 번호 전송.

// ------- 차량 데이터 요청 -------
// request: 차량 데이터 요청
// response: 차량 데이터 응답

// ------- 정산 -----------
//`