pub trait DataPacket {
    fn get_data(&self) -> &str;
    fn get_best_ask(&self) -> f64;
    fn get_ask_amt(&self) -> f64;
}