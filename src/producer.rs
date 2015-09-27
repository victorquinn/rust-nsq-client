//! NSQ Producer

use config::Config;
use connection::Connection;

pub struct Producer {
    config: Config,
    connection: Connection,
    host: String,
    ready: bool,
}

impl Producer {
    /// Create a new Producer
    pub fn new(host: &str, config: Config) -> Producer {
        // Connect to the specified nsqd
        Producer{
            config: config,
            connection: Connection::new(host),
            host: host.to_string(),
            ready: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::Config;

    #[test]
    fn test_new() {
        let config = Config::new();
        let producer = Producer::new("localhost:4150", config);
        assert_eq!(producer.host, "localhost:4150");
        assert_eq!(producer.config.timeout, 120);
    }
}
