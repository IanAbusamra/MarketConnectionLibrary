mod binance_exchange_listener;
mod data_packet;
mod market_data;
mod trade_data;
mod web_socket;
mod exchange_listener;

use crate::binance_exchange_listener::BinanceExchangeListener;
use crate::data_packet::DataPacket;
use crate::web_socket::WebSocket;
use crate::exchange_listener::ExchangeListener;

fn main() {
    let websocket = WebSocket::new("wss://stream.binance.com:9443/ws/btcusdt@trade");
    let mut binance_listener = BinanceExchangeListener::new(1, "Binance".to_string(), websocket);

    binance_listener.subscribe();

    loop {
        binance_listener.on_message();

        if let Some(data_packet) = binance_listener.next() {
            println!("Received data: {}", data_packet.get_data());
        }

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
