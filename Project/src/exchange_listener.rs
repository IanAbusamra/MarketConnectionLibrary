use async_trait::async_trait;
use crate::data_packet::DataPacket;
use std::option::Option;

#[async_trait]
pub trait ExchangeListener {
    async fn subscribe(&mut self);
    async fn unsubscribe(&mut self);
    fn parse_message(&self, message: &str) -> Box<DataPacket>;
    async fn next(&mut self) -> Option<Box<DataPacket>>;
    fn set_id(&mut self, new_id: i32);
    fn get_id(&self) -> i32;
}