pub struct DataPacket {
    pub Data: DataEnum,
    pub Exchange: String,
    pub Channel: String,
}

///////////////////////////////////////////////

pub enum DataEnum {
    BBABinanceBTCData(BestBidAskDataBTCBinance),
    BBABinanceETHData(BestBidAskDataETHBinance),
}

pub struct BestBidAskDataBTCBinance {
    pub bestask: f64,
    pub askamt: f64,
}

pub struct BestBidAskDataETHBinance {
    pub bestask: f64,
    pub askamt: f64,
}
