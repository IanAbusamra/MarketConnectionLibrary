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
use futures::task::{Context, Poll};
use futures::stream::Stream;
use futures_util::stream::StreamExt; // Import the StreamExt trait
use std::pin::Pin;
use std::task::Waker;
use tokio::time::{sleep, Duration};
use tokio;

static BINANCE_WS_API: &str = "wss://stream.binance.us:9443";

#[tokio::main]
async fn main() {
    //will need to change this url depending upon what data we need
    let binance_url = format!("{}/ws/ethbtc@depth5@100ms", BINANCE_WS_API);

    let mut websocket = WebSocket::new(&binance_url);

    let mut binance_listener = BinanceExchangeListener::new(1, &mut websocket);

    binance_listener.subscribe().await;

    let waker = futures::task::noop_waker();
    let mut cx = Context::from_waker(&waker);

    let mut cnt = 0;
    loop {
        if let Some(socket) = binance_listener.get_subscription().get_mut_socket() {
            let mut socket = Pin::new(socket);
            match socket.poll_next(&mut cx) {
                Poll::Ready(Some(Ok(message))) => {
                    if let Some(data_packet) = binance_listener.next().await {
                        match data_packet.Data {
                            DataEnum::BBABinanceBTCData(bba_data) => {
                                let bestask_value = bba_data.bestask;
                                println!("Best Ask: {}", bestask_value);
                            }
                            DataEnum::BBABinanceETHData(_) => {
                                println!("Placeholder");
                            }
                            DataEnum::BBAHuobiBTCData(_) => {
                                println!("Placeholder");
                            }
                            DataEnum::BBAHuobiETHData(_) => {
                                println!("Placeholder");
                            }
                        }
                    }
                },
                Poll::Ready(Some(Err(e))) => {
                    println!("Error receiving message: {:?}", e);
                },
                Poll::Ready(None) => break, 
                Poll::Pending => println!("Waiting..."), // No message available
            }
        } else {
            println!("WebSocket is not connected.");
            break;
        }

        sleep(Duration::from_millis(1000)).await;
        cnt += 1;
        if cnt == 10 {
            break;
        }
    }
}