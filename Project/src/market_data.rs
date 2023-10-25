use crate::DataPacket;

pub struct MarketData {
    data: String,
}

impl MarketData {
    pub fn new(data: String) -> Self {
        MarketData { data }
    }
}

impl DataPacket for MarketData {
    fn get_data(&self) -> &str {
        &self.data
    }
}
