mod mqtt;
pub mod message;

use std::{env, process};
use std::time::Duration;
use tokio::join;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task::spawn_blocking;
use mqtt::run_mqtt;
use message::SubMessage;
use message::AsyncMessage;

// The topics to which we subscribe.
const TOPICS: &[&str] = &["test/#", "hello"];

#[tokio::main]
async fn main() {
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtt://localhost:1883".to_string());

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<AsyncMessage>();

    tokio::join!(
        run_mqtt(host, tx),
        run_async_task(rx),
    );
}

async fn run_async_task(mut rx: UnboundedReceiver<AsyncMessage>) {
    // async worker
    let _ = tokio::spawn(async move {
        println!("start async receiver");
        loop {
            let Some(msg) = rx.recv().await else {
                println!("what!?");
                continue;
            };

            match msg {
                AsyncMessage::SubMessage(req) => {
                    match req {
                        SubMessage::OcrRequest(ocr) => {
                            println!("ocr msg recv");
                        }
                        SubMessage::FeelInfoRequest(feel_info) => {
                            println!("feel_info msg recv");
                        }
                    }
                }
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