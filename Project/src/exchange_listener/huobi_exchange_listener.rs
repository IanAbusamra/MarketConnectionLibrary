use async_trait::async_trait;
use crate::exchange_listener::exchange_listener::ExchangeListener;
use crate::web_socket::WebSocket;
use crate::data_packet::*;
use crate::data_packet::SymbolEnum::*;
use crate::data_packet::ExchangeEnum::*;
use futures::task::{Context, Poll, noop_waker_ref};
use std::pin::Pin;
use futures_util::Stream;
use flate2::read::GzDecoder;
use tokio_tungstenite::tungstenite::{Message};
use std::io::Read;
use serde_json::{Value, json};
use tokio_tungstenite::tungstenite::{error::Error as TungsteniteError};
use hmac_sha256::HMAC;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use urlencoding::encode;
use reqwest::{self, Client, Error as ReqwestError, Method};


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>, // Use serde_json::Value to represent any structured data
}

// You can also create more specific structs for known response formats:
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub id: i64,
    #[serde(rename = "type")]
    pub account_type: String,
    pub state: String,
    // ... other fields as documented by the API
}

// If you expect a list of accounts in the `data` field:
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsResponse {
    pub status: String,
    pub data: Vec<AccountInfo>,
    // ... include status, err_code, err_msg similar to ApiResponse
}

pub struct HuobiExchangeListener<'a> {
    id: i32,
    subscription: &'a mut WebSocket,
    prevNum: i64,
}

impl<'a> HuobiExchangeListener<'a> {
    pub fn new(id: i32, subscription: &'a mut WebSocket) -> Self {
        HuobiExchangeListener { id, subscription, prevNum: -1}
    }

    pub fn get_subscription(&mut self) -> &mut WebSocket {
        &mut self.subscription
    }

    pub async fn place_order(&mut self, account_id: &str, amount: f64,  price: f64, symbol: &str, order_type: &str) -> Result<(), TungsteniteError> {
        let order_message = json!({
            "account-id": account_id,
            "amount": amount.to_string(),
            "price": price.to_string(),
            "symbol": symbol,
            "type": order_type
        }).to_string();
    
        let send_message = json!({
            "op": "order",
            "data": order_message
        }).to_string();
    
        self.subscription.send(&send_message).await.expect("Failed to send order");
    
        Ok(())
    }

    pub async fn authenticated_request(
        &self,
        api_key: &str,
        secret_key: &str,
        http_method: &str,
        endpoint: &str,
        params: &[(String, String)],
    ) -> Result<ApiResponse, ReqwestError> {
        let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();

        // Manually building the parameter string in the required order
        let mut query_string = format!(
            "SignatureMethod=HmacSHA256&SignatureVersion=2&AccessKeyId={}&Timestamp={}",
            encode(api_key), encode(&timestamp)
        );

        // Add any additional parameters
        for (key, value) in params {
            query_string = format!("{}&{}={}", query_string, key, encode(value));
        }

        let pre_signed_text = format!("{}\napi.huobi.pro\n{}\n{}", http_method.to_uppercase(), endpoint, query_string);
        let signature = HMAC::mac(pre_signed_text.as_bytes(), secret_key.as_bytes());
        let signature_base64 = base64::encode(signature);

        let url = format!("https://api.huobi.pro{}?{}&Signature={}", endpoint, query_string, encode(&signature_base64));
        println!("Request URL: {}", url);

        let reqwest_method = match http_method.parse::<Method>() {
            Ok(valid_method) => valid_method,
            Err(_) => {
                eprintln!("Invalid HTTP method: {}", http_method);
                panic!("Invalid HTTP method: {}", http_method);
            }
        };

        let client = Client::new();
        let response = client
            .request(reqwest_method, &url)
            .send()
            .await?
            .json::<ApiResponse>()
            .await?;

        Ok(response)
    }

    /* 
    pub asych fn cancel_order(){

    }
    */
}

#[async_trait]
impl<'a> ExchangeListener for HuobiExchangeListener<'a> {
    async fn subscribe(&mut self) {
        self.subscription.connect().await.expect("Failed to connect");
        println!("Subscribed to Huobi WebSocket");
    }

    async fn unsubscribe(&mut self) {
        self.subscription.close().await.expect("Failed to close connection");
        println!("Unsubscribed from Huobi WebSocket");
    }

