pub struct DataPacket {
    pub Data: DataEnum,
    pub Exchange: String,
    pub Channel: String,
}

///////////////////////////////////////////////

pub enum DataEnum {
    BBABinanceData(BestBidAskDataBinance),
    M2(MessageType2),
}

pub struct BestBidAskDataBinance {
    pub bestask: f64,
    pub askamt: f64,
}

pub struct MessageType2 {
    pub placeholder: String,
}
