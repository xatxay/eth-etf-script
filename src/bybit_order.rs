use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde_json::{json, Value};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn generate_signature(
    timestamp: &str,
    api_key: &str,
    recv_window: &str,
    params: &serde_json::Map<String, Value>,
    api_secret: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut mac =
        HmacSha256::new_from_slice(api_secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(timestamp.as_bytes());
    mac.update(api_key.as_bytes());
    mac.update(recv_window.as_bytes());
    mac.update(serde_json::to_string(&params)?.as_bytes());

    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    Ok(hex::encode(code_bytes))
}

pub async fn place_bybit_order(
    api_key: &str,
    api_secret: &str,
    recv_window: &str,
    place_order_url: &str,
    ticker: &str,
    price: &f64,
    quantity: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut params = serde_json::Map::new();
    params.insert("category".to_string(), json!("linear"));
    params.insert("symbol".to_string(), json!(ticker));
    params.insert("side".to_string(), json!("Buy"));
    params.insert("orderType".to_string(), json!("Market"));
    params.insert("qty".to_string(), json!(quantity));
    // params.insert("price".to_string(), json!(price.to_string()));

    let timestamp = Utc::now().timestamp_millis().to_string();
    let signature = generate_signature(&timestamp, api_key, recv_window, &params, api_secret)?;

    let response = client
        .post(place_order_url)
        .json(&params)
        .header("X-BAPI-API-KEY", api_key)
        .header("X-BAPI-SIGN", &signature)
        .header("X-BAPI-SIGN-TYPE", "2")
        .header("X-BAPI-TIMESTAMP", timestamp)
        .header("X-BAPI-RECV-WINDOW", recv_window)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    println!("\rResponse: {:?}", response.text().await?);
    Ok(())
}