    fn parse_message(&self, message: &str) -> Box<DataPacket> {
        let parsed_data: serde_json::Value = serde_json::from_str(message).expect("Unable to parse message");
    
        let mut ask_vector: Vec<(f64, f64)> = Vec::new();
        let mut bid_vector: Vec<(f64, f64)> = Vec::new();
    
        for i in 0..5 {
            let ask_price: Option<f64> = parsed_data["tick"]["asks"][i][0].as_f64();
            let ask_quantity: Option<f64> = parsed_data["tick"]["asks"][i][1].as_f64();
            let bid_price: Option<f64> = parsed_data["tick"]["bids"][i][0].as_f64();
            let bid_quantity: Option<f64> = parsed_data["tick"]["bids"][i][1].as_f64();

            let ask_pair: (f64, f64) = (
                ask_price.unwrap_or_default(),
                ask_quantity.unwrap_or_default(),
            );

            let bid_pair: (f64, f64) = (
                bid_price.unwrap_or_default(),
                bid_quantity.unwrap_or_default(),
            );

            ask_vector.push(ask_pair);
            bid_vector.push(bid_pair);
        }

        let enum_creator = MarketIncremental {
            asks: ask_vector,
            bids: bid_vector,
        };

        let prevNum: i64 = parsed_data["tick"]["prevSeqNum"].as_i64().unwrap_or_default();
        let curNum: i64 = parsed_data["tick"]["seqNum"].as_i64().unwrap_or_default();
        
        let ret = DataPacket {
            curNum: curNum,
            prevNum: prevNum,
            data: DataEnum::MBP(enum_creator),
            exchange: Huobi,
            symbol_pair: BTCUSD,
            channel: String::from("Channel 1"),
            timestamp: 0,
        };
        Box::new(ret)
    }

    fn poll(&mut self) -> Result<Option<Box<DataPacket>>, String> {
        let waker = noop_waker_ref();
        let mut context = Context::from_waker(&waker);

        if let Some(socket) = self.get_subscription().get_mut_socket() {
            let socket = Pin::new(socket);

            match socket.poll_next(&mut context) {
                Poll::Ready(Some(Ok(msg))) => {
                    match msg {
                        Message::Ping(ping_data) => {
                            //Never Reached
                            Ok(None)
                        },
                        Message::Binary(data) => {
                            // Attempt to decompress the data using a GZIP decoder
                            let mut decoder = GzDecoder::new(&data[..]);
                            let mut decompressed_data = Vec::new();
                            match decoder.read_to_end(&mut decompressed_data) {
                                Ok(_) => {
                                    // Convert decompressed data to text
                                    let text = String::from_utf8(decompressed_data).expect("Found invalid UTF-8");
                                    println!("{}", text);
            
                                    // Respond to pings
                                    if let Ok(parsed) = serde_json::from_str::<Value>(&text) {
                                        if let Some(ping) = parsed.get("ping") {
                                            println!("{}", text);
                                            let pong_response = format!("{{\"pong\":{}}}", ping);
                                            println!("BEGIN");
                                            self.subscription.send2(&pong_response);
                                            println!("Sent Pong response: {}", pong_response);
                                            println!("END");
                                            println!("");
                                            println!("");
                                            println!("");
                                            Ok(None)
                                        } else {
                                            let dpp = self.parse_message(&text);
                                            if self.prevNum == -1 {
                                                self.prevNum = dpp.curNum;
                                            } else if dpp.prevNum != self.prevNum {
                                                return Err("Sequence number gap detected. Refresh needed.".to_string());
                                            } else {
                                                self.prevNum = dpp.curNum;
                                            }
                                            Ok(Some(dpp))
                                        }
                                    } else {
                                        Ok(None)
                                    }
                                },
                                Err(e) => {
                                    println!("Failed to decompress GZIP data: {:?}", e);
                                    Ok(None)
                                }
                            }
                        },
                        Message::Text(text) => {
                            //Never Reached
                            Ok(None)
                        },
                        _ => {
                            //Never Reached
                            Ok(None)
                        }
                    }
                },
                Poll::Ready(Some(Err(e))) => {
                    println!("Error receiving message: {:?}", e);
                    Ok(None)
                },
                Poll::Ready(None) => {
                    println!("Socket closed.");
                    Ok(None)
                },
                Poll::Pending => {
                    //println!("Waiting...");
                    Ok(None)
                }
            }
        } else {
            println!("WebSocket is not connected.");
            Ok(None)
        }
    }
/*     
        async fn next(&mut self) -> Option<Box<DataPacket>> {
        match self.subscription.receive().await {
            Ok(Some(message)) => {
                // We expect that the message is a JSON string here, whether it was originally text or binary.
                if let Ok(parsed) = serde_json::from_str::<Value>(&message) {
                    println!("parsed: {}", parsed);
                    if parsed.get("ping").is_some() {
                        // If it's a ping message, construct a pong response.
                        println!("Received Ping: {:?}", parsed);
                        let pong_response = json!({ "pong": parsed["ping"] }).to_string();
                        // Send the pong response asynchronously and ignore errors here.
                        let _ = self.subscription.send(&pong_response).await;
                        println!("Sent Pong response: {}", pong_response);
                        None
                    } else {
                        // Parse the message as a data packet if it's not a ping.
                        println!("Received non-ping/pong message: {}", message);
                        Some(self.parse_message(&message))
                    }
                } else {
                    // If the message is not valid JSON, print an error message.
                    println!("Received message that was not valid JSON: {}", message);
                    None
                }
            },
            Ok(None) => {
                // No new messages are available at the moment.
                None
            },
            Err(e) => {
                // An error occurred while trying to receive a message.
                println!("Error receiving message: {:?}", e);
                None
            }
        }
    }
    */
     
    fn set_id(&mut self, new_id: i32) {
        self.id = new_id;
    }

    fn get_id(&self) -> i32 {
        self.id
    }
}
