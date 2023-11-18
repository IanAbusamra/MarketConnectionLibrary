use async_trait::async_trait;
use crate::exchange_listener::ExchangeListener;
use crate::web_socket::WebSocket;
// use crate::data_packet::{DataPacket, ExchangeEnum, SymbolEnum, MarketIncremental, RefreshBidAsk, DataEnum, SymbolEnum};
use crate::data_packet::*;
use crate::data_packet::SymbolEnum::*;
use crate::ExchangeEnum::*;
use tokio_tungstenite::tungstenite::Error as TungsteniteError;

pub struct BinanceExchangeListener<'a> {
    id: i32,
    subscription: &'a mut WebSocket,
}

impl<'a> BinanceExchangeListener<'a> {
    pub fn new(id: i32, subscription: &'a mut WebSocket) -> Self {
        BinanceExchangeListener { id, subscription }
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
            Data: DataEnum::MBP(enum_creator),
            Exchange: Binance,
            SymbolPair: BTCUSD,
            Channel: String::from("Channel 1"),
            timestamp: 0,
        };
        Box::new(ret)
    }

    // No longer necessary
    async fn next(&mut self) -> Option<Box<DataPacket>> {
        match self.subscription.receive().await {
            Ok(Some(message)) => Some(self.parse_message(&message)),
            Ok(None) => None,
            Err(e) => {
                println!("Error receiving message: {:?}", e);
                None
            }
        }
    }

    fn set_id(&mut self, new_id: i32) {
        self.id = new_id;
    }

    fn get_id(&self) -> i32 {
        self.id
    }
}
