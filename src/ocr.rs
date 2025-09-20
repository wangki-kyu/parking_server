use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::message::{OcrPub, PubMessage};

pub async fn run_ocr_task(mut rx: UnboundedReceiver<String>, tx_pub: UnboundedSender<PubMessage>) {
    loop {
        let Some(data) = rx.recv().await else {
            println!("fail to receive ocr data");
            continue;
        };

        // ocr 인식을 한 뒤, tx_pub으로 메시지를 보내준다.
        let ocr_pub = OcrPub::new(true, "12가1234".to_string(), 84.2, 12312123);
        let message = PubMessage::OcrPub(ocr_pub);

        tx_pub.send(message).unwrap();
    }
}