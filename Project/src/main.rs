mod binance_exchange_listener;
mod data_packet;
mod web_socket;
mod exchange_listener;

use crate::binance_exchange_listener::BinanceExchangeListener;
use crate::data_packet::*;
use crate::web_socket::WebSocket;
use crate::exchange_listener::ExchangeListener;
use tokio::time::{sleep, Duration};
use tokio;
use serde_json::{Value, json};

static BINANCE_WS_API: &str = "wss://api.huobi.pro/ws";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    //will need to change this url depending upon what data we need
    let binance_url = format!("{}", BINANCE_WS_API);

    let mut websocket = WebSocket::new(&binance_url);

    let mut binance_listener = BinanceExchangeListener::new(1, &mut websocket);

    binance_listener.subscribe().await;

    println!("WEBSOCKET CONNECTION ESTABLISHED");
    //sleep(Duration::from_millis(1000)).await;
    let depth_subscription = json!({
        "sub": "market.btcusdt.depth.step0",
        "id": "id1"
    }).to_string();
    binance_listener.subscription.send(&depth_subscription);
    //sleep(Duration::from_millis(1000)).await;
    let mut cnt = 0;
    loop {
        binance_listener.poll();
        
        sleep(Duration::from_millis(100)).await;
        cnt += 1;
        // if cnt % 10 == 0 {
        //     println!("SEND ATTEMPT");
        //     println!("");
        //     binance_listener.subscription.send(&depth_subscription);
        // }
        // if cnt == 10 {
        //     break;
        // }
    }
}