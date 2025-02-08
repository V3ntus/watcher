use std::fmt::Formatter;
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use crate::kismet::structs::_base::{FromRow, FromRowResult};

#[derive(Serialize, Deserialize)]
pub struct DataSource {
    pub uuid: String,
    pub typestring: String,
    pub definition: String,
    pub name: String,
    pub interface: String,
}

impl FromRow for DataSource {
    fn from_row(row: &Row) -> FromRowResult<Self> {
        Ok(DataSource {
            uuid: row.get(0)?,
            typestring: row.get(1)?,
            definition: row.get(2)?,
            name: row.get(3)?,
            interface: row.get(4)?,
        })
    }
}

impl std::fmt::Display for DataSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<DataSource uuid: {}, typestring: {}, definition: {}, name: {}, interface: {}>",
               self.uuid,
               self.typestring,
               self.definition,
               self.name,
               self.interface,
        )
    }
}

impl Default for DataSource {
    fn default() -> Self {
        DataSource {
            uuid: "".to_string(),
            typestring: "".to_string(),
            definition: "".to_string(),
            name: "none".to_string(),
            interface: "none".to_string(),
        }
    }
}
