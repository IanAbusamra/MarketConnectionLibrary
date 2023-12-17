use async_trait::async_trait;
use crate::exchange_listener::exchange_listener::ExchangeListener;
use crate::web_socket::WebSocket;
use crate::data_packet::*;
use crate::data_packet::SymbolEnum::*;
use crate::data_packet::ExchangeEnum::*;
use futures::task::{Context, Poll, noop_waker_ref};
use std::pin::Pin;
use futures_util::Stream;
use chrono::{Utc, TimeZone};
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use hex::encode as hex_encode;
use reqwest::{self, Error as ReqwestError};
use reqwest::Response;

pub struct BinanceExchangeListener<'a> {
    pub id: i32,
    pub subscription: &'a mut WebSocket,
}

impl<'a> BinanceExchangeListener<'a> {
    pub fn new(id: i32, subscription: &'a mut WebSocket) -> Self {
        BinanceExchangeListener { 
            id, 
            subscription,
        }
    }

    pub fn get_subscription(&mut self) -> &mut WebSocket {
        &mut self.subscription
    }
}

impl<'a> BinanceExchangeListener<'a> {
    pub async fn authenticated_request(
        &self,
        api_key: &str,
        secret_key: &str,
    ) -> Result<String, String> {
        let endpoint = "https://api.binance.com/api/v3/account";
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let query_string = format!("timestamp={}", timestamp);
    
        // Create HMAC SHA256 signature
        let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(query_string.as_bytes());
        let signature = hex_encode(mac.finalize().into_bytes());
    
        // Append signature to query string
        let signed_query = format!("{}&signature={}", query_string, signature);
    
        // Make HTTP GET request
        let client = reqwest::Client::new();
        let response = client.get(endpoint)
            .header("X-MBX-APIKEY", api_key)
            .query(&[("timestamp", timestamp.as_str()), ("signature", signature.as_str())])
            .send()
            .await
            .map_err(|e| e.to_string())?;
    
        if response.status().is_success() {
            response.text().await.map_err(|e| e.to_string())
        } else {
            Err(format!("Error: {}", response.status()))
        }
    }
}

#[async_trait]
impl<'a> ExchangeListener for BinanceExchangeListener<'a> {
    async fn subscribe(&mut self) {
        self.subscription.connect().await.expect("Failed to connect");
        println!("Subscribed to Binance WebSocket");
    }

    async fn unsubscribe(&mut self) {
        self.subscription.close().await.expect("Failed to close connection");
        println!("Unsubscribed from Binance WebSocket");
    }

    fn parse_message(&self, message: &str) -> Box<DataPacket> {
        let parsed_data: serde_json::Value = serde_json::from_str(&message).expect("Unable to parse message");
    
        let mut ask_vector: Vec<(f64, f64)> = Vec::new();
        let mut bid_vector: Vec<(f64, f64)> = Vec::new();

        for i in 0..5 {
            let ask_price: Option<f64> = parsed_data["asks"][i][0].as_f64();
            let ask_quantity: Option<f64> = parsed_data["asks"][i][1].as_f64();
            let bid_price: Option<f64> = parsed_data["bids"][i][0].as_f64();
            let bid_quantity: Option<f64> = parsed_data["bids"][i][1].as_f64();

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

        let given_id = parsed_data["lastUpdateId"].as_i64().unwrap_or_default();

        let ret = DataPacket {
            prevNum: -1,
            curNum: given_id,
            data: DataEnum::MBP(enum_creator),
            exchange: Binance,
            symbol_pair: BTCUSD,
            channel: String::from("Channel 1"),
            timestamp: 0,
        };
        Box::new(ret)
    }

    // fn trade(&self, api_key: &str, secret_key: &str, symbol: &str, side: &str, quantity: &str) {
    //     let timestamp = chrono::Utc::now().timestamp_millis().to_string();
    // }

    fn poll(&mut self) -> Result<Option<Box<DataPacket>>, String> {
        let waker = noop_waker_ref();
        let mut context = Context::from_waker(&waker);
        if let Some(socket) = self.get_subscription().get_mut_socket() {
            let socket = Pin::new(socket);

            match socket.poll_next(&mut context) {
                Poll::Ready(Some(Ok(message))) => {
                    println!("{}", message);
                    let dpp = self.parse_message(&message.to_string());
                    let timestamp = dpp.curNum;
                    //
                    //
                    //
                    //Implement with binance's server time if possible
                    let serverTime: i64 = 0;
                    //
                    //
                    //there's also an additional recvWindow parameter to use - I don't think needed.
                    if timestamp < (serverTime + 1000) {
                        Ok(Some(dpp))
                    } else {
                        return Err("Sequence number gap detected. Refresh needed.".to_string());
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
                    println!("Waiting...");
                    Ok(None)
                }
            }
        } else {
            println!("WebSocket is not connected.");
            Ok(None)
        }
    }

    fn set_id(&mut self, new_id: i32) {
        self.id = new_id;
    }

    fn get_id(&self) -> i32 {
        self.id
    }
}
