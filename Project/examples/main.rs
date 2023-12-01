use market_connection::ExchangeListener;
use market_connection::HuobiExchangeListener;
use market_connection::BinanceExchangeListener;
use market_connection::WebSocket;
use serde_json::json;

use tokio::time::{sleep, Duration};

static BINANCE_WS_API: &str = "wss://stream.binance.us:9443";
static HUOBI_WS_API: &str = "wss://api.huobi.pro/ws";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Binance initialization
    // let binance_url = format!("{}/ws/ethbtc@depth5@100ms", BINANCE_WS_API);
    // let mut binance_websocket = WebSocket::new(&binance_url);
    // let mut binance_listener: BinanceExchangeListener<'_> = BinanceExchangeListener::new(1, &mut binance_websocket);
    // binance_listener.subscribe().await;

    // Huobi Initialization
    let huobi_url = format!("{}", HUOBI_WS_API);
    let mut huobi_websocket = WebSocket::new(&huobi_url);
    let depth_subscription = json!({
        //"sub": "market.btcusdt.mbp.refresh.20",
        "sub": "market.btcusdt.mbp.150",
        "id": "id1"
    })
    .to_string();
    let mut huobi_listener = HuobiExchangeListener::new(1, &mut huobi_websocket);
    huobi_listener.subscribe().await;
    huobi_websocket
        .send(&depth_subscription)
        .await
        .expect("Failed to subscribe to market depth");
    let mut huobi_listener: HuobiExchangeListener<'_> =
        HuobiExchangeListener::new(1, &mut huobi_websocket);

    // Example Event Loop
    let mut cnt = 0;
    loop {
        huobi_listener.poll();
        //binance_listener.poll();

        sleep(Duration::from_millis(5)).await;
        // if cnt < 1 {
        //     sleep(Duration::from_millis(500)).await;
        // }
        // cnt = cnt + 1;
    }
}
