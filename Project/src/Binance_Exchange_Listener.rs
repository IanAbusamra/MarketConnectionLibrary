use crate::exchange_listener::ExchangeListener;
use crate::market_data::MarketData;
use crate::web_socket::WebSocket;
use crate::data_packet::DataPacket;
use std::collections::VecDeque;

pub struct BinanceExchangeListener {
    id: i32,
    subscription: WebSocket,
    queue: VecDeque<Box<dyn DataPacket>>,
}

impl BinanceExchangeListener {
    pub fn new(id: i32, subscription: WebSocket) -> Self {
        BinanceExchangeListener {
            id,
            subscription,
            queue: VecDeque::new(),
        }
    }
}

impl ExchangeListener for BinanceExchangeListener {
    fn subscribe(&mut self, ws: &WebSocket) {
        self.subscription.connect().expect("Failed to connect");
        println!("Subscribed to Binance WebSocket");
    }

    fn unsubscribe(&mut self) {
        self.subscription.close().expect("Failed to close connection");
        println!("Unsubscribed from Binance WebSocket");
    }

    fn on_message(&mut self, json: &str) {
        if let Ok(Some(message)) = self.subscription.receive() {
            let data_packet = self.parse_message(&message);
            self.add_parsed_data(data_packet);
        }
    }

    fn parse_message(&self, message: &str) -> DataPacket {
        //Box::new(MarketData::new("Test".to_string()))
        //Unfinished, also currently only implemented for binance
        //maybe change to vector of top x values instead of just best.
        let parsed_data: serde_json::Value = serde_json::from_str(&message).expect("Unable to parse message");
        let cur_marketdata = MarketData {
            exchange: Binance,
            best_ask: parsed_data["asks"][0][0].as_f64(),
            ask_size: parsed_data["asks"][0][1].as_i64(),
        }
        let ret = DataPacket::MarketData(cur_marketdata);
        return ret;
    }

    fn add_parsed_data(&mut self, data_packet: DataPacket) {
        self.queue.push_back(data_packet);
    }

    fn next(&self) -> Option<&Box<dyn DataPacket>> {
        self.queue.front()
    }

    fn set_id(&self, new_id: i32) {
        self.id = new_id;
    }

    fn get_id(&self) -> i32 {
        self.id
    }
}