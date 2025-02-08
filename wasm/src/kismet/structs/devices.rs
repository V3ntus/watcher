use std::fmt::Formatter;
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use crate::kismet::structs::_base::{FromRow, FromRowResult};

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub first_time: u32,
    pub last_time: u32,
    pub devkey: String,
    pub phyname: String,
    pub devmac: String,
    pub strongest_signal: f64,
    pub min_lat: f64,
    pub min_lon: f64,
    pub max_lat: f64,
    pub max_lon: f64,
    pub avg_lat: f64,
    pub avg_lon: f64,
    pub _type: String,
}

impl FromRow for Device {
    fn from_row(row: &Row) -> FromRowResult<Self> {
        Ok(Device {
            first_time: row.get(0)?,
            last_time: row.get(1)?,
            devkey: row.get(2)?,
            phyname: row.get(3)?,
            devmac: row.get(4)?,
            strongest_signal: row.get(5)?,
            min_lat: row.get(6)?,
            min_lon: row.get(7)?,
            max_lat: row.get(8)?,
            max_lon: row.get(9)?,
            avg_lat: row.get(10)?,
            avg_lon: row.get(11)?,
            _type: row.get(13)?,
        })
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Device first_time: {}, last_time: {}, devkey: {}, phyname: {}, devmac: {}, \
        strongest_signal: {}, min_lat: {}, min_lon: {}, max_lat: {}, max_lon: {}, avg_lat: {}, \
        avg_lon: {}, type: {}>",
               self.first_time,
               self.last_time,
               self.devkey,
               self.phyname,
               self.devmac,
               self.strongest_signal,
               self.min_lat,
               self.min_lon,
               self.max_lat,
               self.max_lon,
               self.avg_lat,
               self.avg_lon,
               self._type,
        )
    }
}

impl Default for Device {
    fn default() -> Self {
        Device {
            first_time: 0,
            last_time: 0,
            devkey: "".to_string(),
            phyname: "".to_string(),
            devmac: "".to_string(),
            strongest_signal: 0.0,
            min_lat: 0.0,
            min_lon: 0.0,
            max_lat: 0.0,
            max_lon: 0.0,
            avg_lat: 0.0,
            avg_lon: 0.0,
            _type: "".to_string(),
        }
    }
}
