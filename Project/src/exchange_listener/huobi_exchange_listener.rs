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
use tokio::time::{sleep, Duration};

pub struct HuobiExchangeListener<'a> {
    id: i32,
    subscription: &'a mut WebSocket,
}

impl<'a> HuobiExchangeListener<'a> {
    pub fn new(id: i32, subscription: &'a mut WebSocket) -> Self {
        HuobiExchangeListener { id, subscription, }
    }

    pub fn get_subscription(&mut self) -> &mut WebSocket {
        &mut self.subscription
    }
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
            //println!("{}", parsed_data["tick"]["bids"][i][1]);

            //TODO: not unwrapping correctly always going to default value
            let ask_pair: (f64, f64) = (
                ask_price.unwrap_or_default(),
                ask_quantity.unwrap_or_default(),
            );

            let bid_pair: (f64, f64) = (
                bid_price.unwrap_or_default(),
                bid_quantity.unwrap_or_default(),
            );
            //println!("{}", bid_pair.0);

            ask_vector.push(ask_pair);
            bid_vector.push(bid_pair);
        }

        let enum_creator = MarketIncremental {
            asks: ask_vector,
            bids: bid_vector,
        };

        let prevNum: i64 = parsed_data["tick"]["prevSeqNum"].as_i64().unwrap_or_default();
        
        let ret = DataPacket {
            prevSeqNum: prevNum,
            data: DataEnum::MBP(enum_creator),
            exchange: Huobi,
            symbol_pair: BTCUSD,
            channel: String::from("Channel 1"),
            timestamp: 0,
        };
        Box::new(ret)
    }
    fn poll(&mut self) -> Option<()> {
        let waker = noop_waker_ref();
        let mut context = Context::from_waker(&waker);

        if let Some(socket) = self.get_subscription().get_mut_socket() {
            let socket = Pin::new(socket);

            match socket.poll_next(&mut context) {
                Poll::Ready(Some(Ok(msg))) => {
                    match msg {
                        Message::Ping(ping_data) => {
                            let pong_response = json!({ "pong": ping_data }).to_string();
                            println!("SPR: {}", pong_response);
                            self.subscription.send2(&pong_response);
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
                                        } else {
                                            let dpp = self.parse_message(&text);
                                            if let DataEnum::MBP(mbp) = dpp.data {
                                                let asks_vector = &mbp.asks;
                                                println!("Asks: {:?}", asks_vector);
                                            } else {
                                                println!("failure");
                                            }
                                        }
                                    }
                                },
                                Err(e) => {
                                    println!("Failed to decompress GZIP data: {:?}", e);
                                }
                            }
                        },
                        Message::Text(text) => {
                            println!("Received text: {}", text);
                            // Handle text message.
                        },
                        _ => {
                            // Handle other message types
                        }
                    }

                    Some(())
                },
                Poll::Ready(Some(Err(e))) => {
                    println!("Error receiving message: {:?}", e);
                    None
                },
                Poll::Ready(None) => {
                    println!("Socket closed.");
                    None
                },
                Poll::Pending => {
                    //println!("Waiting...");
                    None
                }
            }
        } else {
            println!("WebSocket is not connected.");
            None
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
