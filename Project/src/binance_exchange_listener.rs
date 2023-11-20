use async_trait::async_trait;
use crate::exchange_listener::ExchangeListener;
use crate::web_socket::WebSocket;
use crate::data_packet::*;
use crate::data_packet::SymbolEnum::*;
use crate::data_packet::ExchangeEnum::*;
use futures::task::{Context, Poll, noop_waker_ref};
use std::pin::Pin;
use futures_util::Stream;

pub struct BinanceExchangeListener<'a> {
    id: i32,
    subscription: &'a mut WebSocket,
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
                Poll::Ready(Some(Ok(message))) => {
                    let data_packet = self.parse_message(&message.to_string());
                    match data_packet.data {
                        DataEnum::MBP(bba_data) => {
                            let bestask_value = bba_data.bestask;
                            println!("Best Ask: {}", bestask_value);
                        }
                        DataEnum::RBA(_) => {
                            println!("Received RBA data.");
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
