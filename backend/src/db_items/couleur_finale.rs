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

pub fn color_str_from_u64(val: u64) -> String {
    match val {
        1 => 	"Rousse", 				      
        2 => 	"Amande", 				      
        3 => 	"Dorée", 				        
        4 => 	"Rousse - Amande", 		  
        5 => 	"Rousse - Dorée", 		  
        6 => 	"Amande - Dorée", 		  
        7 => 	"Indigo", 				      
        8 => 	"Ebène", 				        
        9 => 	"Rousse - Indigo", 		  
        10 => 	"Rousse - Ebène", 		  
        11 => 	"Amande - Indigo", 		  
        12 => 	"Amande - Ebène", 		  
        13 => 	"Dorée - Indigo", 		  
        14 => 	"Dorée - Ebène", 		    
        15 => 	"Indigo - Ebène", 		  
        16 => 	"Pourpre", 				      
        17 => 	"Orchidée", 			      
        18 => 	"Rousse - Pourpre", 	  
        19 => 	"Rousse - Orchidée", 	  
        20 => 	"Amande - Pourpre", 	  
        21 => 	"Amande - Orchidée", 	  
        22 => 	"Dorée - Pourpre", 		  
        23 => 	"Dorée - Orchidée", 	  
        24 => 	"Indigo - Pourpre", 	  
        25 => 	"Indigo - Orchidée", 	  
        26 => 	"Ebène - Pourpre", 		  
        27 => 	"Ebène - Orchidée", 	  
        28 => 	"Pourpre - Orchidée", 	
        29 => 	"Ivoire", 				      
        30 => 	"Turquoise", 			      
        31 => 	"Rousse - Ivoire", 		  
        32 => 	"Rousse - Turquoise", 	
        33 => 	"Amande - Ivoire", 		  
        34 => 	"Amande - Turquoise", 	
        35 => 	"Dorée - Ivoire", 		  
        36 => 	"Dorée - Turquoise", 	  
        37 => 	"Indigo - Ivoire", 		  
        38 => 	"Indigo - Turquoise", 	
        39 => 	"Ebène - Ivoire", 		  
        40 => 	"Ebène - Turquoise", 	  
        41 => 	"Pourpre - Ivoire", 	  
        42 => 	"Pourpre - Turquoise", 	
        43 => 	"Orchidée - Ivoire", 	  
        44 => 	"Orchidée - Turquoise", 
        45 => 	"Ivoire - Turquoise", 	
        46 => 	"Emeraude", 			      
        47 => 	"Prune", 				        
        48 => 	"Rousse - Emeraude", 	  
        49 => 	"Rousse - Prune", 		  
        50 => 	"Amande - Emeraude", 	  
        51 => 	"Amande - Prune", 		  
        52 => 	"Dorée - Emeraude", 	  
        53 => 	"Dorée - Prune", 		    
        54 => 	"Indigo - Emeraude", 	  
        55 => 	"Indigo - Prune", 		  
        56 => 	"Ebène - Emeraude", 	  
        57 => 	"Ebène - Prune", 		    
        58 => 	"Pourpre - Emeraude", 	
        59 => 	"Pourpre - Prune", 		  
        60 => 	"Orchidée - Emeraude", 	
        61 => 	"Orchidée - Prune", 	  
        62 => 	"Ivoire - Emeraude", 	  
        63 => 	"Ivoire - Prune", 		  
        64 => 	"Turquoise - Emeraude", 
        65 => 	"Turquoise - Prune", 	  
        66 => 	"Emeraude - Prune",
        _ => "Unknown"	  
    }.to_string()
}