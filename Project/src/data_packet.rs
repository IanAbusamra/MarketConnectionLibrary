pub struct DataPacket {
    //change back to dataenum
    pub DataTest: String,
    pub Exchange: String,
    pub Channel: String,
}

///////////////////////////////////////////////

pub enum DataEnum {
    M1(MessageType1),
    M2(MessageType2),
}

pub struct MessageType1 {
    data: String,
    BestAsk: f64,
    AskAmt: f64,
}

pub struct MessageType2 {
    data: String,
}