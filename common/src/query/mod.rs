use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoryQuery {
    pub name: String,
    pub description: String,
    pub image_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PostReturn {
    pub id: u64
}
