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
    use reqwest::blocking::Client;
    use std::{thread, time::Duration};

    use crate::{
        db_items::{
            couleur::CouleurBase, couleur_finale::CouleurFinale, dragodinde::DragodindeReturn,
            DbItem,
        },
        main,
        routes::query_items::DragodindeQuery,
    };

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
        c2: CouleurBase,
    ) -> Option<CouleurFinale> {
        let resulting_colors = get_all_colors(client)
            .iter()
            .cloned()
            .filter(|c| match c.couleur_2_id {
                Some(cid2) => {
                    ((c.couleur_1_id == c1.to_id()) && (cid2 == c2.to_id()))
                        || ((c.couleur_1_id == c2.to_id()) && (cid2 == c1.to_id()))
                }
                None => false,
            })
            .collect::<Vec<CouleurFinale>>();

        resulting_colors.get(0).cloned()
    }

    fn create_dd(client: &Client) {
        let dd_grand_papa = client
            .post(format!("{SERVER_URL}/dragodinde"))
            .json(&DragodindeQuery {
                name: "grand_papa".to_string(),
                description: "aucune".to_string(),
                genre: 0,
                couleur_finale_id: find_final_color(
                    client,
                    CouleurBase::AMANDE,
                    CouleurBase::DOREE,
                )
                .unwrap()
                .id,
                parent_pere_id: None,
                parent_mere_id: None,
                gestation_nb: None,
                capacity_ids: vec![],
            })
            .send()
            .unwrap()
            .json::<DragodindeReturn>()
            .unwrap();

        let dd_grande_maman = client
            .post(format!("{SERVER_URL}/dragodinde"))
            .json(&DragodindeQuery {
                name: "grande_maman".to_string(),
                description: "aucune".to_string(),
                genre: 0,
                couleur_finale_id: find_final_color(
                    client,
                    CouleurBase::AMANDE,
                    CouleurBase::ROUSSE,
                )
                .unwrap()
                .id,
                parent_pere_id: None,
                parent_mere_id: None,
                gestation_nb: None,
                capacity_ids: vec![],
            })
            .send()
            .unwrap()
            .json::<DragodindeReturn>()
            .unwrap();

        let dd_maman = client
            .post(format!("{SERVER_URL}/dragodinde"))
            .json(&DragodindeQuery {
                name: "maman".to_string(),
                description: "aucune".to_string(),
                genre: 0,
                couleur_finale_id: find_final_color(
                    client,
                    CouleurBase::AMANDE,
                    CouleurBase::ROUSSE,
                )
                .unwrap()
                .id,
                parent_pere_id: Some(dd_grand_papa.id),
                parent_mere_id: Some(dd_grande_maman.id),
                gestation_nb: None,
                capacity_ids: vec![],
            })
            .send()
            .unwrap()
            .json::<DragodindeReturn>()
            .unwrap();

        let dd_papa = client
            .post(format!("{SERVER_URL}/dragodinde"))
            .json(&DragodindeQuery {
                name: "papa".to_string(),
                description: "aucune".to_string(),
                genre: 0,
                couleur_finale_id: CouleurBase::DOREE.to_id(),
                parent_pere_id: None,
                parent_mere_id: None,
                gestation_nb: None,
                capacity_ids: vec![],
            })
            .send()
            .unwrap()
            .json::<DragodindeReturn>()
            .unwrap();

        let dd_moi = client
            .post(format!("{SERVER_URL}/dragodinde"))
            .json(&DragodindeQuery {
                name: "moi".to_string(),
                description: "aucune".to_string(),
                genre: 0,
                couleur_finale_id: find_final_color(
                    client,
                    CouleurBase::AMANDE,
                    CouleurBase::DOREE,
                )
                .unwrap()
                .id,
                parent_pere_id: Some(dd_papa.id),
                parent_mere_id: Some(dd_maman.id),
                gestation_nb: None,
                capacity_ids: vec![],
            })
            .send()
            .unwrap()
            .json::<DragodindeReturn>()
            .unwrap();
    }

    #[test]
    fn test_1() {
        let _main_thread = thread::spawn(|| main());

        thread::sleep(Duration::from_secs(1));

        let client = Client::new();

        create_dd(&client);
    }
}
