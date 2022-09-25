#[derive(Debug)]
pub struct Host {
    pub id: usize,
    pub ip: String,
    pub hostname: String,
    pub line_number: usize,
}

impl Host {
    pub fn new(id: usize, ip: String, hostname: String, line_number: usize) -> Host {
        Host {
            id,
            ip,
            hostname,
            line_number,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.ip, self.hostname)
    }

    pub fn to_vec(&self) -> Vec<String> {
        vec![self.id.to_string(), self.ip.clone(), self.hostname.clone()]
    }
}
