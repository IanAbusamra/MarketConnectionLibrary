pub struct SubscriptionManager {
    lookup : Vec<HashMap<String, Vec<Box<dyn Subscription>>>>,
}

impl SubscriptionManager{
    pub fn new() -> SubscriptionManager {
        Self {Vec::new()}
    }
    pub fn add_subscription(&self, s: &Box<dyn Subscription>) {
        while s.exchange_listener.id >= self.lookup.len() {
            self.lookup.push(HashMap::new());
        }

        // no clue if the below will work or not
        *(self.lookup[s.exchange_lister.id].entry(s.attribute).or_insert(Vec::new())).push(s);
    }
    pub fn update_subscriptions(&self, e: &impl ExchangelListener, a: &String, val : &impl DataPacket) {
        // iterate through subscriptions and update according to val
        // details will need to be worked out with other teams
        for sub in self.lookup[e.id][a].iter() {
            *sub.data_structure.add_datapacket(val);        
        }
    }
}