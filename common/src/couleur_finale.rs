use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::CErr;

#[derive(Clone, Serialize, Deserialize)]
pub struct CouleurFinale {
    pub id: u64,
    pub name: String,
    pub generation_id: u64,
    couleur_nb: u32,
    pub couleur_1_id: u64,
    pub couleur_2_id: u64,
    pods_base: u64,
    pods_par_level: u32,
    energie_base: u32,
    energie_par_level: u32,
    maturite: u64,
    gestation_h: u64,
    pub coef_pourcent: u32,
}

impl CouleurFinale {
    pub fn list() -> Vec<CouleurFinale> {
        vec![
            CouleurFinale {
                id: 1,
                name: "Rousse".to_string(),
                generation_id: 1,
                couleur_nb: 1,
                couleur_1_id: 10,
                couleur_2_id: 0,
                pods_base: 100,
                pods_par_level: 5,
                energie_base: 1000,
                energie_par_level: 10,
                maturite: 1000,
                gestation_h: 48,
                coef_pourcent: 100,
            },
            CouleurFinale {
                id: 2,
                name: "Amande".to_string(),
                generation_id: 1,
                couleur_nb: 1,
                couleur_1_id: 1,
                couleur_2_id: 0,
                pods_base: 100,
                pods_par_level: 5,
                energie_base: 1000,
                energie_par_level: 10,
                maturite: 1000,
                gestation_h: 48,
                coef_pourcent: 100,
            },
            CouleurFinale {
                id: 3,
                name: "Dorée".to_string(),
                generation_id: 1,
                couleur_nb: 1,
                couleur_1_id: 2,
                couleur_2_id: 0,
                pods_base: 100,
                pods_par_level: 5,
                energie_base: 1000,
                energie_par_level: 10,
                maturite: 10000,
                gestation_h: 48,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 4,
                name: "Rousse - Amande".to_string(),
                generation_id: 2,
                couleur_nb: 2,
                couleur_1_id: 10,
                couleur_2_id: 1,
                pods_base: 150,
                pods_par_level: 5,
                energie_base: 1100,
                energie_par_level: 15,
                maturite: 2000,
                gestation_h: 60,
                coef_pourcent: 80,
            },
            CouleurFinale {
                id: 5,
                name: "Rousse - Dorée".to_string(),
                generation_id: 2,
                couleur_nb: 2,
                couleur_1_id: 10,
                couleur_2_id: 2,
                pods_base: 150,
                pods_par_level: 5,
                energie_base: 1100,
                energie_par_level: 15,
                maturite: 2000,
                gestation_h: 60,
                coef_pourcent: 80,
            },
            CouleurFinale {
                id: 6,
                name: "Amande - Dorée".to_string(),
                generation_id: 2,
                couleur_nb: 2,
                couleur_1_id: 1,
                couleur_2_id: 2,
                pods_base: 150,
                pods_par_level: 5,
                energie_base: 1100,
                energie_par_level: 15,
                maturite: 2000,
                gestation_h: 60,
                coef_pourcent: 80,
            },
            CouleurFinale {
                id: 7,
                name: "Indigo".to_string(),
                generation_id: 3,
                couleur_nb: 1,
                couleur_1_id: 5,
                couleur_2_id: 0,
                pods_base: 200,
                pods_par_level: 10,
                energie_base: 1200,
                energie_par_level: 20,
                maturite: 3000,
                gestation_h: 75,
                coef_pourcent: 80,
            },
            CouleurFinale {
                id: 8,
                name: "Ebène".to_string(),
                generation_id: 3,
                couleur_nb: 1,
                couleur_1_id: 3,
                couleur_2_id: 0,
                pods_base: 200,
                pods_par_level: 10,
                energie_base: 1200,
                energie_par_level: 20,
                maturite: 3000,
                gestation_h: 75,
                coef_pourcent: 80,
            },
            CouleurFinale {
                id: 9,
                name: "Rousse - Indigo".to_string(),
                generation_id: 4,
                couleur_nb: 2,
                couleur_1_id: 10,
                couleur_2_id: 5,
                pods_base: 250,
                pods_par_level: 10,
                energie_base: 1300,
                energie_par_level: 25,
                maturite: 4000,
                gestation_h: 84,
                coef_pourcent: 80,
            },
            CouleurFinale {
                id: 10,
                name: "Rousse - Ebène".to_string(),
                generation_id: 4,
                couleur_nb: 2,
                couleur_1_id: 10,
                couleur_2_id: 3,
                pods_base: 250,
                pods_par_level: 10,
                energie_base: 1300,
                energie_par_level: 25,
                maturite: 4000,
                gestation_h: 84,
                coef_pourcent: 80,
            },
            CouleurFinale {
                id: 11,
                name: "Amande - Indigo".to_string(),
                generation_id: 4,
                couleur_nb: 2,
                couleur_1_id: 1,
                couleur_2_id: 5,
                pods_base: 250,
                pods_par_level: 10,
                energie_base: 1300,
                energie_par_level: 25,
                maturite: 4000,
                gestation_h: 84,
                coef_pourcent: 80,
            },
            CouleurFinale {
                id: 12,
                name: "Amande - Ebène".to_string(),
                generation_id: 4,
                couleur_nb: 2,
                couleur_1_id: 1,
                couleur_2_id: 3,
                pods_base: 250,
                pods_par_level: 10,
                energie_base: 1300,
                energie_par_level: 25,
                maturite: 4000,
                gestation_h: 84,
                coef_pourcent: 80,
            },
            CouleurFinale {
                id: 13,
                name: "Dorée - Indigo".to_string(),
                generation_id: 4,
                couleur_nb: 2,
                couleur_1_id: 2,
                couleur_2_id: 5,
                pods_base: 250,
                pods_par_level: 10,
                energie_base: 1300,
                energie_par_level: 25,
                maturite: 4000,
                gestation_h: 84,
                coef_pourcent: 80,
            },
            CouleurFinale {
                id: 14,
                name: "Dorée - Ebène".to_string(),
                generation_id: 4,
                couleur_nb: 2,
                couleur_1_id: 2,
                couleur_2_id: 3,
                pods_base: 250,
                pods_par_level: 10,
                energie_base: 1300,
                energie_par_level: 25,
                maturite: 4000,
                gestation_h: 84,
                coef_pourcent: 80,
            },
            CouleurFinale {
                id: 15,
                name: "Indigo - Ebène".to_string(),
                generation_id: 4,
                couleur_nb: 2,
                couleur_1_id: 5,
                couleur_2_id: 3,
                pods_base: 250,
                pods_par_level: 10,
                energie_base: 1300,
                energie_par_level: 25,
                maturite: 4000,
                gestation_h: 84,
                coef_pourcent: 80,
            },
            CouleurFinale {
                id: 16,
                name: "Pourpre".to_string(),
                generation_id: 5,
                couleur_nb: 1,
                couleur_1_id: 8,
                couleur_2_id: 0,
                pods_base: 300,
                pods_par_level: 15,
                energie_base: 1400,
                energie_par_level: 30,
                maturite: 5000,
                gestation_h: 96,
                coef_pourcent: 60,
            },
            CouleurFinale {
                id: 17,
                name: "Orchidée".to_string(),
                generation_id: 5,
                couleur_nb: 1,
                couleur_1_id: 7,
                couleur_2_id: 0,
                pods_base: 300,
                pods_par_level: 15,
                energie_base: 1400,
                energie_par_level: 30,
                maturite: 5000,
                gestation_h: 96,
                coef_pourcent: 60,
            },
            CouleurFinale {
                id: 18,
                name: "Rousse - Pourpre".to_string(),
                generation_id: 6,
                couleur_nb: 2,
                couleur_1_id: 10,
                couleur_2_id: 8,
                pods_base: 350,
                pods_par_level: 15,
                energie_base: 1500,
                energie_par_level: 35,
                maturite: 6000,
                gestation_h: 108,
                coef_pourcent: 60,
            },
            CouleurFinale {
                id: 19,
                name: "Rousse - Orchidée".to_string(),
                generation_id: 6,
                couleur_nb: 2,
                couleur_1_id: 10,
                couleur_2_id: 7,
                pods_base: 350,
                pods_par_level: 15,
                energie_base: 1500,
                energie_par_level: 35,
                maturite: 6000,
                gestation_h: 108,
                coef_pourcent: 60,
            },
            CouleurFinale {
                id: 20,
                name: "Amande - Pourpre".to_string(),
                generation_id: 6,
                couleur_nb: 2,
                couleur_1_id: 1,
                couleur_2_id: 8,
                pods_base: 350,
                pods_par_level: 15,
                energie_base: 1500,
                energie_par_level: 35,
                maturite: 6000,
                gestation_h: 108,
                coef_pourcent: 60,
            },
            CouleurFinale {
                id: 21,
                name: "Amande - Orchidée".to_string(),
                generation_id: 6,
                couleur_nb: 2,
                couleur_1_id: 1,
                couleur_2_id: 7,
                pods_base: 350,
                pods_par_level: 15,
                energie_base: 1500,
                energie_par_level: 35,
                maturite: 6000,
                gestation_h: 108,
                coef_pourcent: 60,
            },
            CouleurFinale {
                id: 22,
                name: "Dorée - Pourpre".to_string(),
                generation_id: 6,
                couleur_nb: 2,
                couleur_1_id: 2,
                couleur_2_id: 8,
                pods_base: 350,
                pods_par_level: 15,
                energie_base: 1500,
                energie_par_level: 35,
                maturite: 6000,
                gestation_h: 108,
                coef_pourcent: 60,
            },
            CouleurFinale {
                id: 23,
                name: "Dorée - Orchidée".to_string(),
                generation_id: 6,
                couleur_nb: 2,
                couleur_1_id: 2,
                couleur_2_id: 7,
                pods_base: 350,
                pods_par_level: 15,
                energie_base: 1500,
                energie_par_level: 35,
                maturite: 6000,
                gestation_h: 108,
                coef_pourcent: 60,
            },
            CouleurFinale {
                id: 24,
                name: "Indigo - Pourpre".to_string(),
                generation_id: 6,
                couleur_nb: 2,
                couleur_1_id: 5,
                couleur_2_id: 8,
                pods_base: 350,
                pods_par_level: 15,
                energie_base: 1500,
                energie_par_level: 35,
                maturite: 6000,
                gestation_h: 108,
                coef_pourcent: 60,
            },
            CouleurFinale {
                id: 25,
                name: "Indigo - Orchidée".to_string(),
                generation_id: 6,
                couleur_nb: 2,
                couleur_1_id: 5,
                couleur_2_id: 7,
                pods_base: 350,
                pods_par_level: 15,
                energie_base: 1500,
                energie_par_level: 35,
                maturite: 6000,
                gestation_h: 108,
                coef_pourcent: 60,
            },
            CouleurFinale {
                id: 26,
                name: "Ebène - Pourpre".to_string(),
                generation_id: 6,
                couleur_nb: 2,
                couleur_1_id: 3,
                couleur_2_id: 8,
                pods_base: 350,
                pods_par_level: 15,
                energie_base: 1500,
                energie_par_level: 35,
                maturite: 6000,
                gestation_h: 108,
                coef_pourcent: 60,
            },
            CouleurFinale {
                id: 27,
                name: "Ebène - Orchidée".to_string(),
                generation_id: 6,
                couleur_nb: 2,
                couleur_1_id: 3,
                couleur_2_id: 7,
                pods_base: 350,
                pods_par_level: 15,
                energie_base: 1500,
                energie_par_level: 35,
                maturite: 6000,
                gestation_h: 108,
                coef_pourcent: 60,
            },
            CouleurFinale {
                id: 28,
                name: "Pourpre - Orchidée".to_string(),
                generation_id: 6,
                couleur_nb: 2,
                couleur_1_id: 8,
                couleur_2_id: 7,
                pods_base: 350,
                pods_par_level: 15,
                energie_base: 1500,
                energie_par_level: 35,
                maturite: 6000,
                gestation_h: 108,
                coef_pourcent: 60,
            },
            CouleurFinale {
                id: 29,
                name: "Ivoire".to_string(),
                generation_id: 7,
                couleur_nb: 1,
                couleur_1_id: 6,
                couleur_2_id: 0,
                pods_base: 400,
                pods_par_level: 20,
                energie_base: 1600,
                energie_par_level: 40,
                maturite: 7000,
                gestation_h: 120,
                coef_pourcent: 60,
            },
            CouleurFinale {
                id: 30,
                name: "Turquoise".to_string(),
                generation_id: 7,
                couleur_nb: 1,
                couleur_1_id: 11,
                couleur_2_id: 0,
                pods_base: 400,
                pods_par_level: 20,
                energie_base: 1600,
                energie_par_level: 40,
                maturite: 7000,
                gestation_h: 120,
                coef_pourcent: 60,
            },
            CouleurFinale {
                id: 31,
                name: "Rousse - Ivoire".to_string(),
                generation_id: 8,
                couleur_nb: 2,
                couleur_1_id: 10,
                couleur_2_id: 6,
                pods_base: 450,
                pods_par_level: 20,
                energie_base: 1700,
                energie_par_level: 45,
                maturite: 8000,
                gestation_h: 132,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 32,
                name: "Rousse - Turquoise".to_string(),
                generation_id: 8,
                couleur_nb: 2,
                couleur_1_id: 10,
                couleur_2_id: 11,
                pods_base: 450,
                pods_par_level: 20,
                energie_base: 1700,
                energie_par_level: 45,
                maturite: 8000,
                gestation_h: 132,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 33,
                name: "Amande - Ivoire".to_string(),
                generation_id: 8,
                couleur_nb: 2,
                couleur_1_id: 1,
                couleur_2_id: 6,
                pods_base: 450,
                pods_par_level: 20,
                energie_base: 1700,
                energie_par_level: 45,
                maturite: 8000,
                gestation_h: 132,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 34,
                name: "Amande - Turquoise".to_string(),
                generation_id: 8,
                couleur_nb: 2,
                couleur_1_id: 1,
                couleur_2_id: 11,
                pods_base: 450,
                pods_par_level: 20,
                energie_base: 1700,
                energie_par_level: 45,
                maturite: 8000,
                gestation_h: 132,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 35,
                name: "Dorée - Ivoire".to_string(),
                generation_id: 8,
                couleur_nb: 2,
                couleur_1_id: 2,
                couleur_2_id: 6,
                pods_base: 450,
                pods_par_level: 20,
                energie_base: 1700,
                energie_par_level: 45,
                maturite: 8000,
                gestation_h: 132,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 36,
                name: "Dorée - Turquoise".to_string(),
                generation_id: 8,
                couleur_nb: 2,
                couleur_1_id: 2,
                couleur_2_id: 11,
                pods_base: 450,
                pods_par_level: 20,
                energie_base: 1700,
                energie_par_level: 45,
                maturite: 8000,
                gestation_h: 132,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 37,
                name: "Indigo - Ivoire".to_string(),
                generation_id: 8,
                couleur_nb: 2,
                couleur_1_id: 5,
                couleur_2_id: 6,
                pods_base: 450,
                pods_par_level: 20,
                energie_base: 1700,
                energie_par_level: 45,
                maturite: 8000,
                gestation_h: 132,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 38,
                name: "Indigo - Turquoise".to_string(),
                generation_id: 8,
                couleur_nb: 2,
                couleur_1_id: 5,
                couleur_2_id: 11,
                pods_base: 450,
                pods_par_level: 20,
                energie_base: 1700,
                energie_par_level: 45,
                maturite: 8000,
                gestation_h: 132,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 39,
                name: "Ebène - Ivoire".to_string(),
                generation_id: 8,
                couleur_nb: 2,
                couleur_1_id: 3,
                couleur_2_id: 6,
                pods_base: 450,
                pods_par_level: 20,
                energie_base: 1700,
                energie_par_level: 45,
                maturite: 8000,
                gestation_h: 132,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 40,
                name: "Ebène - Turquoise".to_string(),
                generation_id: 8,
                couleur_nb: 2,
                couleur_1_id: 3,
                couleur_2_id: 11,
                pods_base: 450,
                pods_par_level: 20,
                energie_base: 1700,
                energie_par_level: 45,
                maturite: 8000,
                gestation_h: 132,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 41,
                name: "Pourpre - Ivoire".to_string(),
                generation_id: 8,
                couleur_nb: 2,
                couleur_1_id: 8,
                couleur_2_id: 6,
                pods_base: 450,
                pods_par_level: 20,
                energie_base: 1700,
                energie_par_level: 45,
                maturite: 8000,
                gestation_h: 132,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 42,
                name: "Pourpre - Turquoise".to_string(),
                generation_id: 8,
                couleur_nb: 2,
                couleur_1_id: 8,
                couleur_2_id: 11,
                pods_base: 450,
                pods_par_level: 20,
                energie_base: 1700,
                energie_par_level: 45,
                maturite: 8000,
                gestation_h: 132,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 43,
                name: "Orchidée - Ivoire".to_string(),
                generation_id: 8,
                couleur_nb: 2,
                couleur_1_id: 7,
                couleur_2_id: 6,
                pods_base: 450,
                pods_par_level: 20,
                energie_base: 1700,
                energie_par_level: 45,
                maturite: 8000,
                gestation_h: 132,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 44,
                name: "Orchidée - Turquoise".to_string(),
                generation_id: 8,
                couleur_nb: 2,
                couleur_1_id: 7,
                couleur_2_id: 11,
                pods_base: 450,
                pods_par_level: 20,
                energie_base: 1700,
                energie_par_level: 45,
                maturite: 8000,
                gestation_h: 132,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 45,
                name: "Ivoire - Turquoise".to_string(),
                generation_id: 8,
                couleur_nb: 2,
                couleur_1_id: 6,
                couleur_2_id: 11,
                pods_base: 450,
                pods_par_level: 20,
                energie_base: 1700,
                energie_par_level: 45,
                maturite: 8000,
                gestation_h: 132,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 46,
                name: "Emeraude".to_string(),
                generation_id: 9,
                couleur_nb: 1,
                couleur_1_id: 4,
                couleur_2_id: 0,
                pods_base: 500,
                pods_par_level: 25,
                energie_base: 1800,
                energie_par_level: 50,
                maturite: 9000,
                gestation_h: 144,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 47,
                name: "Prune".to_string(),
                generation_id: 9,
                couleur_nb: 1,
                couleur_1_id: 9,
                couleur_2_id: 0,
                pods_base: 500,
                pods_par_level: 25,
                energie_base: 1800,
                energie_par_level: 50,
                maturite: 9000,
                gestation_h: 144,
                coef_pourcent: 40,
            },
            CouleurFinale {
                id: 48,
                name: "Rousse - Emeraude".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 10,
                couleur_2_id: 4,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 49,
                name: "Rousse - Prune".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 10,
                couleur_2_id: 9,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 50,
                name: "Amande - Emeraude".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 1,
                couleur_2_id: 4,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 51,
                name: "Amande - Prune".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 1,
                couleur_2_id: 9,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 52,
                name: "Dorée - Emeraude".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 2,
                couleur_2_id: 4,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 53,
                name: "Dorée - Prune".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 2,
                couleur_2_id: 9,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 54,
                name: "Indigo - Emeraude".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 5,
                couleur_2_id: 4,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 55,
                name: "Indigo - Prune".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 5,
                couleur_2_id: 9,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 56,
                name: "Ebène - Emeraude".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 3,
                couleur_2_id: 4,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 57,
                name: "Ebène - Prune".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 3,
                couleur_2_id: 9,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 58,
                name: "Pourpre - Emeraude".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 8,
                couleur_2_id: 4,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 59,
                name: "Pourpre - Prune".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 8,
                couleur_2_id: 9,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 62,
                name: "Orchidée - Emeraude".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 7,
                couleur_2_id: 4,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 63,
                name: "Orchidée - Prune".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 7,
                couleur_2_id: 9,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 64,
                name: "Ivoire - Emeraude".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 6,
                couleur_2_id: 4,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 65,
                name: "Ivoire - Prune".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 6,
                couleur_2_id: 9,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 66,
                name: "Turquoise - Emeraude".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 11,
                couleur_2_id: 4,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 65,
                name: "Turquoise - Prune".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 11,
                couleur_2_id: 9,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
            CouleurFinale {
                id: 66,
                name: "Emeraude - Prune".to_string(),
                generation_id: 10,
                couleur_nb: 2,
                couleur_1_id: 4,
                couleur_2_id: 9,
                pods_base: 550,
                pods_par_level: 25,
                energie_base: 1900,
                energie_par_level: 55,
                maturite: 10000,
                gestation_h: 156,
                coef_pourcent: 20,
            },
        ]
    }

    pub fn from_u64(id: u64) -> Result<Self, CErr> {
        let mut collected = Self::list().iter().cloned().filter(|c| c.id == id).collect::<VecDeque<CouleurFinale>>();
        
        let first = match collected.pop_front() {
            Some(first) => first,
            None => return Err("Undefined id".into()),
        };

        Ok(first)
    }

}

// pub fn color_str_from_u64(val: u64) -> String {
//     match val {
//         1 => 	"Rousse", 				      
//         2 => 	"Amande", 				      
//         3 => 	"Dorée", 				        
//         4 => 	"Rousse - Amande", 		  
//         5 => 	"Rousse - Dorée", 		  
//         6 => 	"Amande - Dorée", 		  
//         7 => 	"Indigo", 				      
//         8 => 	"Ebène", 				        
//         9 => 	"Rousse - Indigo", 		  
//         10 => 	"Rousse - Ebène", 		  
//         11 => 	"Amande - Indigo", 		  
//         12 => 	"Amande - Ebène", 		  
//         13 => 	"Dorée - Indigo", 		  
//         14 => 	"Dorée - Ebène", 		    
//         15 => 	"Indigo - Ebène", 		  
//         16 => 	"Pourpre", 				      
//         17 => 	"Orchidée", 			      
//         18 => 	"Rousse - Pourpre", 	  
//         19 => 	"Rousse - Orchidée", 	  
//         20 => 	"Amande - Pourpre", 	  
//         21 => 	"Amande - Orchidée", 	  
//         22 => 	"Dorée - Pourpre", 		  
//         23 => 	"Dorée - Orchidée", 	  
//         24 => 	"Indigo - Pourpre", 	  
//         25 => 	"Indigo - Orchidée", 	  
//         26 => 	"Ebène - Pourpre", 		  
//         27 => 	"Ebène - Orchidée", 	  
//         28 => 	"Pourpre - Orchidée", 	
//         29 => 	"Ivoire", 				      
//         30 => 	"Turquoise", 			      
//         31 => 	"Rousse - Ivoire", 		  
//         32 => 	"Rousse - Turquoise", 	
//         33 => 	"Amande - Ivoire", 		  
//         34 => 	"Amande - Turquoise", 	
//         35 => 	"Dorée - Ivoire", 		  
//         36 => 	"Dorée - Turquoise", 	  
//         37 => 	"Indigo - Ivoire", 		  
//         38 => 	"Indigo - Turquoise", 	
//         39 => 	"Ebène - Ivoire", 		  
//         40 => 	"Ebène - Turquoise", 	  
//         41 => 	"Pourpre - Ivoire", 	  
//         42 => 	"Pourpre - Turquoise", 	
//         43 => 	"Orchidée - Ivoire", 	  
//         44 => 	"Orchidée - Turquoise", 
//         45 => 	"Ivoire - Turquoise", 	
//         46 => 	"Emeraude", 			      
//         47 => 	"Prune", 				        
//         48 => 	"Rousse - Emeraude", 	  
//         49 => 	"Rousse - Prune", 		  
//         50 => 	"Amande - Emeraude", 	  
//         51 => 	"Amande - Prune", 		  
//         52 => 	"Dorée - Emeraude", 	  
//         53 => 	"Dorée - Prune", 		    
//         54 => 	"Indigo - Emeraude", 	  
//         55 => 	"Indigo - Prune", 		  
//         56 => 	"Ebène - Emeraude", 	  
//         57 => 	"Ebène - Prune", 		    
//         58 => 	"Pourpre - Emeraude", 	
//         59 => 	"Pourpre - Prune", 		  
//         60 => 	"Orchidée - Emeraude", 	
//         61 => 	"Orchidée - Prune", 	  
//         62 => 	"Ivoire - Emeraude", 	  
//         63 => 	"Ivoire - Prune", 		  
//         64 => 	"Turquoise - Emeraude", 
//         65 => 	"Turquoise - Prune", 	  
//         66 => 	"Emeraude - Prune",
//         _ => "Unknown"	  
//     }.to_string()
// }