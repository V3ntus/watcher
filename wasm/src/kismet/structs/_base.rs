use rusqlite::Row;

pub type FromRowResult<T> = Result<T, Box<dyn std::error::Error>>;

pub trait FromRow {
    fn from_row(row: &Row) -> FromRowResult<Self>
    where
        Self: Sized;
}