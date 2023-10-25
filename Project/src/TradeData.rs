enum exchange_type {
    Binance,
    Huobi,
}

enum order_type {
    Buy, 
    Sell,
}

pub struct TradeData {
    //don't know what info the traders are going to put here, placeholders for now
    exchange: exchange_type,
    coin: String,
    quantity: f32,
    order: order_type,
}