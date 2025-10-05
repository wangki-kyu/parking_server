mod mqtt;
pub mod message;
mod ocr;
mod car;
mod db;

use std::{env, process};
use std::time::Duration;
use dotenvy::dotenv;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::mpsc::error::SendError;
use tokio::task::spawn_blocking;
use mqtt::run_mqtt;
use message::SubMessage;
use message::AsyncMessage;
use crate::message::{OcrPub, OcrSub, PubMessage};
use ocr::run_ocr_task;
use crate::db::{init_db_pool, insert_entry_car, DBMessage};

#[derive(Clone)]
struct AsyncTxBundle {
    tx_pub: UnboundedSender<PubMessage>,
    tx_ocr: UnboundedSender<OcrSub>,
    tx_db: UnboundedSender<DBMessage>,
    tx_async: UnboundedSender<AsyncMessage>,
}

impl AsyncTxBundle {
    fn new(
        tx_pub: UnboundedSender<PubMessage>,
        tx_ocr: UnboundedSender<OcrSub>,
        tx_db: UnboundedSender<DBMessage>,
        tx_async: UnboundedSender<AsyncMessage>
    ) -> Self {
        Self {
            tx_pub,
            tx_ocr,
            tx_db,
            tx_async,
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_db_pool().await.expect("fail to initialize db");
    let mosquitto_url = env::var("MOSQUITTO_URL").unwrap();

    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| mosquitto_url);

    // create sub channel
    let (tx_async, rx_async) = tokio::sync::mpsc::unbounded_channel::<AsyncMessage>();
    // create pub channel
    let (tx_pub, rx_pub) = tokio::sync::mpsc::unbounded_channel::<PubMessage>();
    // create ocr channel
    let (tx_ocr, rx_ocr) = tokio::sync::mpsc::unbounded_channel::<OcrSub>();    // 추후에 bytes로 변경될 수 있음.
    // create db channel
    let (tx_db, rx_db) = tokio::sync::mpsc::unbounded_channel::<DBMessage>();    // 추후에 bytes로 변경될 수 있음.

    let tx_pub_cloned = tx_pub.clone();
    let async_tx_bundle = AsyncTxBundle::new(tx_pub.clone(), tx_ocr.clone(), tx_db.clone(), tx_async);
    tokio::join!(
        run_mqtt(host, async_tx_bundle.clone(), rx_pub),
        run_async_task(rx_async, async_tx_bundle.clone()),
        run_ocr_task(rx_ocr, tx_pub_cloned),
        db_async_worker(rx_db, async_tx_bundle.clone()),
    );
}

async fn run_async_task(mut rx: UnboundedReceiver<AsyncMessage>, tx_bundle: AsyncTxBundle) {
    // async worker
    println!("async worker start!");
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
                            match tx_ocr_clone.send(ocr) {
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

async fn db_async_worker(mut rx: UnboundedReceiver<DBMessage>, tx_bundle: AsyncTxBundle) {
    println!("db async worker start!");
    while let Some(db_msg) = rx.recv().await {
        let cloned_tx_bundle = tx_bundle.clone();
        match db_msg {
            DBMessage::InsertCar(entry_data) => {
                insert_entry_car(entry_data).await;
            }
            DBMessage::UpdateCar(exit_data) => {

            }
        }
    }
}

// ------- 번호판 -------
// 1. 번호판을 Request 받음
// 2. OCR 객체인식을 하여 차량 번호 전송.

// ------- 차량 데이터 요청 -------
// request: 차량 데이터 요청
// response: 차량 데이터 응답

// ------- 정산 -----------
//`