use serde::{Deserialize, Serialize};

use crate::{routes::query_items::CategoryQuery, CErr, ServerPool};

use super::{DbItem, DbItemQuery};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Category {
    id: u64,
    name: String,
    description: String,
    image_url: String,
}

impl DbItem for Category {
    fn table_name() -> String {
        "category".to_string()
    }

    fn query_name() -> String {
        "category".to_string()
    }

    async fn insert_into_db(&self, sql_conn: &ServerPool) -> Result<(), CErr> {
        let result = sqlx::query(
            "insert into category (
                id, 
                name, 
                description,
                image_url) 
            values (?, ?, ?, ?)",
        )
        .bind(&self.id)
        .bind(&self.name)
        .bind(&self.description)
        .bind(&self.image_url)
        .execute(sql_conn)
        .await;

        self.handle_insert_res(result)
    }
}

impl DbItemQuery<CategoryQuery> for Category {
    async fn create_from_query(query: CategoryQuery, sql_conn: &ServerPool) -> Result<Self, String> {
        Ok(Self {
            id: Self::get_random_unique_id(sql_conn).await,
            name: query.name,
            description: query.description,
            image_url: query.image_url,
        })
    }
}

impl super::DbItemSearchByName for Category {}

impl crate::routes::menu_item_routes::PostQuery for Category {}
impl crate::routes::menu_item_routes::GetQueryList for Category {}
