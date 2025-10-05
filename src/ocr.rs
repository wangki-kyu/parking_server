use std::{env, fs::File, io::Write};

use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use tokio::{process::Command, sync::mpsc::{UnboundedReceiver, UnboundedSender}};
use crate::message::{OcrPub, OcrSub, PubMessage};

pub async fn run_ocr_task(mut rx: UnboundedReceiver<OcrSub>, tx_pub: UnboundedSender<PubMessage>) {
    loop {
        let Some(data) = rx.recv().await else {
            println!("fail to receive ocr data");
            continue;
        };
        
        let cloned_tx_pub = tx_pub.clone();
        // ocr py를 호출해주는 비동기 코드를 작성해야할 것 같음 
        // 만약에 여기서 대기한다면 동기적으로 처리하는거랑 다름이 없음.
        // 따라서 하나의 task를 생성하는게 좋아보임. 
        tokio::spawn(async move {
            call_ocr(data, cloned_tx_pub).await;
        });
    }
}

async fn call_ocr(ocr_sub: OcrSub, tx: UnboundedSender<PubMessage>) {
    // py path
    let py_path = "/app/py/ocr_script.py";    // docker /app 
    let image_path = "/app/license_plates/target.jpg";
    let mut file = File::create(image_path).unwrap();
    let decoding_image = general_purpose::STANDARD.decode(ocr_sub.img).unwrap();
    file.write_all(&decoding_image).unwrap();

    let output = Command::new("python")
        .arg(py_path)
        .arg("/app/license_plates/target.jpg")
        .output().await.unwrap();

    let res_str = String::from_utf8(output.stdout).unwrap();
    println!("{}", res_str);

    match tokio::fs::remove_file(&image_path).await {
        Ok(_) => println!("success to remove image file"),
        Err(e) => eprintln!("fail to remove file, e: {}", e),
    }

    // ocr 인식을 한 뒤, tx_pub으로 메시지를 보내준다.
    let request_time = Utc::now().timestamp();
    let ocr_pub = OcrPub::new(ocr_sub.gate_id, true, res_str, 84.2, request_time as u64);
    let message = PubMessage::OcrPub(ocr_pub);

    println!("OcrPub: {:?}", message);

    match tx.send(message) {
        Ok(_) => {
            println!("success to send message to pub receiver");
        },
        Err(e) => {
            eprintln!("fail to send pub, err: {}", e);
        },
    }
}