pub struct Payload {
    timestamp: u64,
    ecp: String,
    sender_name: String,
    receiver_name: String,
    amount: u64,
}

impl Payload {
    pub fn new(
        timestamp: u64,
        ecp: impl Into<String>,
        sender_name: impl Into<String>,
        receiver_name: impl Into<String>,
        amount: u64,
    ) -> Self {
        Self {
            timestamp,
            ecp: ecp.into(),
            sender_name: sender_name.into(),
            receiver_name: receiver_name.into(),
            amount,
        }
    }
}

pub struct Order {
    sender: String,
    receiver: String,
    amount: u64,
}

struct Connector;

impl Connector {
    fn new() -> Self {
        Self
    }

    fn send(&self, msg: Order) -> Result<String, String> {
        if msg.sender.is_empty() || msg.receiver.is_empty() {
            return Err("Sender and receiver must not be empty".to_string());
        }
        if msg.amount == 0 {
            return Err("Amount must be greater than zero".to_string());
        }
        Ok("OK".to_string())
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

    mq_connector.send(order)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_transaction_is_sent() {
        let payload = Payload::new(1_700_000_000, "signature", "Alice", "Bob", 42);
        assert_eq!(make_transaction(payload).as_deref(), Ok("OK"));
    }

    #[test]
    fn zero_amount_is_rejected() {
        let payload = Payload::new(1_700_000_000, "signature", "Alice", "Bob", 0);
        assert_eq!(
            make_transaction(payload).unwrap_err(),
            "Amount must be greater than zero"
        );
    }
}
