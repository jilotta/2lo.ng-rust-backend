use actix_web::{App, HttpServer};

use tokio::spawn; // for spawning the database client off
use tokio_postgres::{Client, NoTls}; // the database itself

mod add;
mod redirect;
mod stats;

use std::env;

use tokio::sync::Mutex;
struct SharedState {
    client: Mutex<Client>,
    strid_length: Mutex<usize>,
}
type Data = actix_web::web::Data<SharedState>;

#[macro_export]
macro_rules! http_error {
    ($x:ident) => {
        HttpResponse::new(actix_web::http::StatusCode::$x)
    };
}

#[actix_web::get("/")]
async fn index() -> String {
    String::from("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok(); // load .env files

    let host = env::var("PG_HOST").unwrap_or("".to_string());
    let user = env::var("PG_USER").unwrap_or("".to_string());
    let pass = env::var("PG_PASS").unwrap_or("".to_string());
    let port = env::var("PG_PORT").unwrap_or("".to_string());

    let (client, connection) = tokio_postgres::connect(
        &format!("host={host} user={user} password={pass} port={port}"),
        NoTls,
    )
    .await
    .unwrap();

    // The connection object performs the communication with the database,
    // so spawn it off to run on its own.
    spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let link_count: i64 = client
        .query("SELECT count(*) FROM Links", &[])
        .await
        .unwrap()[0]
        .get("count");

    let strid_length: usize = (link_count.ilog(36) + 1) as usize;

    // Create the app state object separately so that it is accessible
    // from all threads
    let app_data = Data::new(SharedState {
        client: Mutex::new(client),
        strid_length: Mutex::new(strid_length),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .service(add::add)
            .service(add::with_strid)
            .service(redirect::by_numid)
            .service(redirect::by_strid)
            .service(stats::by_numid)
            .service(stats::by_strid)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
