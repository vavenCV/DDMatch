use std::{collections::HashMap, future::Future, pin::Pin};
use custom_error::custom_error;
use crate::{db_items::{couleur_finale::CouleurFinale, dragodinde::Dragodinde}, CErr, ServerPool};
use std::collections::VecDeque;

pub mod combinaisons;

custom_error!{MyError
    // Io{source: io::Error}             = "input/output error",
    // WidgetNotFoundError{name: String} = "could not find widget '{name}'",
    NoParent                        = "This dragodinde has no parents",
    EndOfTree                       = "End of tree"
}


async fn get_dd_from_id(sql_conn: &ServerPool, id: u64) -> Result<Dragodinde, CErr> {
    Ok(<Dragodinde as crate::db_items::DbItem>::get_self_from_id(&sql_conn, id).await?)
}

pub fn calculate_PGC(couleur_finale: CouleurFinale) -> f32 {
    couleur_finale.coef_pourcent as f32 / ((2 - couleur_finale.generation_id % 2) as f32)
}

async fn calculate_pgc(sql_conn: &ServerPool, dragodinde: &Dragodinde) -> Result<f32, CErr> {
    Ok(calculate_PGC(<CouleurFinale as crate::db_items::DbItem>::get_self_from_id(&sql_conn, dragodinde.couleur_finale_id).await?))
}

fn calculate_pgc_for_parents<'a>(sql_conn: &'a ServerPool, dragodinde: &'a Dragodinde, weight: &'a mut Vec<u8>) -> Pin<Box<dyn Future<Output = Result<Vec<Option<(f32, f32)>>, CErr>> + Send + 'a>> {

    Box::pin(async move {

        let dragodinde_pere_id = match dragodinde.parent_pere_id {
            Some(id) => id,
            None => return Ok(vec![None]),
        };
        let dragodinde_mere_id = match dragodinde.parent_mere_id {
            Some(id) => id,
            None => return Ok(vec![None]),
        };

        if weight.len() == 0 {
            return Ok(vec![None]);
        } 
        
        let _weight_self = match weight.pop() {
            Some(w) => w,
            None => return Ok(vec![None]),
        };
        
        let pere_dd = &get_dd_from_id(sql_conn, dragodinde_pere_id).await?;
        let mere_dd = &get_dd_from_id(sql_conn, dragodinde_mere_id).await?;
    
        let pere_pgc = calculate_pgc(sql_conn, pere_dd).await?;
        let mere_pgc = calculate_pgc(sql_conn, mere_dd).await?;
    
        let pere_recurs = calculate_pgc_for_parents(sql_conn, pere_dd, weight).await?;
        let mere_recurs = calculate_pgc_for_parents(sql_conn, mere_dd, weight).await?;

        Ok(vec![Some((pere_pgc, mere_pgc))]
            .into_iter()
            .chain(pere_recurs.into_iter())
            .chain(mere_recurs.into_iter())
            .collect())
    })
}


// pub async fn proba_couleur(sql_conn: &ServerPool, id: u64) -> impl std::future::Future<Output = Result<f32, CErr>> + std::marker::Send + use<'_> {
//     async move {
//         let dragodinde = &get_dd_from_id(sql_conn, id).await?;

//         let dd_pgc = calculate_pgc(sql_conn, dragodinde).await?;
//         let parent_pgcs = calculate_pgc_for_parents(sql_conn, dragodinde, &mut vec![10,6,3,1]).await?;
//         println!("{:?}", dd_pgc);
//         println!("{:?}", parent_pgcs);
//         Ok(0.0)
//     })
// }

pub const weight_by_parent_gen: [u8; 4] = [10, 6, 3, 1];
pub const weight_by_parent_gen_predisposed: [u8; 4] = [20, 6, 3, 1];

fn combine_maps(map1: &mut HashMap<u64, u8>, map2: &HashMap<u64, u8>) {
    for (&key, &value) in map2.iter() {
        *map1.entry(key).or_insert(0) += value;
    }
}

pub fn proba_couleurs<'a>(sql_conn: &'a ServerPool, dd_id: u64, wpg: &'a mut VecDeque<u8>) -> Pin<Box<dyn Future<Output = Result<(u8, HashMap<u64, u8>), CErr>> + Send + 'a>> {
    Box::pin(async move {

        let current_weight = match wpg.pop_front() {
            Some(weight) => weight,
            None => return Err("End of tree".into()),
        };

        let dd = get_dd_from_id(sql_conn, dd_id).await?;
        let mut weight_by_color: HashMap<u64, u8> = HashMap::new();

        let color = dd.couleur_finale_id;
        *weight_by_color.entry(color).or_insert(0) += current_weight;

        // println!("dd_id: {:?}", dd_id);
        // println!("weight_by_color: {:?}", weight_by_color);

        let pere_id = match dd.parent_pere_id {
            Some(id) => id,
            None => return Ok((current_weight, weight_by_color)),
        };
        let mere_id = match dd.parent_mere_id {
            Some(id) => id,
            None => return Ok((current_weight, weight_by_color)),
        };

        let (total_weight_pere, weight_by_color_pere) = proba_couleurs(sql_conn, pere_id, &mut wpg.clone()).await.unwrap_or_else(|_| (0, HashMap::new()));
        let (total_weight_mere, weight_by_color_mere) = proba_couleurs(sql_conn, mere_id, &mut wpg.clone()).await.unwrap_or_else(|_| (0, HashMap::new()));
        
        // println!("weight_by_color_pere: {:?}", weight_by_color_pere);
        // println!("weight_by_color_mere: {:?}", weight_by_color_pere);

        combine_maps(&mut weight_by_color, &weight_by_color_pere);
        combine_maps(&mut weight_by_color, &weight_by_color_mere);

        let total_weight = total_weight_pere + total_weight_mere + current_weight;

        // println!("dd_id: {:?}, weight_by_color after: {:?}", dd_id, weight_by_color);

        Ok((total_weight, weight_by_color))
    })
}

pub async fn calculate_probabilities_by_color(sql_conn: &ServerPool, dd_id: u64) -> HashMap<u64, f32> {

    let mut wpg: VecDeque<u8> = weight_by_parent_gen.iter().cloned().collect();

    let (total_weight, weight_by_color) = proba_couleurs(sql_conn, dd_id, &mut wpg).await.unwrap_or_else(|_| (0, HashMap::new()));

    let mut res: HashMap<u64, f32> = HashMap::new();

    for (color, weight) in weight_by_color.iter() {
        res.insert(*color, *weight as f32 / total_weight as f32);
    }

    // println!("res: {:?}", res);

    res
}