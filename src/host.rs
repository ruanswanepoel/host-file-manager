pub struct Host {
    pub ip: String,
    pub hostname: String,
}

impl Host {
    pub fn new(ip: String, hostname: String) -> Host {
        Host {
            ip,
            hostname,
        }
    }

    pub fn to_string(&self) -> String {
        format!("IP: {}, Hostname: {}", self.ip, self.hostname)
    }
}
