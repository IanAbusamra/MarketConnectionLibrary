// use crate::DataPacket;

// //we can modify the parsing process to make in any format traders would like
// pub struct MarketData {
//     data: String,
//     BestAsk: f64,
//     AskAmt: f64,
// }

// impl MarketData {
//     pub fn new(data: String) -> Self {
//         let parsed_data: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse message");
//         MarketData {
//             data: data,
//             BestAsk: parsed_data["asks"][0][0].as_str().expect("Issue parsing JSON").parse().unwrap(),
//             AskAmt: parsed_data["asks"][0][1].as_str().expect("Issue parsing JSON").parse().unwrap(),
//         }
//     }
// }

// impl DataPacket for MarketData {
//     fn get_data(&self) -> &str {
//         &self.data
//     }

//     fn get_best_ask(&self) -> f64 {
//         self.BestAsk
//     }

//     fn get_ask_amt(&self) -> f64 {
//         self.AskAmt
//     }
// }
