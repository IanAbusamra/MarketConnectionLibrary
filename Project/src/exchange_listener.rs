use crate::data_packet::DataPacket;
use std::option::Option;

pub trait ExchangeListener {
    fn subscribe(&mut self);
    fn unsubscribe(&mut self);
    fn on_message(&mut self, json: Option<&str>); // need help implementing
    fn parse_message(&self, message: &str) -> Box<dyn DataPacket>;
    fn add_parsed_data(&mut self, dp: Box<dyn DataPacket>);
    fn next(&self) -> Option<&Box<dyn DataPacket>>;
    fn set_id(&mut self, new_id: i32);
    fn get_id(&self) -> i32;
}