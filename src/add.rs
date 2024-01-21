use actix_web::{HttpResponse, Responder};
use tokio_postgres::Row; // the database itself

use url::ParseError;
use url::Url;

use crate::choices;
use crate::http_error;
use crate::Data;

#[derive(PartialEq, Debug)]
struct NotUniqueError; // custom error

use actix_web::{get, web::Path};

async fn insert_url(data: &Data, strid: &str, url: &Url) -> Result<(String, i32), NotUniqueError> {
    let db = data.client.lock().unwrap();

    let existing_link: Vec<Row> = db
        .query("SELECT url, id FROM Links WHERE strid = $1", &[&strid])
        .await
        .unwrap();

    let is_http = url.scheme() == "http" || url.scheme() == "https";
    let strurl = url.as_str();

    if existing_link.len() > 0 {
        let existing_url: String = existing_link[0].get(0);
        if existing_url == strurl {
            let numid: i32 = existing_link[0].get("id");
            return Ok((String::from(strid), numid));
        } else {
            return Err(NotUniqueError);
        }
    }

    let numid: i32 = db
        .query(
            "INSERT INTO Links (strid, url, is_http) VALUES ($1, $2, $3) RETURNING id",
            &[&strid, &strurl, &is_http],
        )
        .await
        .unwrap()[0]
        .get("id");

    db.execute("commit", &[]).await.unwrap();

    return Ok((String::from(strid), numid));
}

async fn parse_url(urlstr: String) -> Result<Url, ParseError> {
    let url = Url::parse(&urlstr);

    if url == Err(ParseError::RelativeUrlWithoutBase) {
        Url::parse(&format!("http://{}", &urlstr))
    } else {
        url
    }
}

#[get("api/add/{strid}/{url}")]
async fn with_strid(data: Data, path: Path<(String, String)>) -> impl Responder {
    let (strid, url) = path.into_inner();

    let url = parse_url(url).await.unwrap();

    let response = insert_url(&data, &strid, &url).await;
    if response == Err(NotUniqueError) {
        return http_error!(CONTINUE);
    }

    let (_, numid) = response.unwrap();

    HttpResponse::Ok().body(format!("{} {}", numid, strid))
}

#[get("/api/add/{urlstr}")]
async fn add(data: Data, urlstr: Path<String>) -> impl Responder {
    let urlstr = urlstr.into_inner();

    let url = parse_url(urlstr).await.unwrap();

    let mut response = Err(NotUniqueError);
    while response == Err(NotUniqueError) {
        response = insert_url(&data, &choices::gen_strid(3), &url).await;
    }

    let (strid, numid) = response.unwrap();

    HttpResponse::Ok().body(format!("{} {}", numid, strid))
}
