pub struct Subscription {
    data_structure: DataStructure, // likely need to use some sort of boxing mechanism
    exchange_listener: impl ExchangeListener,
    attribute: String,
}

impl Subscription {
    pub fn new(data_structure: DataStructure, exchange_listener: impl ExchangeListener, attribute: String) -> Self {
        Self {
            data_structure, 
            exchange_listener,
            attribute
        }
    }
}