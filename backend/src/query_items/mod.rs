
use crate::{db_items::DbItem, ServerPool};

pub trait QueryItem {
    /// Returns query name
    fn query_name() -> String;
}

// pub trait GetById {
//     fn by_id(sql_conn: ServerPool, id: u64) -> Self;
// }

// pub trait GetAll: Sized {
//     fn list_all(sql_conn: ServerPool) -> Vec<Self>;
// }

// pub trait PutModify: DbItem {
//     fn modify(sql_conn: ServerPool, id: u64, new_self: Self);
// }