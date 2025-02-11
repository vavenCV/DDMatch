use serde::{Deserialize, Serialize};

use crate::{CErr, ServerPool};

use super::DbItem;

/*
    (1, "Amande"),
    (2, "Dorée"),
    (3, "Ebène"),
    (4, "Emeraude"),
    (5, "Indigo"),
    (6, "Ivoire"),
    (7, "Orchidée"),
    (8, "Pourpre"),
    (9, "Prune"),
    (10, "Rousse"),
    (11, "Turquoise")
*/
#[repr(u64)]
#[derive(PartialEq, Copy, Clone)]
pub enum CouleurBase {
    AMANDE = 1,
    DOREE = 2,
    EBENE = 3,
    EMERAUDE = 4,
    INDIGO = 5,
    IVOIRE = 6,
    ORCHIDEE = 7,
    POURPRE = 8,
    PRUNE = 9,
    ROUSSE = 10,
    TURQUOISE = 11,
}

impl CouleurBase {
    pub fn to_id(self) -> u64 {
        unsafe { ::std::mem::transmute(self) }
    }
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Default)]
pub struct Couleur {
    pub id: u64,
    name: String,
}

impl DbItem for Couleur {
    fn table_name() -> String {
        "couleur".to_string()
    }

    fn query_name() -> String {
        "couleur_detailee".to_string()
    }

    fn get_id(self) -> u64 {
        self.id
    }

    fn db_to_output(self) -> Result<impl Serialize, CErr> {
        Ok(self)
    }

    async fn insert_into_db(&self, _sql_conn: &ServerPool) -> Result<(), CErr> {
        Err(String::from("Impossible d'ajouter de couleur").into())
    }
}

impl super::DbItemSearchByName for Couleur {}

impl crate::routes::menu_item_routes::GetQueryList for Couleur {}
impl crate::routes::menu_item_routes::GetQuery for Couleur {}
impl crate::routes::menu_item_routes::GetQueryFromName for Couleur {}