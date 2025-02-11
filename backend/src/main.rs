use routes::menu_item_routes::{DelQueryFromId, GetQuery, GetQueryFromName, GetQueryList, PostQuery};
use routes::query_items::DragodindeQuery;
use sqlx::MySql;
use sqlx::Pool;
use dotenv::dotenv;
use tide::Request;

mod db_items;
mod routes;
mod utils;
mod combinaisons;
mod test;

#[derive(Clone, Debug)]
pub struct ServerState {
    pub sql_conn: Pool<MySql>,
}

type ServerPool = Pool<MySql>;
type CErr = Box<dyn std::error::Error>;
type ServerRequest = Request<ServerState>;

const IP_ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8520;

async fn create_server() -> Result<tide::Server<ServerState>, CErr> {
    dotenv().ok();

    let db_user = std::env::var("DB_USER").unwrap();
    let db_name = std::env::var("DB_NAME").unwrap();
    let db_passwd = std::env::var("DB_PASSWD").unwrap();
    let pool = sqlx::MySqlPool::connect(&format!("mysql://{db_user}:{db_passwd}@127.0.0.1/{db_name}")).await?;
    // sqlx::migrate!().run(&pool).await?;
    let state = ServerState { sql_conn: pool };

    Ok(tide::with_state(state))
}

fn set_server_routes(server: &mut tide::Server<ServerState>) {
    server.at("/").get(|_| async { Ok("Hello, world!") });

    // Add 'GET' method to routes to return list
    db_items::dragodinde::Dragodinde::add_get_list_route(server);
    db_items::couleur_finale::CouleurFinale::add_get_list_route(server);

    db_items::dragodinde::Dragodinde::add_get_route(server);
    db_items::couleur_finale::CouleurFinale::add_get_route(server);

    db_items::dragodinde::Dragodinde::add_get_route_from_name(server);
    db_items::couleur_finale::CouleurFinale::add_get_route_from_name(server);

    // add 'POST' method to create item
    db_items::dragodinde::Dragodinde::add_post_route::<DragodindeQuery>(server);
    db_items::dragodinde::Dragodinde::add_del_route_from_id(server);

    server.at("/pgc/:id").get(|req: ServerRequest| async move {
        let id: u64 = req.param("id")?.parse()?;
        let _ = combinaisons::calculate_probabilities_by_color(&req.state().sql_conn, id).await;
        Ok::<_, tide::Error>("pgc")

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
