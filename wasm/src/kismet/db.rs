use std::ffi::c_int;
use std::ptr::NonNull;
use rusqlite::{Connection, DatabaseName, Result};
use rusqlite::serialize::OwnedData;
use wasm_bindgen::{throw_str, UnwrapThrowExt};
use kismet::structs::kismet_log::KismetLog;
use crate::kismet;
use crate::kismet::structs::_base::{FromRow};

fn select_from<T: FromRow>(conn: &Connection, table: &str) -> Result<Vec<T>> {
    log::info!("Querying data from {}...", table);
    let mut stmt = conn.prepare(format!("SELECT * FROM {}", table).as_str())?;
    let data_iter = stmt.query_map([], |row| {
        Ok(T::from_row(row))
    })?;

    Ok(data_iter.map(|x| {
        match x {
            Ok(v) => match v {
                Ok(v) => v,
                Err(e) => throw_str(e.to_string().as_str()),
            },
            Err(e) => {
                log::error!("Error occurred mapping iterator {}", e);
                throw_str(e.to_string().as_str())
            }
        }
    }).collect())
}

fn kismet_db_check(conn: &Connection) -> Result<u8> {
    log::info!("Checking database version...");
    match conn.prepare("SELECT db_version FROM KISMET") {
        Ok(mut r) => {
            let db_version: Result<u8> = r.query_row([], |row| {
                Ok(row.get(0)?)
            });
            match db_version {
                Ok(v) => {
                    if (v >= 8) && (v <= 9) {
                        Ok(v)
                    } else {
                        const DB_INCOMPATIBLE: &str = "Kismet log database version not compatible: db_version must be >= 8 && <=9";
                        log::error!("{}", DB_INCOMPATIBLE);
                        throw_str(DB_INCOMPATIBLE)
                    }
                }
                Err(e) => throw_str(e.to_string().as_str()),
            }
        }
        Err(_) => throw_str("Could not prepare Kismet log database check SQL statement")
    }
}

pub fn load_kismetdb(db_bytes: &[u8]) -> Result<KismetLog> {
    log::info!("Deserializing sqlite byte array into memory...");
    let mut conn = Connection::open_in_memory().expect_throw("Could not open SQLite database in memory");
    unsafe {
        let ptr = rusqlite::ffi::sqlite3_malloc(db_bytes.len() as c_int);

        if ptr.is_null() {
            throw_str(rusqlite::Error::SqliteFailure(rusqlite::ffi::Error::new(1), None).to_string().as_str());
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

    kismet_db_check(&conn).expect_throw("Unable to check Kismet log database version");

    let final_struct = KismetLog {
        data: select_from(&conn, "data")?,
        datasources: select_from(&conn, "datasources")?,
        devices: select_from(&conn, "devices")?,
        messages: select_from(&conn, "messages")?,
        packets: select_from(&conn, "packets")?,
        snapshots: select_from(&conn, "snapshots")?,
    };
    log::info!("Successfully extracted all from Kismet log");
    Ok(final_struct)
}
