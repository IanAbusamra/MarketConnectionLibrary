use async_trait::async_trait;
use crate::exchange_listener::ExchangeListener;
use crate::web_socket::WebSocket;
use crate::data_packet::*;
use crate::data_packet::SymbolEnum::*;
use crate::data_packet::ExchangeEnum::*;
use futures::task::{Context, Poll, noop_waker_ref};
use std::pin::Pin;
use futures_util::Stream;
use flate2::read::GzDecoder;
use tungstenite::Message;
use std::io::Read;
use serde_json::json;
use serde_json::Value;

pub struct BinanceExchangeListener<'a> {
    pub id: i32,
    pub subscription: &'a mut WebSocket,
}

impl<'a> BinanceExchangeListener<'a> {
    pub fn new(id: i32, subscription: &'a mut WebSocket) -> Self {
        BinanceExchangeListener { id, subscription, }
    }

    pub fn get_subscription(&mut self) -> &mut WebSocket {
        &mut self.subscription
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
        let parsed_data: serde_json::Value = serde_json::from_str(message).expect("Unable to parse message");
    
        let enum_creator = MarketIncremental {
            bestask: parsed_data["asks"][0][0].as_str().expect("Issue parsing JSON").parse().unwrap(),
            askamount: parsed_data["asks"][0][1].as_str().expect("Issue parsing JSON").parse().unwrap(),
            bestbid: 0.0,
            bidamount: 0.0, //just for testing
        };

        let ret = DataPacket {
            data: DataEnum::MBP(enum_creator),
            exchange: Binance,
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
                // Poll::Ready(Message::Ping(ping_data)) => {
                //     println!("Received Ping: {:?}", ping_data);
                //     socket.send(Message::Pong(ping_data)).expect("Error sending pong");
                //     println!("Response pong sent");
                //     Some(())
                // },
                Poll::Ready(Some(Ok(msg))) => {
                    match msg {
                        Message::Ping(ping_data) => {
                            println!("Ping branch Reached");
                            // println!("Received Ping: {:?}", ping_data);
                            // socket.write_message(Message::Pong(ping_data)).expect("Error sending pong");
                        },
                        Message::Binary(data) => {
                            println!("Binary branch reached!!!!");
                            println!("Received binary data: {:?}", data);
            
                            // Attempt to decompress the data using a GZIP decoder
                            let mut decoder = GzDecoder::new(&data[..]);
                            let mut decompressed_data = Vec::new();
                            match decoder.read_to_end(&mut decompressed_data) {
                                Ok(_) => {
                                    println!("WE HAVE DECOMPRESSED THE DATA");
                                    // println!("Decompressed data: {:?}", decompressed_data);
                                    
                                    // Convert decompressed data to text
                                    let text = String::from_utf8(decompressed_data).expect("Found invalid UTF-8");
                                    println!("Decompressed text: {}", text);
            
                                    // Respond to pings
                                    if let Ok(parsed) = serde_json::from_str::<Value>(&text) {
                                        if let Some(ping) = parsed.get("ping") {
                                            let pong_response = json!({ "pong": ping }).to_string();
                                            //self.subscription.send("").expect("Failed to send pong");
                                            self.subscription.send(&pong_response);
                                            println!("Sent Pong response: {}", pong_response);
                                        }
                                    }
                                },
                                Err(e) => {
                                    println!("Failed to decompress GZIP data: {:?}", e);
                                }
                            }
                        },
                        Message::Text(text) => {
                            println!("text branch reached");
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
                    println!("Waiting...");
                    None
                }
            }
        } else {
            println!("WebSocket is not connected.");
            None
        }
    }

    fn set_id(&mut self, new_id: i32) {
        self.id = new_id;
    }

    fn get_id(&self) -> i32 {
        self.id
    }
}
