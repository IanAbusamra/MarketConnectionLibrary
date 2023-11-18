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
    pub Data: DataEnum,
    pub Exchange: ExchangeEnum,
    pub SymbolPair: SymbolEnum,
    pub Channel: String,
    pub timestamp: i64,
}

pub enum DataEnum {
    RBA(RefreshBidAsk),
    MBP(MarketIncremental),
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