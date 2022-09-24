#[derive(Debug)]
pub struct Host {
    pub id: usize,
    pub ip: String,
    pub hostname: String,
}

impl Host {
    pub fn new(id: usize, ip: String, hostname: String) -> Host {
        Host {
            id,
            ip,
            hostname,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.ip, self.hostname)
    }

    pub fn to_vec(&self) -> Vec<String> {
        vec![self.id.to_string(), self.ip.clone(), self.hostname.clone()]
    }
}
