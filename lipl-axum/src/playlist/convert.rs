use super::sql;
use lipl_types::Summary;
use tokio_postgres::Row;

pub fn to_list<F, T>(f: F) -> impl Fn(Vec<Row>) -> Vec<T>
where
    F: Fn(Row) -> T + Copy,
{
    move |rows| rows.into_iter().map(f).collect::<Vec<_>>()
}

pub fn to_summary(row: Row) -> Summary {
    Summary {
        id: row
            .get::<&str, uuid::Uuid>(sql::column::ID)
            .into(),
        title: row.get::<&str, String>(sql::column::TITLE),
    }
}
