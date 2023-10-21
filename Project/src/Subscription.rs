pub struct Subscription {
    data_structure: DataStructure,
    exchange_listener: ExchangeListener,
    attribute: String,
}

impl Subscription {
    pub fn new(data_structure: DataStructure, exchange_listener: ExchangeListener, attribute: String) -> Self {
        Self {
            data_structure, 
            exchange_listener,
            attribute
        }
    }
}