enum exchange_type {
    Binance,
    Huobi,
}

pub struct MarketData {
    //issues: 
    //- numerical values from API returned as type "Value" which I can't cast into a primitive type.
    //- don't know exactly what info the traders want.
    exchange: exchange_type,
    best_ask: f32,
    ask_size: i32,
}