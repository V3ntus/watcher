use std::ffi::c_int;
use std::ptr::NonNull;
use rusqlite::{Connection, DatabaseName, Result};
use rusqlite::serialize::OwnedData;
use wasm_bindgen::throw_str;
use crate::kismet;
use crate::kismet::structs::Data;

pub fn load_kismetdb(db_bytes: &[u8]) -> Result<Vec<Data>> {
    let mut conn = Connection::open_in_memory()?;
    unsafe {
        let ptr = rusqlite::ffi::sqlite3_malloc(db_bytes.len() as c_int);

        if ptr.is_null() {
            return Err(rusqlite::Error::SqliteFailure(rusqlite::ffi::Error::new(1), None));
        }

        std::ptr::copy_nonoverlapping(db_bytes.as_ptr(), ptr as *mut u8, db_bytes.len());

        let owned_data = OwnedData::from_raw_nonnull(NonNull::new_unchecked(ptr as *mut u8), db_bytes.len());

        conn
            .deserialize(
                DatabaseName::Main,
                owned_data,
                true,
            )?;
    }

    let mut stmt = conn.prepare("SELECT * FROM data")?;
    let data_iter = stmt.query_map([], |row| {
        Ok(kismet::structs::Data {
            ts_sec: row.get(0)?,
            ts_usec: row.get(1)?,
            phyname: row.get(2)?,
            devmac: row.get(3)?,
        })
    })?;

    Ok(data_iter.map(|x| {
        match x {
            Ok(v) => v,
            Err(e) => throw_str(e.to_string().as_str()),
        }
    }).collect())
}
