//! NSQ Producer

pub struct Producer {
    host: String,
}

impl Producer {
    pub fn new(host: &str) -> Producer {
        Producer{
            host: host.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let producer = Producer::new("localhost:4150");
        assert_eq!(producer.host, "localhost:4150");
    }
}
