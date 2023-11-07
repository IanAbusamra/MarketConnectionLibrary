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

use crate::data_packet::MessageType2;

static BINANCE_WS_API: &str = "wss://stream.binance.us:9443";

fn main() {
    let binance_url = format!("{}/ws/ethbtc@depth5@100ms", BINANCE_WS_API);

    let mut websocket = WebSocket::new(&binance_url);
    
    let mut binance_listener = BinanceExchangeListener::new(1, &mut websocket);

    binance_listener.subscribe();

    let mut cnt = 0;
    loop {
        let message = match binance_listener.get_subscription().receive() {
            Ok(Some(message)) => Some(message),
            Ok(None) => None,
            Err(e) => {
                println!("Error receiving message: {:?}", e);
                None
            }
        };
    
        binance_listener.on_message(message.as_deref());
    
        if let Some(data_packet) = binance_listener.next() {
            //trying to actually access value from enum
            // let mut testm2 = match data_packet.Data {
            //     DataPacket::DataEnum::MessageType2(c) => c,
            //     _ => unreachable!()
            // };
            //let testm2: MessageType2 = data_packet.Data;
            println!("{}", "testing");
            //println!("Received data: {}", data_packet.get_data());
            //println!("Formatted version: Best Ask: {}, Best Ask Amount: {}", data_packet.get_best_ask(), data_packet.get_ask_amt());
        }
    
        std::thread::sleep(std::time::Duration::from_millis(1001));
        cnt += 1;
        if cnt == 5 {
            break;
        }
    }
    // binance_listener.close();
}
