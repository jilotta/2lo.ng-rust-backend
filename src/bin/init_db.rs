use std::env;

#[actix_web::main()]
async fn main() {
    dotenv::dotenv().ok(); // load .env files

    let host = env::var("PG_HOST").unwrap_or("".to_string());
    let user = env::var("PG_USER").unwrap_or("".to_string());
    let pass = env::var("PG_PASS").unwrap_or("".to_string());
    let port = env::var("PG_PORT").unwrap_or("".to_string());

    let (client, connection) = tokio_postgres::connect(
        &format!(
            "host={host} user={user}
             password={pass} port={port}"
        ),
        tokio_postgres::NoTls,
    )
    .await
    .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
        .execute("DROP TABLE IF EXISTS Links", &[])
        .await
        .unwrap();
    client
        .execute(
            "
CREATE TABLE Links (
    id       SERIAL                               PRIMARY KEY,
    strid    TEXT    UNIQUE, 
    url      TEXT            NOT NULL,
    is_http  BOOLEAN         NOT NULL,
    clicks   BIGINT          NOT NULL  DEFAULT 0
);",
            &[],
        )
        .await
        .unwrap();
}
