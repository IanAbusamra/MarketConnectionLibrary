pub struct ExchangeListenerSpecific {
    id: i32,
    name: String,
    subscription: WebSocket,
    queue: Queue<DataPacket>,
}

impl ExchangeListenerSpecific{
    fn subscribe(&self, ws: &WebSocket){

    }
    fn unsubscribe(&self){

    }
    fn onmessage(json){
        
    }
    fn parse_message(&self) -> DataPacket {

    }
    fn add_parsed_data(&self, dp: &DataPacket) {

    }
    fn next(&self) -> Option<DataPacket> {

    }
}