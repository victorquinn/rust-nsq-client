//! NSQ Config

/// Obviously most is missing from this, will fill out in time!
pub struct Config {
    timeout: i32,
}

impl Config {
    pub fn new() -> Config {
        // Hard coded for now!
        Config{
            timeout: 120,
        }
    }
}
