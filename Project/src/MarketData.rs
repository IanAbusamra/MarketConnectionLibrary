enum exchange_type {
    Binance,
    Huobi,
}

pub struct MarketDataPacket {
    //issues: 
    //- don't know exactly what info the traders want.
    exchange: exchange_type,
    best_ask: f32,
    ask_size: i32,
}