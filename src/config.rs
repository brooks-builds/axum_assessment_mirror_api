pub struct Config {
    pub port: u16,
    pub address: [u8; 4],
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: 3000,
            address: [127, 0, 0, 1],
        }
    }

    pub fn set_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
