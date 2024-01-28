use crate::http_error;
use crate::Data;
use actix_web::{get, web::Path};
use actix_web::{HttpResponse, Responder};

async fn generic(
    data: &Data,
    strid: Option<String>,
    numid: Option<i32>,
) -> impl Responder {
    let db = data.client.lock().await;

    let current_row = if let Some(strid) = strid {
        db.query("SELECT clicks FROM Links WHERE strid = $1", &[&strid])
            .await
            .unwrap()
    } else {
        db.query("SELECT clicks FROM Links WHERE id = $1", &[&numid.unwrap()])
            .await
            .unwrap()
    };

    if current_row.is_empty() {
        http_error!(NOT_FOUND)
    } else {
        let clicks: i64 = current_row[0].get("clicks");
        HttpResponse::Ok().body(clicks.to_string())
    }
}

#[get("/api/stats/{strid}")]
async fn by_strid(data: Data, strid: Path<String>) -> impl Responder {
    generic(&data, Some(strid.into_inner()), None).await
}

#[get("/api/stats/.{numid}")]
async fn by_numid(data: Data, numid: Path<i32>) -> impl Responder {
    generic(&data, None, Some(numid.into_inner())).await
}
