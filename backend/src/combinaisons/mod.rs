use crate::{
    db_items::dragodinde::Dragodinde,
    ServerPool,
};
use combinaisons::color_mix;
use common::couleur_finale::CouleurFinale;
use common::CErr;
use custom_error::custom_error;
use std::collections::VecDeque;
use std::{collections::HashMap, future::Future, pin::Pin};

pub mod combinaisons;

custom_error! {MyError
    // Io{source: io::Error}             = "input/output error",
    // WidgetNotFoundError{name: String} = "could not find widget '{name}'",
    NoParent                        = "This dragodinde has no parents",
    EndOfTree                       = "End of tree"
}

async fn get_dd_from_id(sql_conn: &ServerPool, id: u64) -> Result<Dragodinde, CErr> {
    Ok(<Dragodinde as crate::db_items::DbItem>::get_self_from_id(&sql_conn, id).await?)
}

fn calculate_pgc(couleur_id: u64) -> Result<f32, CErr> {
    let color = CouleurFinale::from_u64(couleur_id)?;

    Ok(color.coef_pourcent as f32 / ((2 - color.generation_id % 2) as f32))
}

pub const WEIGHT_BY_PARENT_GEN: [u8; 4] = [10, 6, 3, 1];
pub const WEIGHT_BY_PARENT_GEN_PREDISPOSED: [u8; 4] = [20, 6, 3, 1];

fn combine_maps(map1: &mut HashMap<u64, u8>, map2: &HashMap<u64, u8>) {
    for (&key, &value) in map2.iter() {
        *map1.entry(key).or_insert(0) += value;
    }
}

pub fn proba_couleurs<'a>(
    sql_conn: &'a ServerPool,
    dd_id: u64,
    wpg: &'a mut VecDeque<u8>,
) -> Pin<Box<dyn Future<Output = Result<(u8, HashMap<u64, u8>), CErr>> + Send + 'a>> {
    Box::pin(async move {
        let current_weight = match wpg.pop_front() {
            Some(weight) => weight,
            None => return Err("End of tree".into()),
        };

        let dd = get_dd_from_id(sql_conn, dd_id).await?;
        let mut weight_by_color: HashMap<u64, u8> = HashMap::new();

        let color = dd.couleur_finale_id;
        *weight_by_color.entry(color).or_insert(0) += current_weight;

        let pere_id = match dd.parent_pere_id {
            Some(id) => id,
            None => return Ok((current_weight, weight_by_color)),
        };
        let mere_id = match dd.parent_mere_id {
            Some(id) => id,
            None => return Ok((current_weight, weight_by_color)),
        };

        let (total_weight_pere, weight_by_color_pere) =
            proba_couleurs(sql_conn, pere_id, &mut wpg.clone())
                .await
                .unwrap_or_else(|_| (0, HashMap::new()));
        let (total_weight_mere, weight_by_color_mere) =
            proba_couleurs(sql_conn, mere_id, &mut wpg.clone())
                .await
                .unwrap_or_else(|_| (0, HashMap::new()));

        combine_maps(&mut weight_by_color, &weight_by_color_pere);
        combine_maps(&mut weight_by_color, &weight_by_color_mere);

        let total_weight = total_weight_pere + total_weight_mere + current_weight;

        Ok((total_weight, weight_by_color))
    })
}

pub async fn calculate_probabilities_by_color(
    sql_conn: &ServerPool,
    dd_id: u64,
) -> HashMap<u64, f32> {
    let mut wpg: VecDeque<u8> = WEIGHT_BY_PARENT_GEN.iter().cloned().collect();

    let (total_weight, weight_by_color) = proba_couleurs(sql_conn, dd_id, &mut wpg)
        .await
        .unwrap_or_else(|_| (0, HashMap::new()));

    let mut res: HashMap<u64, f32> = HashMap::new();

    for (color, weight) in weight_by_color.iter() {
        res.insert(*color, *weight as f32 / total_weight as f32);
    }

    res
}

pub async fn calculate_combined_prob(
    sql_conn: &ServerPool,
    dad_id: u64,
    mom_id: u64,
) -> Result<HashMap<u64, f32>, CErr> {
    let mut proba_by_color: HashMap<u64, f32> = HashMap::new();

    let dad = get_dd_from_id(sql_conn, dad_id).await?;
    let mom = get_dd_from_id(sql_conn, mom_id).await?;

    for dad_color in serde_json::from_str::<HashMap<u64, f32>>(&dad.proba_couleur)? {
        for mom_color in serde_json::from_str::<HashMap<u64, f32>>(&mom.proba_couleur)? {
            let combined_pgc = match color_mix(&(dad_color.0 - 1), &(mom_color.0 - 1)) {
                Some(color_mix) => (color_mix + 1, calculate_pgc(color_mix + 1)? / 2.0),
                None => (0, 0.0),
            };
            let dad_color_pgc = calculate_pgc(dad_color.0)?;
            let mom_color_pgc = calculate_pgc(mom_color.0)?;

            let coef_mix = dad_color.1 * mom_color.1;

            if combined_pgc.0 != 0 {
                let proba_combined = (
                    combined_pgc.0,
                    coef_mix * combined_pgc.1 / (dad_color_pgc + mom_color_pgc + combined_pgc.1),
                );
                *proba_by_color.entry(proba_combined.0).or_insert(0.0) += proba_combined.1;
            }
            
            let proba_dad = (
                dad_color.0,
                coef_mix * dad_color_pgc / (dad_color_pgc + mom_color_pgc + combined_pgc.1),
            );
            let proba_mom = (
                mom_color.0,
                coef_mix * mom_color_pgc / (dad_color_pgc + mom_color_pgc + combined_pgc.1),
            );

            *proba_by_color.entry(proba_dad.0).or_insert(0.0) += proba_dad.1;
            *proba_by_color.entry(proba_mom.0).or_insert(0.0) += proba_mom.1;
        }
    }

    println!("{:?}", proba_by_color);
    Ok(proba_by_color)
}
