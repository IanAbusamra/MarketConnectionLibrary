use crate::DataPacket;

pub struct TradeData {
    data: String,
}

impl TradeData {
    pub fn new(data: String) -> Self {
        TradeData { data }
    }
}

impl DataPacket for TradeData {
    fn get_data(&self) -> &str {
        &self.data
    }
}
