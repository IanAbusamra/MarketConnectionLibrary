pub struct SubscriptionManager {
    lookup : Vec<HashMap<String, Vec<Subscription>>>,
}

impl SubscriptionManager{
    pub fn new() -> SubscriptionManager {
        Self {Vec::new()}
    }
    pub fn add_subscription(&self, s: &Subscription) {
        while s.exchange_listener.id >= self.lookup.len() {
            self.lookup.push(HashMap::new());
        }

        // no clue if the below will work or not
        *(self.lookup[s.exchange_lister.id].entry(s.attribute).or_insert(Vec::new())).push(s);
    }
    pub fn get_subscriptions(&self, e: &impl ExchangeListener, a: &String) -> List<Subscription> {
        self.lookup[e.id][a]
    }
    pub fn update_subscriptions<T:DataPacket>(&self, e: &impl ExchangelListener, a: &String, val : T) {
        let target_subscriptions = self.get_subscriptions(e, a);

        // iterate through subscriptions and update according to val
        // details will need to be worked out with other teams
        for sub in target_subscriptions.iter() {
            *sub.data_structure.update(val);        
        }
    }
}