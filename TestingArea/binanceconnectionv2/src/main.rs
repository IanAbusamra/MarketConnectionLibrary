use tungstenite::{connect, Message};
use url::Url;
use flate2::read::GzDecoder;
use std::io::Read;
use serde_json::{Value, json};

//static HUOBI_WS_API: &str = "wss://api.huobi.pro/feed";
// static HUOBI_WS_API: &str = "wss://api.huobi.pro/market/trade?symbol=ethusdt";
// static HUOBI_WS_API: &str = "wss://api.huobi.pro/market.$BTC.bbo";
//static HUOBI_WS_API: &str = "https://api.huobi.pro/market/history/kline?period=1day&size=200&symbol=btcusdt";
// static HUOBI_WS_API: &str = "wss://api.huobi.pro/market.$BTC$.kline.$1min$";
static BINANCE_WS_API: &str = "wss://stream.binance.us:9443/ws/ethbtc@depth5@100ms";

fn main() {
    let (mut socket, response) =
        connect(Url::parse(BINANCE_WS_API).unwrap()).expect("Can't connect.");

    println!("Connected to the Binance stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");
    for (ref header, ref header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }

    // Subscribe to market depth for BTC/USDT with no aggregation
    let depth_subscription = json!({
        
            "method": "SUBSCRIBE",
            "params":
            [
            "btcusdt@depth"
            ],
            "id": 1
            
    }).to_string();

    socket.write_message(Message::Text(depth_subscription)).expect("Failed to subscribe to market depth");

    loop {
        let msg = socket.read_message().expect("Error reading message");

        match msg {
            Message::Ping(ping_data) => {
                println!("Received Ping: {:?}", ping_data);
                socket.write_message(Message::Pong(ping_data)).expect("Error sending pong");
            },
            Message::Text(text) => {
                println!("Received text: {}", text);
                // Handle text message.
            },
            _ => {
                // Handle other message types
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}