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
        db_items::dragodinde::{Dragodinde, DragodindeReturn},
        main,
        query_items::QueryItem,
    };
    use common::{
        couleur::CouleurBase,
        couleur_finale::CouleurFinale,
        query::{DragodindeQuery, PostReturn},
    };
    use reqwest::blocking::Client;
    use std::{collections::HashMap, thread, time::Duration};

    const SERVER_URL: &str = "http://localhost:8520";

    fn get_all_colors() -> Vec<CouleurFinale> {
        CouleurFinale::list()
    }

    fn find_final_color(
        c1: CouleurBase,
        c2: Option<CouleurBase>,
    ) -> Option<CouleurFinale> {
        let resulting_colors = get_all_colors()
            .iter()
            .cloned()
            .filter(|c| match c2 {
                Some(c2_asked) => {
                    (c.couleur_1_id == c1.to_id() && c.couleur_2_id == c2_asked.to_id())
                        || (c.couleur_1_id == c2_asked.to_id() && c.couleur_2_id == c1.to_id())
                }
                None => c.couleur_1_id == c1.to_id() && c.couleur_2_id == 0,
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
        let final_color_id = find_final_color(c1, c2).unwrap().id;

        client
            .post(format!("{SERVER_URL}/{}", Dragodinde::query_name()))
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

    fn create_dd_couple(client: &Client) -> (u64, u64) {
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
            (CouleurBase::POURPRE, Some(CouleurBase::IVOIRE)),
            (None, None),
        );

        let dd_papa = create_one_dd(client, "papa", 0, (CouleurBase::ROUSSE, None), (None, None));

        let dd_moi = create_one_dd(
            client,
            "moi",
            0,
            (CouleurBase::ORCHIDEE, Some(CouleurBase::TURQUOISE)),
            (Some(dd_papa), Some(dd_maman)),
        );

        let dd_papa_elle = create_one_dd(
            client,
            "papa_elle",
            0,
            (CouleurBase::ORCHIDEE, Some(CouleurBase::TURQUOISE)),
            (None, None),
        );

        let dd_maman_elle = create_one_dd(
            client,
            "maman_elle",
            1,
            (CouleurBase::IVOIRE, Some(CouleurBase::TURQUOISE)),
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
            .unwrap()
    }

    fn pgcs_to_str(pgc: HashMap<u64, f32>) -> HashMap<String, f32> {
        pgc.iter()
            .map(|p| (CouleurFinale::from_u64(*p.0).unwrap().name, *p.1))
            .collect()
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
        // let (pgc_m, pgc_f) = (get_pgc_for_dd(&client, m_id), get_pgc_for_dd(&client, f_id));
        // let (pgc_m_str, pgc_f_str) = (pgcs_to_str(pgc_m), pgcs_to_str(pgc_f));

        // println!("moi: {:?}", pgc_m_str);
        // println!("elle: {:?}", pgc_f_str);

        let probas_per_color = client
            .get(format!("{SERVER_URL}/combined/{}/{}", m_id, f_id))
            .send()
            .unwrap()
            .json::<HashMap<u64, f32>>()
            .unwrap()
            .iter()
            .map(|h| (CouleurFinale::from_u64(*h.0).unwrap().name, *h.1))
            .collect::<HashMap<String, f32>>();

        println!("{:?}", probas_per_color)
    }
}
