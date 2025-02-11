use serde::{Deserialize, Serialize};

use crate::{routes::query_items::DragodindeQuery, CErr, ServerPool};

use super::{DbItem, DbItemQuery};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct Dragodinde {
    id: u64,
    name: String,
    description: String,
    genre_id: u64,
    pub couleur_finale_id: u64,
    pub parent_pere_id: Option<u64>,
    pub parent_mere_id: Option<u64>,
    gestation_nb: u64,
    capacite_nb: u16,
    capacite_1_id: Option<u64>,
    capacite_2_id: Option<u64>,
    capacite_3_id: Option<u64>,
    capacite_4_id: Option<u64>,
    proba_couleur: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DragodindeReturn {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub genre: u64,
    pub couleur_finale_id: u64,
    pub parent_pere_id: Option<u64>,
    pub parent_mere_id: Option<u64>,
    pub gestation_nb: Option<u64>,
    pub capacity_ids: Vec::<u64>,
}

impl DbItem for Dragodinde {
    fn table_name() -> String {
        "dragodinde".to_string()
    }

    fn query_name() -> String {
        "dragodinde".to_string()
    }
    
    fn get_id(self) -> u64 {
        self.id
    }

    fn db_to_output(self) -> Result<impl Serialize, CErr> {
        Ok(DragodindeReturn {
            id: self.id,
            name: self.name,
            description: self.description,
            genre: self.genre_id,
            couleur_finale_id: self.couleur_finale_id,
            parent_pere_id: self.parent_pere_id,
            parent_mere_id: self.parent_mere_id,
            gestation_nb: Some(self.gestation_nb),
            capacity_ids: vec![
                self.capacite_1_id.unwrap_or_default(),
                self.capacite_2_id.unwrap_or_default(),
                self.capacite_3_id.unwrap_or_default(),
                self.capacite_4_id.unwrap_or_default(),
            ],
        })
    }

    async fn insert_into_db(&self, sql_conn: &ServerPool) -> Result<(), CErr> {
        let result = sqlx::query(
            "insert into dragodinde (
                id,
                name,
                description,
                genre_id,
                couleur_finale_id,
                parent_pere_id,
                parent_mere_id,
                gestation_nb,
                capacite_nb,
                capacite_1_id,
                capacite_2_id,
                capacite_3_id,
                capacite_4_id,
                proba_couleur)
            values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&self.id)
        .bind(&self.name)
        .bind(&self.description)
        .bind(&self.genre_id)
        .bind(&self.couleur_finale_id)
        .bind(&self.parent_pere_id)
        .bind(&self.parent_mere_id)
        .bind(&self.gestation_nb)
        .bind(&self.capacite_nb)
        .bind(&self.capacite_1_id)
        .bind(&self.capacite_2_id)
        .bind(&self.capacite_3_id)
        .bind(&self.capacite_4_id)
        .bind(&self.proba_couleur)
        .execute(sql_conn)
        .await;

        self.handle_insert_res(result)
    }
}

impl DbItemQuery<DragodindeQuery> for Dragodinde {
    async fn create_from_query(
        query: DragodindeQuery,
        sql_conn: &ServerPool,
    ) -> Result<Self, String> {
        Ok(Self {
            id: Self::get_random_unique_id(sql_conn).await,
            name: query.name,
            description: query.description,
            genre_id: query.genre,
            couleur_finale_id: query.couleur_finale_id,
            parent_pere_id: query.parent_mere_id,
            parent_mere_id: query.parent_mere_id,
            gestation_nb: query.gestation_nb.map_or(5, |f| f),
            capacite_nb: query.capacity_ids.len() as u16,
            capacite_1_id: query.capacity_ids.get(0).copied(),
            capacite_2_id: query.capacity_ids.get(1).copied(),
            capacite_3_id: query.capacity_ids.get(2).copied(),
            capacite_4_id: query.capacity_ids.get(3).copied(),
            proba_couleur: "".to_string(),
        })
    }
}


impl super::DbItemSearchByName for Dragodinde {}

impl crate::routes::menu_item_routes::PostQuery for Dragodinde {}
impl crate::routes::menu_item_routes::GetQueryList for Dragodinde {}
impl crate::routes::menu_item_routes::GetQuery for Dragodinde {}
impl crate::routes::menu_item_routes::GetQueryFromName for Dragodinde {}

impl super::DbItemRemove for Dragodinde {}
impl crate::routes::menu_item_routes::DelQueryFromId for Dragodinde {}
