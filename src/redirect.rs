use crate::http_error;
use crate::Data;
use actix_web::{get, web::Path, web::Redirect};
use actix_web::{HttpResponse, Responder};

macro_rules! html_redirect {
    ($x:expr) => {
        HttpResponse::Ok().content_type("text/html").body(format!(
            "<!DOCTYPE html><head><meta http-equiv=\"refresh\" content=\"0; url={}\" /></head><body></body>",
            $x
        ))
    };
}

use actix_web::Either; // to return either http redirect, html redirect or 404 error
async fn generic(
    data: &Data,
    strid: Option<String>,
    numid: Option<i32>,
) -> Either<Redirect, HttpResponse> {
    let db = data.client.lock().unwrap();

    let current_row = if let Some(strid) = strid {
        db.query(
            "SELECT is_http, url, strid FROM Links WHERE strid = $1",
            &[&strid],
        )
        .await
        .unwrap()
    } else {
        db.query(
            "SELECT is_http, url, strid FROM Links WHERE id = $1",
            &[&numid.unwrap()],
        )
        .await
        .unwrap()
    };

    if current_row.len() == 0 {
        return Either::Right(http_error!(NOT_FOUND));
    }
    let url: String = current_row[0].get("url");
    let is_http: bool = current_row[0].get("is_http");

    let strid: String = current_row[0].get("strid");
    db.execute(
        "UPDATE Links SET clicks = clicks + 1 WHERE strid = $1",
        &[&strid],
    )
    .await
    .unwrap();

    if is_http {
        return Either::Left(Redirect::to(url));
    } else {
        Either::Right(html_redirect!(url))
    }
}

#[get("{strid}")]
async fn by_strid(data: Data, strid: Path<String>) -> impl Responder {
    generic(&data, Some(strid.into_inner()), None).await
}

#[get(".{numid}")]
async fn by_numid(data: Data, numid: Path<i32>) -> impl Responder {
    generic(&data, None, Some(numid.into_inner())).await
}
