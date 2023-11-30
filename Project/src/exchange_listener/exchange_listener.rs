use async_trait::async_trait;
use crate::data_packet::DataPacket;
use std::option::Option;

#[async_trait]
pub trait ExchangeListener {
    /// Asynchronously subscribes to an exchange's data feed.
    async fn subscribe(&mut self);

    /// Asynchronously unsubscribes from an exchange's data feed.
    async fn unsubscribe(&mut self);

    /// Parses a message received from the exchange into a `DataPacket`.
    ///
    /// # Arguments
    /// * `message` - A string slice representing the message to be parsed.
    ///
    /// # Returns
    /// A `Box<DataPacket>` containing the parsed data.
    fn parse_message(&self, message: &str) -> Box<DataPacket>;

    /// Sets a new identifier for the exchange listener.
    ///
    /// # Arguments
    /// * `new_id` - An `i32` representing the new identifier.
    fn set_id(&mut self, new_id: i32);

    /// Retrieves the current identifier of the exchange listener.
    ///
    /// # Returns
    /// An `i32` representing the current identifier.
    fn get_id(&self) -> i32;

    /// Polls for new messages or data from the exchange.
    ///
    /// # Returns
    /// An `Option<()>` which is `Some(())` if new data is received or `None` if no data is available.
    fn poll(&mut self) -> Option<()>;
}