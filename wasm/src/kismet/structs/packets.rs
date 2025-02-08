use rusqlite::Row;
use serde::{Deserialize, Serialize};
use crate::kismet::structs::_base::{FromRow, FromRowResult};

#[derive(Serialize, Deserialize)]
pub struct Packet {
    pub ts_sec: u32,
    pub ts_usec: u32,
    pub phyname: String,
    pub sourcemac: String,
    pub destmac: String,
    pub transmac: String,
    pub frequency: f64,
    pub devkey: String,
    pub lat: f64,
    pub lon: f64,
    pub packet_len: u16,
    pub packet_len_full: u16,
    pub signal: u8,
    pub datasource: String,
    pub dlt: u16,
    pub error: bool,
    pub tags: String,
    pub datarate: f64,
    pub hash: u32,
    pub packetid: u32,
}

impl FromRow for Packet {
    fn from_row(row: &Row) -> FromRowResult<Self> {
        Ok(Packet {
            ts_sec: row.get(0)?,
            ts_usec: row.get(1)?,
            phyname: row.get(2)?,
            sourcemac: row.get(3)?,
            destmac: row.get(4)?,
            transmac: row.get(5)?,
            frequency: row.get(6)?,
            devkey: row.get(7)?,
            lat: row.get(8)?,
            lon: row.get(9)?,
            packet_len: row.get(10)?,
            packet_len_full: row.get(11)?,
            signal: row.get(12)?,
            datasource: row.get(13)?,
            dlt: row.get(14)?,
            error: row.get(16)?,
            tags: row.get(17)?,
            datarate: row.get(18)?,
            hash: row.get(19)?,
            packetid: row.get(20)?,
        })
    }
}

impl Default for Packet {
    fn default() -> Self {
        Packet {
            ts_sec: 0,
            ts_usec: 0,
            phyname: "".to_string(),
            sourcemac: "".to_string(),
            destmac: "".to_string(),
            transmac: "".to_string(),
            frequency: 0.0,
            devkey: "".to_string(),
            lat: 0.0,
            lon: 0.0,
            packet_len: 0,
            packet_len_full: 0,
            signal: 0,
            datasource: "".to_string(),
            dlt: 0,
            error: false,
            tags: "".to_string(),
            datarate: 0.0,
            hash: 0,
            packetid: 0,
        }
    }
}
