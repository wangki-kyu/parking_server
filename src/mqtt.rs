use std::collections::HashMap;
use std::{env, process};
use std::time::Duration;
use serde::Serialize;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
use tokio::sync::Mutex;
use crate::message::{AsyncMessage, FeelInfoSub, OcrSub, PubMessage, SubMessage};
use crate::car::add_car;

const SUB_TOPIC: &[&str] = &["parking/request/#"];
const QOS: &[i32] = &[1];

struct MqttClient {

}

pub async fn run_mqtt(host: String, tx: UnboundedSender<AsyncMessage>, rx_pub: UnboundedReceiver<PubMessage>) {
    // parking/request/ocr sub
    // parking/request/feelinfo sub

    // parking/response/ocr pub
    // parking/response/feelinfo pub

    tokio::join!(
        run_subscribe(host.clone(), tx),
        run_publish(host, rx_pub),
    );
}

fn init_mqtt_clinet() {

}

async fn run_subscribe(host: String, tx: UnboundedSender<AsyncMessage>) {
    // client 생성!

    let create_opts = paho_mqtt::CreateOptionsBuilder::new_v3()
        .server_uri(host)
        .client_id("rust_async_subscribe")
        .finalize();

    let mut cli = paho_mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    let join = tokio::spawn(async move {
        let stream = cli.get_stream(25);

        // Define the set of options for the connection
        // let lwt = paho_mqtt::Message::new(
        //     "test/lwt",
        //     "[LWT] Async subscriber lost connection",
        //     paho_mqtt::QOS_1,
        // );

        // Create the connect options, explicitly requesting MQTT v3.x
        let conn_opts = paho_mqtt::ConnectOptionsBuilder::new_v3()
            .keep_alive_interval(Duration::from_secs(30))
            .clean_session(false)
            // .will_message(lwt)
            .finalize();

        // Make the connection to the broker
        cli.connect(conn_opts).await.unwrap();

        cli.subscribe_many(SUB_TOPIC, QOS).await.unwrap();

        // Just loop on incoming messages.
        println!("Waiting for messages...");

        let mut rconn_attempt: usize = 0;

        // Note that we're not providing a way to cleanly shut down and
        // disconnect. Therefore, when you kill this app (with a ^C or
        // whatever) the server will get an unexpected drop and then
        // should emit the LWT message.


        while let Ok(msg_opt) = stream.recv().await {
            let cloned_tx = tx.clone();
            let Some(msg) = msg_opt else {
                // A "None" means we were disconnected. Try to reconnect...
                println!("Lost connection. Attempting reconnect...");
                while let Err(err) = cli.reconnect().await {
                    rconn_attempt += 1;
                    println!("Error reconnecting #{}: {}", rconn_attempt, err);
                    // For tokio use: tokio::time::delay_for()
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                println!("Reconnected.");
                continue;
            };
            let topic_split_vec = msg.topic().split("/").collect::<Vec<&str>>();

            // last topic str parsing 
            let Some(topic_last_str) =  topic_split_vec.last() else {
                println!("wrong topic!");
                continue;
            };

            let sub_message = match *topic_last_str {
                "ocr" => {
                    println!("ocr!");
                    let req: OcrSub = serde_json::from_str(&msg.payload_str().to_string()).unwrap();
                    Some(SubMessage::OcrRequest(req))
                },
                #[allow(non_snake_case)]
                "feeinfo" => {
                    println!("fee_info");
                    let req: FeelInfoSub = serde_json::from_str(&msg.payload_str().to_string()).unwrap();
                    Some(SubMessage::FeelInfoRequest(req))
                },
                _ => {
                    println!("wrong topic!");
                    None
                }
            };

            let str = String::from_utf8_lossy(msg.payload());
            println!("{:?}", str);
            // send tx with sub message
            if let Some(sub_message) = sub_message {
                 match cloned_tx.send(AsyncMessage::SubMessage(sub_message)) {
                     Ok(_) => println!("success send!"),
                     Err(e) => eprint!("fail to send: {}", e),
                 }
            }
        }
    });

    join.await.unwrap();
}

async fn run_publish(host: String, mut rx: UnboundedReceiver<PubMessage>) -> anyhow::Result<()> {
    let cli = paho_mqtt::AsyncClient::new(host)?;

    loop {
        let cli_clone = cli.clone();
        for msg in rx.recv().await.iter() {
                match msg {
                PubMessage::OcrPub(ocr_pub) => {
                    // OcrPub
                    println!("ocr_pub: {:?}", ocr_pub);

                    // 인메로리 차량 데이터 추가 
                    add_car(ocr_pub.license_plate.clone()).await;

                    cli_clone.connect(None).await?;
                    let encoded = serde_json::to_string(ocr_pub)?;
                    let msg = paho_mqtt::Message::new("parking/response/ocr", encoded, paho_mqtt::QOS_1);
                    cli.publish(msg).await?;

                    cli.disconnect(None).await?;
                }
                PubMessage::FeelInfoPub(feel_info_pub) => {
                    // FeelInfoPub

                }
            }
        }
    }

    Ok(())
}

async fn subscribe_topic(topic: &str) {

}

async fn publish_data_to_topic(topic: &str) {

}










// hashmap 구조 
// 차량 번호 키 - 