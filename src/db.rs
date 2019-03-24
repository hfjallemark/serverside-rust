use postgres::rows::Row;
use postgres::types::ToSql;
use postgres::{Connection, Result};

macro_rules! sql {
    ($file: expr) => {
        include_str!(concat!("sql/", $file, ".sql"));
    };
}

pub mod posts;

#[database("db")]
pub struct Db(Connection);

impl Db {
    fn query_list<T, F: Fn(Row) -> T>(
        self: &Db,
        query: &str,
        params: &[&ToSql],
        op: F,
    ) -> Result<Vec<T>> {
        self.query(query, params).map(|rows| rows.into_iter().map(op).collect::<Vec<T>>())
    }

    fn query_one<T, F: Fn(Row) -> T>(
        self: &Db,
        query: &str,
        params: &[&ToSql],
        op: F,
    ) -> Result<T> {
        self.query(query, params).map(|rows| op(rows.get(0)))
    }
}
