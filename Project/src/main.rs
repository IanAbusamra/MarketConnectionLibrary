mod binance_exchange_listener;
mod huobi_exchange_listener;
mod data_packet;
mod web_socket;
mod exchange_listener;

use crate::binance_exchange_listener::BinanceExchangeListener;
use crate::huobi_exchange_listener::HuobiExchangeListener;

use crate::data_packet::*;
use crate::web_socket::WebSocket;
use crate::exchange_listener::ExchangeListener;
use tokio::time::{sleep, Duration};
use tokio;
use serde_json::{Value, json};

static BINANCE_WS_API: &str = "wss://stream.binance.us:9443";
static HUOBI_WS_API: &str = "wss://api.huobi.pro/ws";

#[tokio::main(flavor = "current_thread")]
async fn main() {
 
    //will need to change this url depending upon what data we need
    let binance_url = format!("{}/ws/ethbtc@depth5@100ms", BINANCE_WS_API);

    let mut binance_websocket = WebSocket::new(&binance_url);

    let mut binance_listener: BinanceExchangeListener<'_> = BinanceExchangeListener::new(1, &mut binance_websocket);

    binance_listener.subscribe().await;

    let mut cnt = 0;
    loop {
        binance_listener.poll();
        //sleep(Duration::from_millis(1000)).await;
        cnt += 1;
        if cnt == 10 {
            break;
        }
    }
    
    /*
    let huobi_url = format!("{}", HUOBI_WS_API);

    let mut huobi_websocket = WebSocket::new(&huobi_url);

    let depth_subscription = json!({
        "sub": "market.btcusdt.mbp.refresh.20",
        "id": "id1"
    }).to_string();

    let mut huobi_listener = HuobiExchangeListener::new(1, &mut huobi_websocket);
    huobi_listener.subscribe().await;

    huobi_websocket.send(&depth_subscription).await.expect("Failed to subscribe to market depth");

    let mut huobi_listener: HuobiExchangeListener<'_> = HuobiExchangeListener::new(1, &mut huobi_websocket);

    let mut cnt = 0;
    loop {
        huobi_listener.poll();
        
        sleep(Duration::from_millis(1)).await;
    }
    */
}