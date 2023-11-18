//TODO: trade detail
pub enum ExchangeEnum {
    Huobi, 
    Binance,
}

pub enum SymbolEnum {
    BTCUSD,
    ETHUSD,
}

pub struct DataPacket {
    pub data: DataEnum,
    pub exchange: ExchangeEnum,
    pub symbol_pair: SymbolEnum,
    pub channel: String,
    pub timestamp: i64,
}

pub enum DataEnum {
    MBP(MarketIncremental),
    RBA(RefreshBidAsk),
}

pub struct MarketIncremental {
    pub bestask: f64,
    pub askamount: f64,
    pub bestbid: f64,
    pub bidamount: f64,
}

pub struct RefreshBidAsk {
    pub asks: Vec<(f64, f64)>, //price, amount
    pub bids: Vec<(f64, f64)>, //price, amount
}