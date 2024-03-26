use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::bybit_struct::{BybitSubscriptionConfirmation, BybitWsResponse};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum BybitMessage {
    SubscriptionConfirmation(BybitSubscriptionConfirmation),
    DataMessage(BybitWsResponse),
}

fn calculate_percentage_change(open_price: &f64, close_price: &f64) -> f64 {
    ((close_price - open_price) / open_price) * 100.0
}

pub async fn bybit_ws(ticker: &str) {
    let (mut ws_stream, _) = connect_async("wss://stream.bybit.com/v5/public/linear")
        .await
        .expect("Failed connecting to bybit ws");
    println!("bybit ws connected");
    let ticker_subscribe = format!("kline.D.{}", ticker);
    println!("ticker: {}", ticker_subscribe);

    let subscribe_message = serde_json::json!({
        "op": "subscribe",
        "args": [ticker_subscribe],
    })
    .to_string();

    ws_stream
        .send(Message::Text(subscribe_message))
        .await
        .expect("Failed subscribing to topic");

    while let Some(messsage) = ws_stream.next().await {
        match messsage {
            Ok(Message::Text(text)) => match serde_json::from_str::<BybitMessage>(&text) {
                Ok(BybitMessage::DataMessage(parse_msg)) => {
                    let percentage_change = calculate_percentage_change(
                        &parse_msg.data[0]
                            .open
                            .parse()
                            .expect("failed converting to floating numbers"),
                        &parse_msg.data[0]
                            .close
                            .parse()
                            .expect("Failed converting to floating numbers"),
                    );
                    println!(
                        "\reth price: current: {}, percentage change: {:.3}%",
                        parse_msg.data[0].close, percentage_change
                    );
                }
                Ok(BybitMessage::SubscriptionConfirmation(sub_confirmation)) => {
                    println!("\rSubscription confirmation: {:#?}", sub_confirmation)
                }
                Err(e) => eprintln!("Failed parsing bybit data: {:#?}", e),
            },
            Ok(data) => println!("Data not parse: {:#?}", data),
            Err(e) => eprintln!("bybit ws error: {:#?}", e),
        }
    }
}
