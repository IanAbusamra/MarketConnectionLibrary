// /* 
// use async_trait::async_trait;
// use crate::exchange_listener::ExchangeListener;
// //use crate::market_data::MarketData;
// use crate::web_socket::WebSocket;
// use crate::data_packet::DataPacket;
// use crate::data_packet::DataEnum;
// use tokio_tungstenite::tungstenite::Error as TungsteniteError;
// use crate::data_packet::BestBidAskDataBTCBinance;
// use crate::data_packet::BestBidAskDataBTCHuobi;
// use serde_json::{Value, json};
// use tungstenite::{connect, Message};
// use url::Url;
// use flate2::read::GzDecoder;
// use std::io::Read;
// pub struct HuobiExchangeListener<'a> {
//     id: i32,
//     subscription: &'a mut WebSocket,
// }

// impl<'a> HuobiExchangeListener<'a> {
//     pub fn new(id: i32, subscription: &'a mut WebSocket) -> Self {
//         HuobiExchangeListener { id, subscription }
//     }

//     pub fn get_subscription(&mut self) -> &mut WebSocket {
//         &mut self.subscription
//     }
// }

// #[async_trait]
// impl<'a> ExchangeListener for HuobiExchangeListener<'a> {
//     async fn subscribe(&mut self) {
//         self.subscription.connect().await.expect("Failed to connect");
//         println!("Subscribed to Huobi WebSocket");
//     }

//     async fn unsubscribe(&mut self) {
//         self.subscription.close().await.expect("Failed to close connection");
//         println!("Unsubscribed from Huobi WebSocket");
//     }

//     fn parse_message(&self, message: &str) -> Box<DataPacket> {
//         let parsed_data: serde_json::Value = serde_json::from_str(message).expect("Unable to parse message");

//         let enum_creator = BestBidAskDataBTCHuobi {
//             bestask: parsed_data["tick"]["asks"][0][0].as_str().expect("Issue parsing JSON").parse().unwrap(),
//             askamt: parsed_data["tick"]["asks"][0][1].as_str().expect("Issue parsing JSON").parse().unwrap(),
//             bestbid: parsed_data["tick"]["bids"][0][0].as_str().expect("Issue parsing JSON").parse().unwrap(),
//             bidamt: parsed_data["tick"]["bids"][0][1].as_str().expect("Issue parsing JSON").parse().unwrap(),
//         };

//         let ret = DataPacket {
//             Data: DataEnum::BBAHuobiBTCData(enum_creator),
//             Exchange: String::from("Huobi"),
//             Channel: String::from("Channel 2"),
//             timestamp: parsed_data["ts"].as_str().expect("Issue parsing JSON").parse().unwrap()
//         };
//         Box::new(ret)
//     }

//     // No longer necessary



//     async fn on_message(&mut self, message: Option<Message>) {
//         if let Some(msg) = message {
//             match msg {
//                 Message::Ping(ping_data) => {
//                     println!("Received Ping: {:?}", ping_data);
//                     // We don't need to serialize the pong message, just send back the ping data.
//                     self.subscription.send(&serde_json::to_string(&ping_data).unwrap()).await.expect("Error sending pong");
//                 },
//                 Message::Binary(data) => {
//                     println!("Received binary data: {:?}", data);

//                     // Attempt to decompress the data using a GZIP decoder
//                     let mut decoder = GzDecoder::new(&data[..]);
//                     let mut decompressed_data = Vec::new();
//                     match decoder.read_to_end(&mut decompressed_data) {
//                         Ok(_) => {
//                             // Convert decompressed data to text
//                             let text = String::from_utf8(decompressed_data).expect("Found invalid UTF-8");
//                             println!("Decompressed text: {}", text);

//                             // Process the decompressed text, which should be JSON
//                             if let Ok(parsed) = serde_json::from_str::<Value>(&text) {
//                                 if let Some(ping) = parsed.get("ping") {
//                                     let pong_response = json!({ "pong": ping }).to_string();
//                                     self.subscription.send(&pong_response).await.expect("Failed to send pong");
//                                     println!("Sent Pong response: {}", pong_response);
//                                 } else {

//                                     // Process other JSON messages
//                                 }
//                             } else {
//                                 eprintln!("Failed to parse decompressed text as JSON.");
//                             }
//                         },
//                         Err(e) => {
//                             println!("Failed to decompress GZIP data: {:?}", e);
//                         }
//                     }
//                 },
//                 Message::Text(text) => {
//                     println!("Received text: {}", text);
//                     // Here you would handle the text message
//                 },
//                 _ => {
//                     // Optionally handle any other message types
//                     println!("Received other message type");
//                 }
//                 // Handle other message variants if necessary
//             }
//         } else {
//             println!("No message received");
//         }
//     }


