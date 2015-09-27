//! NSQ Producer

use config::Config;

pub struct Producer {
    host: String,
    config: Config,
}

impl Producer {
    pub fn new(host: &str, config: Config) -> Producer {
        Producer{
            host: host.to_string(),
            config: config,
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
