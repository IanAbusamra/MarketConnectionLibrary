pub struct ExchangeListenerSpecific {
    id: i32,
    subscription: WebSocket,
    queue: Queue<DataPacket>,
}