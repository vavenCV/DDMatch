use crate::{routes::query_items::QueryItem, CErr, ServerPool};
use serde::Serialize;
use sqlx::mysql::MySqlRow;
use std::fmt::Debug;

// pub mod category;
pub mod couleur;
pub mod couleur_finale;
pub mod dragodinde;
#[macro_use]
pub mod insert_query;
pub trait DbItem:
    Sized + for<'a> sqlx::FromRow<'a, MySqlRow> + Send + Unpin + Serialize + Debug + Default
{
    /// Returns table name
    fn table_name() -> String;

    /// Returns query name
    fn query_name() -> String;

    /// Returns the item from specified `id` if it exists
    ///
    /// # Arguments
    /// * `id` - the identifier used to find the item
    /// * `sql_conn` - the sql database connection
    ///
    async fn by_id(id: &u64, sql_conn: &ServerPool) -> Option<Self> {
        let table_name = Self::table_name().clone();
        sqlx::query_as("SELECT * FROM $1 WHERE id = $2")
            .bind(&table_name)
            .bind(id)
            .fetch_optional(sql_conn)
            .await
            .ok()?
    }

    /// Returns the item `id`
    ///
    /// # Arguments
    /// * `self` - self
    ///
    fn get_id(self) -> u64;

    /// Creates a random unique id by iterating and verifying that the uuid doesn't already exists in the table
    ///
    /// # Arguments
    /// * `sql_conn` - the sql database connection
    ///
    async fn get_random_unique_id(sql_conn: &ServerPool) -> u64 {
        loop {
            let new_id = uuid::Uuid::new_v4().as_u128() as u64;

            if Self::by_id(&new_id, sql_conn).await.is_none() {
                return new_id;
            }
        }
    }

    /// Insert the item into the table
    ///
    /// # Arguments
    /// * `sql_conn` - the sql database connection
    ///
    /// # Errors
    /// Can error if the insert was not successful
    fn insert_into_db(
        &self,
        sql_conn: &ServerPool,
    ) -> impl std::future::Future<Output = Result<(), CErr>> + std::marker::Send;

    /// Handles the result of the insert into the table
    ///
    /// # Arguments
    /// * `result` - the query result
    ///
    /// # Errors
    /// Will error if the insert was not successful
    fn handle_insert_res(
        &self,
        result: Result<sqlx::mysql::MySqlQueryResult, sqlx::Error>,
    ) -> Result<(), CErr> {
        match result {
            Err(e) => {
                println!("Error inserting into {}: {:#?}", Self::table_name(), self);
                println!("Error message: [{}].\n", e.to_string());
                Err(tide::Error::from_str(
                    tide::StatusCode::InternalServerError,
                    format!("Could not insert into table '{}': {e}", Self::table_name()),
                )
                .into())
            }

            Ok(res) => {
                println!("Success inserting into {}.", Self::table_name());
                println!("Number of items inserted: {}", res.rows_affected());
                Ok(())
            }
        }
    }

    /// Convert from db representation to output JSON value
    ///
    /// # Arguments
    /// * `self` - self
    fn db_to_output(self) -> Result<impl Serialize, CErr>;

    /// Convert from vector of db representation to output vector
    ///
    /// # Arguments
    /// * `input` - self object vector
    fn convert_vector(input: Vec<Self>) -> Result<Vec<impl Serialize>, String> {
        input
            .into_iter()
            .map(|q| Self::db_to_output(q).map_err(|e| e.to_string()))
            .collect()
    }

    /// Get all the items from the table
    ///
    /// # Arguments
    /// * `sql_conn` - the sql database connection
    ///
    /// # Errors
    /// Can error if the query was not successful
    fn get_all_items(
        sql_conn: &ServerPool,
    ) -> impl std::future::Future<Output = tide::Result> + std::marker::Send {
        async move {
            let query_result: Vec<Self> =
                sqlx::query_as::<_, Self>(&format!("SELECT * FROM {}", Self::table_name()))
                    .fetch_all(sql_conn)
                    .await?;

            tide::log::info!("{:?}", query_result);

            let res_vec = Self::convert_vector(query_result).map_err(|e| {
                tide::Error::from_str(tide::StatusCode::InternalServerError, e.to_string())
            })?;

            Ok(tide::Response::from_res(serde_json::to_string(&res_vec)?))
        }
    }

    fn get_self_from_id(
        sql_conn: &ServerPool,
        id: u64,
    ) -> impl std::future::Future<Output = Result<Self, sqlx::Error>> + std::marker::Send {
        async move {
            sqlx::query_as::<_, Self>(&format!(
                "SELECT * FROM {} where id = ?",
                Self::table_name()
            ))
            .bind(id)
            .fetch_one(sql_conn)
            .await
        }
    }

    /// Get an item from the table from his id
    ///
    /// # Arguments
    /// * `sql_conn` - the sql database connection
    /// * `id` - the id in the table
    ///
    /// # Errors
    /// Can error if the query was not successful or the id does not exist
    fn get_item_from_id(
        sql_conn: &ServerPool,
        id: u64,
    ) -> impl std::future::Future<Output = tide::Result> + std::marker::Send {
        async move {
            let query_result: Self = Self::get_self_from_id(sql_conn, id).await?;

            let output_result = &Self::db_to_output(query_result).map_err(|e| {
                tide::Error::from_str(tide::StatusCode::InternalServerError, e.to_string())
            })?;

            Ok(tide::Response::from_res(serde_json::to_string(
                &output_result,
            )?))
        }
    }
}

