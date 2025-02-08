use rusqlite::Row;
use serde::{Deserialize, Serialize};
use crate::kismet::structs::_base::{FromRow, FromRowResult};

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub ts_sec: u32,
    pub ts_usec: u32,
    pub lat: f64,
    pub lon: f64,
    pub snaptype: String,
    pub json: String,
}

impl FromRow for Snapshot {
    fn from_row(row: &Row) -> FromRowResult<Self> {
        Ok(Snapshot {
            ts_sec: row.get(0)?,
            ts_usec: row.get(1)?,
            lat: row.get(2)?,
            lon: row.get(3)?,
            snaptype: row.get(4)?,
            json: row.get(5)?,
        })
    }
}

impl Default for Snapshot {
    fn default() -> Self {
        Snapshot {
            ts_sec: 0,
            ts_usec: 0,
            lat: 0.0,
            lon: 0.0,
            snaptype: "".to_string(),
            json: "".to_string(),
        }
    }
}
