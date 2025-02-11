use tide::Server;

use crate::{
    db_items::{DbItem, DbItemQuery, DbItemRemove, DbItemSearchByName},
    ServerRequest, ServerState,
};

use super::QueryItem;

pub trait PostQuery {
    fn add_post_route<V>(server: &mut Server<ServerState>)
    where
        Self: DbItemQuery<V>,
        V: QueryItem,
    {
        server
            .at(&Self::query_name())
            .post(|mut req: ServerRequest| async move {
                let query_item = V::create_from_body(&mut req).await?;
                let sql_conn = &req.state().sql_conn;

                let db_item = Self::create_from_query(query_item, sql_conn)
                    .await
                    .map_err(|e| tide::Error::from_str(500, e.to_string()))?;

                let _ = db_item
                    .insert_into_db(sql_conn)
                    .await
                    .map_err(|e| tide::Error::from_str(500, e.to_string()))?;

                // let result_item = Self::get_item_from_id(sql_conn, db_item.get_id())
                //     .await
                //     .map_err(|e| tide::Error::from_str(500, e.to_string()))?;

                Ok(tide::Response::from(serde_json::to_string(
                    &common::query::PostReturn {
                        id: db_item.get_id(),
                    },
                )?))
            });
    }
}

pub trait GetQueryList {
    fn add_get_list_route(server: &mut Server<ServerState>)
    where
        Self: DbItem,
    {
        server.at(&Self::query_name()).get(|req: ServerRequest| async move {
            Self::get_all_items(&req.state().sql_conn).await
        });
    }
}

pub trait GetQuery {
    fn add_get_route(server: &mut Server<ServerState>)
    where
        Self: DbItem,
    {
        server
            .at(&format!("{}/:id", &Self::query_name()))
            .get(|req: ServerRequest| async move {
                let id: u64 = req.param("id")?.parse()?;
                Self::get_item_from_id(&req.state().sql_conn, id).await
            });
    }
}

pub trait GetQueryFromName {
    fn add_get_route_from_name(server: &mut Server<ServerState>)
    where
        Self: DbItemSearchByName,
    {
        server
            .at(&format!("{}/search_by_name/:name", &Self::query_name()))
            .get(|req: ServerRequest| async move {
                let name: String = req.param("name")?.to_owned();
                Self::search_item_from_name(&req.state().sql_conn, name).await
            });
    }
}

pub trait DelQueryFromId {
    fn add_del_route_from_id(server: &mut Server<ServerState>)
    where
        Self: DbItemRemove,
    {
        server.at(&format!("{}/:id", &Self::query_name())).delete(
            |req: ServerRequest| async move {
                let id: u64 = req.param("id")?.parse()?;
                Self::remove_item_by_id(&req.state().sql_conn, id).await
            },
        );
    }
}
