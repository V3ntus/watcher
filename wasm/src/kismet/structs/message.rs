use std::fmt::Formatter;
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use crate::kismet::structs::_base::{FromRow, FromRowResult};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub ts_sec: u32,
    pub lat: f64,
    pub lon: f64,
    pub msgtype: String,
    pub message: String,
}

impl FromRow for Message {
    fn from_row(row: &Row) -> FromRowResult<Self> {
        Ok(Message {
            ts_sec: row.get(0)?,
            lat: row.get(1)?,
            lon: row.get(2)?,
            msgtype: row.get(3)?,
            message: row.get(4)?,
        })
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Message ts_sec: {}, lat: {}, lon: {}, \
        msgtype: {}, message: {}>",
               self.ts_sec,
               self.lat,
               self.lon,
               self.msgtype,
               self.message,
        )
    }
}

impl Default for Message {
    fn default() -> Self {
        Message {
            ts_sec: 0,
            lat: 0.0,
            lon: 0.0,
            msgtype: "".to_string(),
            message: "".to_string(),
        }
    }
}
