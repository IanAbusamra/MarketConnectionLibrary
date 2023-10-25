pub enum DataPacket {
    MarketData(MarketDataPacket)
    TradeData(TradeDataPacket);
}
