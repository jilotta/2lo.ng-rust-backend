use crate::http_error;
use crate::Data;
use actix_web::{get, web::Path};
use actix_web::{HttpResponse, Responder};

async fn generic(
    data: &Data,
    strid: Option<String>,
    numid: Option<i32>,
) -> impl Responder {
    let (query, elem) = if let Some(strid) = strid {
        ("strid", strid)
    } else {
        ("numid", numid.unwrap().to_string())
    };

    let current_row = {
        let db = data.client.lock().await;
        db.query(
            &format!("SELECT clicks, url FROM Links WHERE {query} = $1"),
            &[&elem],
        )
        .await
        .unwrap()
    };

    if current_row.is_empty() {
        http_error!(NOT_FOUND)
    } else {
        let clicks: i64 = current_row[0].get("clicks");
        let url: &str = current_row[0].get("url");
        HttpResponse::Ok().body(format!("{} {}", clicks, url))
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

#[get("/api/intstats/thousands_of_links")]
async fn thousands_of_links(data: Data) -> impl Responder {
    data.thousands_of_links.lock().await.to_string()
}
