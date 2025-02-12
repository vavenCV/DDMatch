// use testcontainers::{runners::AsyncRunner, GenericImage, ImageExt, core::WaitFor};

// #[sqlx::test]
// async fn basic_test(pool: MySqlPool) -> sqlx::Result<()> {

//     // let container = GenericImage::new("mariadb", "latest")
//     //     .with_exposed_port(testcontainers::core::ContainerPort::Tcp(3306))
//     //     .with_env_var("MARIADB_ROOT_PASSWORD", "example")
//     //     .start().await.expect("Could not start mariadb container");

//     // let pool = sqlx::MySqlPool::connect("mysql://leo74270:my_tool@127.0.0.1/mydb").await?;

//     let mut conn = pool.acquire().await?;

//     // let y = sqlx::query_as("SELECT * FROM dragodinde").fetch_one(&mut conn).await?;

//     let foo = sqlx::query("SELECT * FROM dragodinde")
//         .fetch_one(&mut *conn)
//         .await?;

//     // assert_eq!(foo.get::<String, _>("bar"), "foobar!");

//     Ok(())
// }

#[cfg(test)]
mod main_tests {
    use crate::{
        db_items::{
            couleur::CouleurBase,
            couleur_finale::{color_str_from_u64, CouleurFinale},
            dragodinde::{Dragodinde, DragodindeReturn},
            DbItem,
        },
        main,
    };
    use common::query::{DragodindeQuery, PostReturn};
    use reqwest::blocking::Client;
    use std::{collections::HashMap, io::Read, thread, time::Duration};

    const SERVER_URL: &str = "http://localhost:8520";

    fn get_all_colors(client: &Client) -> Vec<CouleurFinale> {
        client
            .get(format!("{SERVER_URL}/{}", CouleurFinale::query_name()))
            .send()
            .unwrap()
            .json::<Vec<CouleurFinale>>()
            .unwrap()
    }

    fn find_final_color(
        client: &Client,
        c1: CouleurBase,
        c2: Option<CouleurBase>,
    ) -> Option<CouleurFinale> {

        let resulting_colors = get_all_colors(client)
            .iter()
            .cloned()
            .filter(|c| {
                match c2 {
                    Some(c2_asked) => {
                        (c.couleur_1_id == c1.to_id() && c.couleur_2_id == Some(c2_asked.to_id()))
                            || (c.couleur_1_id == c2_asked.to_id()
                                && c.couleur_2_id == Some(c1.to_id()))
                    }
                    None => c.couleur_1_id == c1.to_id() && c.couleur_2_id == Some(0),
                }
            })
            .collect::<Vec<CouleurFinale>>();

        resulting_colors.get(0).cloned()
    }

    fn delete_all_dd(client: &Client) {
        let all_dd = client
            .get(format!("{SERVER_URL}/{}", Dragodinde::query_name()))
            .send()
            .unwrap()
            .json::<Vec<DragodindeReturn>>()
            .unwrap();

        for dd in all_dd {
            client
                .delete(format!(
                    "{SERVER_URL}/{}/{}",
                    Dragodinde::query_name(),
                    dd.id
                ))
                .send()
                .unwrap();
        }
    }

    fn create_one_dd(
        client: &Client,
        name: &str,
        genre: u8,
        (c1, c2): (CouleurBase, Option<CouleurBase>),
        (pere, mere): (Option<u64>, Option<u64>),
    ) -> u64 {
        let final_color_id = find_final_color(client, c1, c2).unwrap().id;

        client
            .post(format!("{SERVER_URL}/dragodinde"))
            .json(&DragodindeQuery {
                name: name.to_string(),
                genre: genre as u64,
                couleur_finale_id: final_color_id,
                parent_pere_id: pere,
                parent_mere_id: mere,
                ..Default::default()
            })
            .send()
            .unwrap()
            .json::<PostReturn>()
            .unwrap()
            .id
    }

    fn create_dd_couple(client: &Client) -> (u64, u64){
        let dd_grand_papa = create_one_dd(
            client,
            "grand_papa",
            0,
            (CouleurBase::AMANDE, Some(CouleurBase::DOREE)),
            (None, None),
        );

        let dd_grande_maman = create_one_dd(
            client,
            "grande_maman",
            1,
            (CouleurBase::AMANDE, Some(CouleurBase::ROUSSE)),
            (None, None),
        );

        let dd_maman = create_one_dd(
            client,
            "maman",
            1,
            (CouleurBase::AMANDE, Some(CouleurBase::ROUSSE)),
            (Some(dd_grand_papa), Some(dd_grande_maman)),
        );

        let dd_papa = create_one_dd(client, "papa", 0, (CouleurBase::INDIGO, None), (None, None));

        let dd_moi = create_one_dd(
            client,
            "moi",
            0,
            (CouleurBase::AMANDE, Some(CouleurBase::INDIGO)),
            (Some(dd_papa), Some(dd_maman)),
        );

        let dd_papa_elle = create_one_dd(
            client,
            "papa_elle",
            0,
            (CouleurBase::AMANDE, None),
            (None, None),
        );

        let dd_maman_elle = create_one_dd(
            client,
            "maman_elle",
            1,
            (CouleurBase::IVOIRE, None),
            (None, None),
        );

        let dd_elle = create_one_dd(
            client,
            "elle",
            1,
            (CouleurBase::AMANDE, Some(CouleurBase::IVOIRE)),
            (Some(dd_papa_elle), Some(dd_maman_elle)),
        );
        (dd_moi, dd_elle)
    }

    fn get_pgc_for_dd(client: &Client, id: u64) -> HashMap<u64, f32> {
        client
            .get(format!("{SERVER_URL}/pgc/{}", id))
            .send()
            .unwrap()
            .json::<HashMap<u64, f32>>()
            .unwrap().iter().map(|p| (color_str_from_u64(*p.0), *p.1)).collect()
    }

    fn pgcs_to_str(pgc: HashMap<u64, f32>) -> HashMap<String, f32> {
        pgc.iter().map(|p| (color_str_from_u64(*p.0), *p.1)).collect()
    }

    fn get_all_dragodindes(client: &Client) -> Vec<DragodindeReturn> {
        client
            .get(format!("{SERVER_URL}/{}", Dragodinde::query_name()))
            .send()
            .unwrap()
            .json::<Vec<DragodindeReturn>>()
            .unwrap()
    }

    fn setup_test() -> Client {
        let _main_thread = thread::spawn(|| main());

        thread::sleep(Duration::from_secs(1));

        let client = Client::new();

        delete_all_dd(&client);

        client
    }

    #[test]
    fn test_1() {
        let client = setup_test();

        let (m_id, f_id) = create_dd_couple(&client);
        let (pgc_m, pgc_f) = (get_pgc_for_dd(&client, m_id), get_pgc_for_dd(&client, f_id));
        let (pgc_m_str, pgc_f_str) = (pgcs_to_str(pgc_m), pgcs_to_str(pgc_f));

        
    }
}
