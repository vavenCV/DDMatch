use common::query::DragodindeQuery;
use common::CErr;
use dotenv::dotenv;
use routes::route_traits::{DelQueryFromId, GetQuery, GetQueryFromName, GetQueryList, PostQuery};
use sqlx::MySql;
use sqlx::Pool;
use tide::Request;

mod combinaisons;
mod db_items;
mod query_items;
mod routes;
mod test;
mod utils;

#[derive(Clone, Debug)]
pub struct ServerState {
    pub sql_conn: Pool<MySql>,
}

type ServerPool = Pool<MySql>;
type ServerRequest = Request<ServerState>;

const IP_ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8520;

async fn create_server() -> Result<tide::Server<ServerState>, CErr> {
    dotenv().ok();

    let db_user = std::env::var("DB_USER").unwrap();
    let db_name = std::env::var("DB_NAME").unwrap();
    let db_passwd = std::env::var("DB_PASSWD").unwrap();
    let pool = sqlx::MySqlPool::connect(&format!(
        "mysql://{db_user}:{db_passwd}@127.0.0.1/{db_name}"
    ))
    .await?;
    // sqlx::migrate!().run(&pool).await?;
    let state = ServerState { sql_conn: pool };

    Ok(tide::with_state(state))
}

fn set_server_routes(server: &mut tide::Server<ServerState>) {
    server.at("/").get(|_| async { Ok("Hello, world!") });

    // Add 'GET' method to routes to return list
    db_items::dragodinde::Dragodinde::add_get_list_route(server);
    // db_items::couleur_finale::CouleurFinale::add_get_list_route(server);

    db_items::dragodinde::Dragodinde::add_get_route(server);
    // db_items::couleur_finale::CouleurFinale::add_get_route(server);

    db_items::dragodinde::Dragodinde::add_get_route_from_name(server);
    // db_items::couleur_finale::CouleurFinale::add_get_route_from_name(server);

    // add 'POST' method to create item
    db_items::dragodinde::Dragodinde::add_post_route::<DragodindeQuery>(server);
    db_items::dragodinde::Dragodinde::add_del_route_from_id(server);

    server
        .at("/combined/:id_p/:id_m")
        .get(|req: ServerRequest| async move {
            let id_p: u64 = req.param("id_p")?.parse()?;
            let id_m: u64 = req.param("id_m")?.parse()?;
            println!("id_pere: {id_p}, id_mere: {id_m}");
            let res = combinaisons::calculate_combined_prob(&req.state().sql_conn, id_p, id_m)
                .await
                .map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::InternalServerError, e.to_string())
                })?;
            Ok(tide::Response::from(serde_json::to_string(&res)?))
            // Ok::<_, tide::Error>("pgc")

            // let color = <couleur_finale::CouleurFinale as db_items::DbItem>::get_self_from_id(&req.state().sql_conn, id).await?;
            // Ok(combinaisons::calculate_PGC(color).to_string())
        });
}

#[async_std::main] // Requires the `attributes` feature of `async-std`
async fn main() {
    tide::log::start();
    let mut app = create_server().await.unwrap();

    set_server_routes(&mut app);

    app.listen(format!("{IP_ADDRESS}:{PORT}")).await.unwrap();
}
