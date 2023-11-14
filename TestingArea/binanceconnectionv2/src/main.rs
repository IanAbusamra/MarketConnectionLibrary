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
static BINANCE_WS_API: &str = "wss://stream.binance.us:9443/ws";

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
        "id": "51e2affb-0aba-4821-ba75-f2625006eb43",
        "method": "depth",
        "params": {
          "symbol": "BNBBTC",
          "limit": 5
        }
    }).to_string();

    socket.write_message(Message::Text(depth_subscription)).expect("Failed to subscribe to market depth");

    loop {
        let msg = socket.read_message().expect("Error reading message");

        match msg {
            Message::Ping(ping_data) => {
                println!("Received Ping: {:?}", ping_data);
                socket.write_message(Message::Pong(ping_data)).expect("Error sending pong");
            },
            Message::Binary(data) => {
                println!("Received binary data: {:?}", data);

                // Attempt to decompress the data using a GZIP decoder
                let mut decoder = GzDecoder::new(&data[..]);
                let mut decompressed_data = Vec::new();
                match decoder.read_to_end(&mut decompressed_data) {
                    Ok(_) => {
                        // println!("Decompressed data: {:?}", decompressed_data);
                        
                        // Convert decompressed data to text
                        let text = String::from_utf8(decompressed_data).expect("Found invalid UTF-8");
                        println!("Decompressed text: {}", text);

                        // Respond to pings
                        if let Ok(parsed) = serde_json::from_str::<Value>(&text) {
                            if let Some(ping) = parsed.get("ping") {
                                let pong_response = json!({ "pong": ping }).to_string();
                                socket.write_message(Message::Text(pong_response.clone())).expect("Failed to send pong");
                                println!("Sent Pong response: {}", pong_response);
                            }
                        }
                    },
                    Err(e) => {
                        println!("Failed to decompress GZIP data: {:?}", e);
                    }
                }
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