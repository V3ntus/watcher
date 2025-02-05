use std::fmt::Formatter;

pub struct Data {
    pub ts_sec: u32,
    pub ts_usec: u32,
    pub phyname: String,
    pub devmac: String,
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Data ts_sec: {}, ts_usec: {}, phyname: {}, devmac: {}>", self.ts_sec, self.ts_usec, self.phyname, self.devmac)
    }
}

impl Default for Data {
    fn default() -> Self {
        Data {
            ts_sec: 0,
            ts_usec: 0,
            phyname: "none".to_string(),
            devmac: "none".to_string(),
        }
    }
}
