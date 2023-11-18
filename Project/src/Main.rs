mod binance_exchange_listener;
mod data_packet;
mod market_data;
mod trade_data;
mod web_socket;
mod exchange_listener;

use crate::binance_exchange_listener::BinanceExchangeListener;
use crate::data_packet::*;
use crate::web_socket::WebSocket;
use crate::exchange_listener::ExchangeListener;
use tokio::time::{sleep, Duration};
use tokio;

static BINANCE_WS_API: &str = "wss://stream.binance.us:9443";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    //will need to change this url depending upon what data we need
    let binance_url = format!("{}/ws/ethbtc@depth5@100ms", BINANCE_WS_API);

    let mut websocket = WebSocket::new(&binance_url);

    let mut binance_listener = BinanceExchangeListener::new(1, &mut websocket);

    binance_listener.subscribe().await;

    let mut cnt = 0;
    loop {
        binance_listener.poll().await;
        
        sleep(Duration::from_millis(1000)).await;
        cnt += 1;
        if cnt == 10 {
            break;
        }
    }
}