//     async fn next(&mut self) -> Option<Box<DataPacket>> {
//         match self.subscription.receive().await {
//             Ok(Some(message)) => {
//                 // We expect that the message is a JSON string here, whether it was originally text or binary.
//                 if let Ok(parsed) = serde_json::from_str::<Value>(&message) {
//                     println!("parsed: {}", parsed);
//                     if parsed.get("ping").is_some() {
//                         // If it's a ping message, construct a pong response.
//                         println!("Received Ping: {:?}", parsed);
//                         let pong_response = json!({ "pong": parsed["ping"] }).to_string();
//                         // Send the pong response asynchronously and ignore errors here.
//                         let _ = self.subscription.send(&pong_response).await;
//                         println!("Sent Pong response: {}", pong_response);
//                         None
//                     } else {
//                         // Parse the message as a data packet if it's not a ping.
//                         println!("Received non-ping/pong message: {}", message);
//                         Some(self.parse_message(&message))
//                     }
//                 } else {
//                     // If the message is not valid JSON, print an error message.
//                     println!("Received message that was not valid JSON: {}", message);
//                     None
//                 }
//             },
//             Ok(None) => {
//                 // No new messages are available at the moment.
//                 None
//             },
//             Err(e) => {
//                 // An error occurred while trying to receive a message.
//                 println!("Error receiving message: {:?}", e);
//                 None
//             }
//         }
//     }

//     fn set_id(&mut self, new_id: i32) {
//         self.id = new_id;
//     }

//     fn get_id(&self) -> i32 {
//         self.id
//     }
// }

// */

// use async_trait::async_trait;
// use crate::exchange_listener::ExchangeListener;
// use crate::web_socket::WebSocket;
// use crate::data_packet::*;
// use crate::data_packet::SymbolEnum::*;
// use crate::data_packet::ExchangeEnum::*;
// use futures::task::{Context, Poll, noop_waker_ref};
// use std::pin::Pin;
// use futures_util::Stream;

// pub struct HuobiExchangeListener<'a> {
//     id: i32,
//     subscription: &'a mut WebSocket,
//     poll_counter: u32,
// }

// impl<'a> HuobiExchangeListener<'a> {
//     pub fn new(id: i32, subscription: &'a mut WebSocket) -> Self {
//         HuobiExchangeListener { id, subscription, poll_counter: 0, }
//     }

//     pub fn get_subscription(&mut self) -> &mut WebSocket {
//         &mut self.subscription
//     }
// }

// #[async_trait]
// impl<'a> ExchangeListener for HuobiExchangeListener<'a> {
//     async fn subscribe(&mut self) {
//         self.subscription.connect().await.expect("Failed to connect");
//         println!("Subscribed to Huobi WebSocket");
//     }

//     async fn unsubscribe(&mut self) {
//         self.subscription.close().await.expect("Failed to close connection");
//         println!("Unsubscribed from Huobi WebSocket");
//     }

//     fn parse_message(&self, message: &str) -> Box<DataPacket> {
//         let parsed_data: serde_json::Value = serde_json::from_str(message).expect("Unable to parse message");
    
//         let enum_creator = MarketIncremental {
//             bestask: parsed_data["asks"][0][0].as_str().expect("Issue parsing JSON").parse().unwrap(),
//             askamount: parsed_data["asks"][0][1].as_str().expect("Issue parsing JSON").parse().unwrap(),
//             bestbid: 0.0,
//             bidamount: 0.0, //just for testing
//         };

//         let ret = DataPacket {
//             data: DataEnum::MBP(enum_creator),
//             exchange: Huobi,
//             symbol_pair: BTCUSD,
//             channel: String::from("Channel 1"),
//             timestamp: 0,
//         };
//         Box::new(ret)
//     }

//     async fn poll(&mut self) -> Option<()> {
//         let waker = noop_waker_ref();
//         let mut context = Context::from_waker(&waker);

//         self.poll_counter += 1;

//         if self.poll_counter % 50 == 0 {
//             self.subscription.send_ping();
//         }

//         if let Some(socket) = self.get_subscription().get_mut_socket() {
//             let socket = Pin::new(socket);

//             match socket.poll_next(&mut context) {
//                 Poll::Ready(Some(Ok(message))) => {
//                     let data_packet = self.parse_message(&message.to_string());
//                     match data_packet.data {
//                         DataEnum::MBP(bba_data) => {
//                             let bestask_value = bba_data.bestask;
//                             println!("Best Ask: {}", bestask_value);
//                         }
//                         DataEnum::RBA(_) => {
//                             println!("Received RBA data.");
//                         }
//                     }

//                     Some(())
//                 },
//                 Poll::Ready(Some(Err(e))) => {
//                     println!("Error receiving message: {:?}", e);
//                     None
//                 },
//                 Poll::Ready(None) => {
//                     println!("Socket closed.");
//                     None
//                 },
//                 Poll::Pending => {
//                     println!("Waiting...");
//                     None
//                 }
//             }
//         } else {
//             println!("WebSocket is not connected.");
//             None
//         }
//     }

//     fn set_id(&mut self, new_id: i32) {
//         self.id = new_id;
//     }

//     fn get_id(&self) -> i32 {
//         self.id
//     }
// }
