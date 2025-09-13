use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::message::PubMessage;
use crate::message::OcrPub;


pub async fn run_ocr_task(mut rx: UnboundedReceiver<String>, tx_pub: UnboundedSender<PubMessage>) {
    loop {
        let Some(data) = rx.recv().await else {
            println!("fail to receive ocr data");
            continue;
        };

        // ocr 인식을 한 뒤, tx_pub으로 메시지를 보내준다.
        let ocr_pub = OcrPub {
            time_stamp: 123123,
            ocr_data: data.into_bytes(),
        };
        let message = PubMessage::OcrPub(ocr_pub);

        tx_pub.send(message).unwrap();
    }
}