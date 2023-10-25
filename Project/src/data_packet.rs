pub trait DataPacket {
    fn get_data(&self) -> &str;
}