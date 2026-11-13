pub struct Payload {
    timestamp: u64,
    ecp: String,
    sender_name: String,
    receiver_name: String,
    amount: u64,
}

pub struct Order {
    sender: String,
    receiver: String,
    amount: u64,
}

struct Connector{}

impl Connector {
    fn send(&self, msg: Order) -> Result<String, String> {
        Ok(String::from("OK"))
    }
    fn new() -> Self {
        Self {}
    }
}

pub fn make_transaction(payload: Payload) -> Result<String, String> {
    let mq_connector = Connector::new();
    let order = Order {
        sender: payload.sender_name,
        receiver: payload.receiver_name,
        amount: payload.amount,
    };

    dbg!(payload.timestamp);
    dbg!(payload.ecp);

    if let Ok(res) = mq_connector.send(order) {
        return Ok(res);
    };

    Err(String::from("Not implemented"))
}
