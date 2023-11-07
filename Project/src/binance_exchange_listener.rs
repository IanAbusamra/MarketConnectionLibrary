use async_trait::async_trait;
use crate::exchange_listener::ExchangeListener;
use crate::market_data::MarketData;
use crate::web_socket::WebSocket;
use crate::data_packet::DataPacket;

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

    async fn on_message(&mut self, json: Option<&str>) {
        if let Some(message) = json {
            let _data_packet = self.parse_message(message);
            // Maybe need to add more functionality with the parsed message
        } else {
            println!("No message received");
        }
    }
    
    fn parse_message(&self, message: &str) -> Box<dyn DataPacket> {
        Box::new(MarketData::new(message.to_string()))
    }

    async fn next(&mut self) -> Option<Box<dyn DataPacket>> {
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
