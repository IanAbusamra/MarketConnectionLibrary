pub struct DataPacket {
    pub TempBestAsk: String,
    pub TempAskAmt: String,
    //change back to dataenum
    pub Data: DataEnum,
    pub Exchange: String,
    pub Channel: String,
}

///////////////////////////////////////////////

pub enum DataEnum {
    M1(MessageType1),
    M2(MessageType2),
}

pub struct MessageType1 {
    pub data: String,
    pub BestAsk: f64,
    pub AskAmt: f64,
}

pub struct MessageType2 {
    pub bestask: String,
}
