use tungstenite::connect;
use url::Url;

static HUOBI_WS_API: &str = "wss://api.huobi.pro/feed";

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
        // let msg = match msg {
        //     tungstenite::Message::Text(s) => s,
        //     _ => {
        //          panic!("Error getting text");
        //     }
        //  };
        println!("{}", msg);
        //  let parsed_data: serde_json::Value = serde_json::from_str(&msg).expect("Unable to parse message");
        //  println!("best ask: {}, ask size: {}", parsed_data["asks"][0][0], parsed_data["asks"][0][1]);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}