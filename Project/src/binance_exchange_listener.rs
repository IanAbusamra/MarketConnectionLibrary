use crate::exchange_listener::ExchangeListener;
use crate::market_data::MarketData;
use crate::web_socket::WebSocket;
use crate::data_packet::DataPacket;
use std::collections::VecDeque;

pub struct BinanceExchangeListener<'a> {
    id: i32,
    subscription: &'a mut WebSocket,
    queue: VecDeque<Box<dyn DataPacket>>,
}

impl<'a> BinanceExchangeListener<'a> {
    pub fn new(id: i32, subscription:  &'a mut WebSocket) -> Self {
        BinanceExchangeListener {
            id,
            subscription,
            queue: VecDeque::new(),
        }
    }

    pub fn get_subscription(&mut self) -> &mut WebSocket {
        &mut self.subscription
    }
}

impl<'a> ExchangeListener for BinanceExchangeListener<'a> {
    fn subscribe(&mut self) {
        self.subscription.connect().expect("Failed to connect");
        println!("Subscribed to Binance WebSocket");
    }

    fn unsubscribe(&mut self) {
        self.subscription.close().expect("Failed to close connection");
        println!("Unsubscribed from Binance WebSocket");
    }

    fn on_message(&mut self, json: Option<&str>) {
        if let Some(message) = json {
            let data_packet = self.parse_message(message);
            self.add_parsed_data(data_packet);
        } else {
            // Not sure what to do when no message comes in
            println!("nothing");
        }
    }
    
    fn parse_message(&self, message: &str) -> Box<dyn DataPacket> {
        Box::new(MarketData::new("Test".to_string()))
    }

    fn add_parsed_data(&mut self, data_packet: Box<dyn DataPacket>) {
        self.queue.push_back(data_packet);
    }

    fn next(&self) -> Option<&Box<dyn DataPacket>> {
        self.queue.front()
    }

    fn set_id(&mut self, new_id: i32) {
        self.id = new_id;
    }

    fn get_id(&self) -> i32 {
        self.id
    }
}
