use serde::de::DeserializeOwned;

use crate::ServerRequest;

pub mod menu_item_routes;

pub trait QueryItem: DeserializeOwned + Clone + Send + Sync + 'static {
    fn create_from_body(
        req: &mut ServerRequest,
    ) -> impl std::future::Future<Output = Result<Self, tide::Error>> + std::marker::Send {
        async {
            match req.body_json::<Self>().await {
                Ok(body) => Ok(body),
                Err(e) => {
                    return Err(tide::Error::from_str(
                        tide::StatusCode::PartialContent,
                        format!("Invalid menu item: {e}"),
                    ))
                }
            }
        }
    }
}

impl QueryItem for common::query::DragodindeQuery {}