pub trait ExchangeListener {
    fn subscribe(&self, ws: &WebSocket);
    fn unsubscribe(&self);
    fn onmessage(json); // need help implementing
    fn parse_message(&self) -> DataPacket;
    fn add_parsed_data(&self, dp: &DataPacket);
    fn next(&self) -> Option<DataPacket>;
    fn set_id(&self, new_id: i32){
        self.id = new_id;
    }
    fn get_id(&self) -> i32 {
        self.id
    }
}