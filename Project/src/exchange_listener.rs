use crate::web_socket::WebSocket;
use crate::data_packet::DataPacket;
use std::option::Option;

pub trait ExchangeListener {
    fn subscribe(&mut self, ws: &WebSocket);
    fn unsubscribe(&mut self);
    fn on_message(&mut self, json: &str); // need help implementing
    fn parse_message(&self, message: &str) -> Box<dyn DataPacket>;
    fn add_parsed_data(&mut self, dp: Box<dyn DataPacket>);
    fn next(&self) -> Option<&Box<dyn DataPacket>>;
    fn set_id(&self, new_id: i32){
        self.id = new_id;
    }
    fn get_id(&self) -> i32 {
        self.id
    }
}