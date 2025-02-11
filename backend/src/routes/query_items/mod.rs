use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::ServerRequest;

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct CategoryQuery {
    pub name: String,
    pub description: String,
    pub image_url: String,
}

#[derive(Clone, Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct DragodindeQuery {
    pub name: String,
    pub description: String,
    pub genre: u64,
    pub couleur_finale_id: u64,
    pub parent_pere_id: Option<u64>,
    pub parent_mere_id: Option<u64>,
    pub gestation_nb: Option<u64>,
    pub capacity_ids: Vec::<u64>,
}

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

impl QueryItem for DragodindeQuery {}
