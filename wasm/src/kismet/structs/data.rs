use std::fmt::Formatter;
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use crate::kismet::structs::_base::{FromRow, FromRowResult};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub ts_sec: u32,
    pub ts_usec: u32,
    pub phyname: String,
    pub devmac: String,
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub speed: f64,
    pub heading: f64,
    pub datasource: String,
    pub _type: String,
}

impl FromRow for Data {
    fn from_row(row: &Row) -> FromRowResult<Self> {
        Ok(Data {
            ts_sec: row.get(0)?,
            ts_usec: row.get(1)?,
            phyname: row.get(2)?,
            devmac: row.get(3)?,
            lat: row.get(4)?,
            lon: row.get(5)?,
            alt: row.get(6)?,
            speed: row.get(7)?,
            heading: row.get(8)?,
            datasource: row.get(9)?,
            _type: row.get(10)?,
        })
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Data ts_sec: {}, ts_usec: {}, phyname: {}, devmac: {}, lat: {}, lon: {}, \
        alt: {}, speed: {}, heading: {}, datasource: {}, type: {}>",
               self.ts_sec,
               self.ts_usec,
               self.phyname,
               self.devmac,
               self.lat,
               self.lon,
               self.alt,
               self.speed,
               self.heading,
               self.datasource,
               self._type,
        )
    }
}

impl Default for Data {
    fn default() -> Self {
        Data {
            ts_sec: 0,
            ts_usec: 0,
            phyname: "".to_string(),
            devmac: "".to_string(),
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
            speed: 0.0,
            heading: 0.0,
            datasource: "".to_string(),
            _type: "".to_string(),
        }
    }
}
