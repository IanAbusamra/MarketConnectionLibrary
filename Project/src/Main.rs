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
use crate::data_packet::DataEnum;
use crate::data_packet::BestBidAskDataBTCBinance;
use tokio;

static BINANCE_WS_API: &str = "wss://stream.binance.us:9443";

#[tokio::main]
async fn main() {
    //will need to change this url depending upon what data we need
    let binance_url = format!("{}/ws/ethbtc@depth5@100ms", BINANCE_WS_API);

    let mut websocket = WebSocket::new(&binance_url);
    
    let mut binance_listener = BinanceExchangeListener::new(1, &mut websocket);

    binance_listener.subscribe().await;

    let mut cnt = 0;
    loop {
        let message = match binance_listener.get_subscription().receive().await {
            Ok(Some(message)) => Some(message),
            Ok(None) => None,
            Err(e) => {
                println!("Error receiving message: {:?}", e);
                None
            }
        };
    
        binance_listener.on_message(message.as_deref()).await;
    
        if let Some(data_packet) = binance_listener.next().await {
            match data_packet.Data {
                DataEnum::BBABinanceBTCData(bba_data) => {
                    let bestask_value = bba_data.bestask;
                    println!("Best Ask: {}", bestask_value);
                }
                DataEnum::BBABinanceETHData(_) => {
                    println!("Placeholder");
                }
            }
        }
    
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        cnt += 1;
        if cnt == 5 {
            break;
        }
    }
}
