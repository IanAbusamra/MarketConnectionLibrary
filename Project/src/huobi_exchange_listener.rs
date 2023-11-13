use async_trait::async_trait;
use crate::exchange_listener::ExchangeListener;
//use crate::market_data::MarketData;
use crate::web_socket::WebSocket;
use crate::data_packet::DataPacket;
use crate::data_packet::DataEnum;
use tokio_tungstenite::tungstenite::Error as TungsteniteError;
use crate::data_packet::BestBidAskDataBTCBinance;

pub struct HuobiExchangeListener<'a> {
    id: i32,
    subscription: &'a mut WebSocket,
}

impl<'a> HuobiExchangeListener<'a> {
    pub fn new(id: i32, subscription: &'a mut WebSocket) -> Self {
        HuobiExchangeListener { id, subscription }
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

        let enum_creator = BestBidAskDataBTCHuobi {
            bestask: parsed_data["tick"]["asks"][0][0].as_str().expect("Issue parsing JSON").parse().unwrap(),
            askamt: parsed_data["tick"]["asks"][0][1].as_str().expect("Issue parsing JSON").parse().unwrap(),
            bestbid: parsed_data["tick"]["bids"][0][0].as_str().expect("Issue parsing JSON").parse().unwrap(),
            bidamt: parsed_data["tick"]["bids"][0][1].as_str().expect("Issue parsing JSON").parse().unwrap(),
        };

        let ret = DataPacket {
            Data: DataEnum::BBAHuobiBTCData(enum_creator),
            Exchange: String::from("Huobi"),
            Channel: String::from("Channel 2"),
            timestamp: parsed_data["ts"].as_str().expect("Issue parsing JSON").parse().unwrap()
        };
        Box::new(ret)
    }

    // No longer necessary
    async fn on_message(&mut self, json: Option<&str>) {
        if let Some(message) = json {
            let _data_packet = self.parse_message(message);
            // Maybe need to add more functionality with the parsed message
        } else {
            println!("No message received");
        }
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