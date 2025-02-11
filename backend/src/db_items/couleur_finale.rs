use serde::{Deserialize, Serialize};

use crate::{CErr, ServerPool};

use super::DbItem;

/*
  	id 			    		BIGINT UNSIGNED PRIMARY KEY NOT NULL,
	name 		    		TEXT NOT NULL,
	generation_id			BIGINT UNSIGNED NOT NULL,
	couleur_nb				INT NOT NULL,
	couleur_1_id			BIGINT UNSIGNED NOT NULL,
	couleur_2_id			BIGINT UNSIGNED,
	pods_base				BIGINT UNSIGNED NOT NULL,
	pods_par_level			INT UNSIGNED NOT NULL,
	energie_base			INT UNSIGNED NOT NULL,
	energie_par_level		INT UNSIGNED NOT NULL,
	maturite				BIGINT UNSIGNED NOT NULL,
	gestation_h				BIGINT UNSIGNED NOT NULL,
	coef_pourcent			INT UNSIGNED NOT NULL
     */

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Default, Clone)]
pub struct CouleurFinale {
    pub id: u64,
    name: String,
    pub generation_id: u64,
    pub couleur_nb: u32,
    pub couleur_1_id: u64,
    pub couleur_2_id: Option<u64>,
    pods_base: u64,
    pods_par_level: u32,
    energie_base: u32,
    energie_par_level: u32,
    maturite: u64,
    gestation_h: u64,
    pub coef_pourcent: u32,
}

impl DbItem for CouleurFinale {
    fn table_name() -> String {
        "couleur_finale".to_string()
    }

    fn query_name() -> String {
        "couleur".to_string()
    }

    fn get_id(self) -> u64 {
        self.id
    }

    fn db_to_output(self) -> Result<impl Serialize, CErr> {
        Ok(self)
    }

    async fn insert_into_db(&self, _sql_conn: &ServerPool) -> Result<(), CErr> {
        Err(String::from("Impossible d'ajouter des couleurs").into())
    }
}

impl super::DbItemSearchByName for CouleurFinale {}

impl crate::routes::menu_item_routes::GetQueryList for CouleurFinale {}
impl crate::routes::menu_item_routes::GetQuery for CouleurFinale {}
impl crate::routes::menu_item_routes::GetQueryFromName for CouleurFinale {}