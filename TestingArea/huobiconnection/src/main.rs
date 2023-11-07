use tungstenite::connect;
use url::Url;
use flate2::write::GzEncoder;
use flate2::{read, Compression};
use std::io;
use std::io::prelude::*;

//static HUOBI_WS_API: &str = "wss://api.huobi.pro/feed";
// static HUOBI_WS_API: &str = "wss://api.huobi.pro/market/trade?symbol=ethusdt";
static HUOBI_WS_API: &str = "wss://api.huobi.pro/market.$BTC.bbo";

fn main() {
    //let binance_url = format!("{}/ws/ethbtc@depth5@100ms", BINANCE_WS_API);

    let (mut socket, response) =
        connect(Url::parse(HUOBI_WS_API).unwrap()).expect("Can't connect.");

    println!("Connected to Huobi stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");
    for (ref header, ref header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }

    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("{}", msg);
        //let msg1 = msg;
        //let msg1 = msg.into();
        // if (msg.is_ping()) {
        //     socket.write_message(msg);
        // }
        //println!("{}", decode_reader(msg.into()).unwrap());
        //std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
    let mut gz = read::GzDecoder::new(&bytes[..]);
    let mut s = String::new();
    gz.read_to_string(&mut s)?;
    Ok(s)
}