pub trait DbItemSearchByName: DbItem {
    fn search_item_from_name(
        sql_conn: &ServerPool,
        name: String,
    ) -> impl std::future::Future<Output = tide::Result> + std::marker::Send {
        async move {
            let query_result: Vec<Self> = sqlx::query_as::<_, Self>(&format!(
                "SELECT * FROM {} where INSTR(name, ?)",
                Self::table_name()
            ))
            .bind(name)
            .fetch_all(sql_conn)
            .await?;

            Ok(tide::Response::from_res(serde_json::to_string(
                &query_result,
            )?))
        }
    }
}

pub trait DbItemRemove: DbItem {
    fn remove_item_by_id(
        sql_conn: &ServerPool,
        id: u64,
    ) -> impl std::future::Future<Output = tide::Result> + std::marker::Send {
        async move {
            let query_result: Vec<Self> = sqlx::query_as::<_, Self>(&format!(
                "DELETE FROM {} where id = ?",
                Self::table_name()
            ))
            .bind(id)
            .fetch_all(sql_conn)
            .await?;

            Ok(tide::Response::new(tide::StatusCode::Ok))
        }
    }
}

pub trait DbItemSearchByPrice: DbItem {
    fn search_item_from_price(
        sql_conn: &ServerPool,
        name: String,
    ) -> impl std::future::Future<Output = tide::Result> + std::marker::Send {
        async move {
            let query_result: Vec<Self> = sqlx::query_as::<_, Self>(&format!(
                "SELECT * FROM {} where INSTR(name, ?)",
                Self::table_name()
            ))
            .bind(name)
            .fetch_all(sql_conn)
            .await?;

            Ok(tide::Response::from_res(serde_json::to_string(
                &query_result,
            )?))
        }
    }
}

pub trait DbItemQuery<T>: DbItem
where
    T: QueryItem,
{
    /// Creates item from a query by adding a random uuid
    ///
    /// # Arguments
    /// * `query_item` - the query item to create from
    /// * `sql_conn` - the sql database connection
    /// # Errors
    /// Can error if the provided query_item is not the
    fn create_from_query(
        query_item: T,
        sql_conn: &ServerPool,
    ) -> impl std::future::Future<Output = Result<Self, String>> + std::marker::Send;
}
