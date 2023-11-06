use axum::response::Html;
use axum::routing::get;
use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
    routing::post,
    Router,
};

use http::Method;
use sqlx::Row;
use sqlx::{Pool, Postgres};
use std::fs;
use std::{error::Error, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PasteReq {
    text: String,
}

#[derive(Serialize)]
struct PasteRes {
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:Tr0069er@localhost:5432/pastebin";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(Any);

    let router = Router::new()
        .route("/pasted", post(paste_handler))
        .route("/paste/:id", get(get_paste_handler))
        .route("/", get(site))
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

async fn site() -> impl IntoResponse {
    let file = fs::read("./src/index.html").unwrap();
    Html(file)
}

async fn get_paste_handler(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    println!("{}", id);
    let query = format!("SELECT * FROM DATA WHERE ID = '{}'", id);
    let res = sqlx::query(query.as_str()).fetch_one(&pool).await;

    match res {
        Ok(row) => {
            let text: String = row.get("data");
            text
        }
        Err(err) => {
            println!("{:?}", err);
            String::from("paste not found")
        }
    }
}

async fn paste_handler(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<PasteReq>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();

    let text: String = payload
        .text
        .trim_matches('\n')
        .trim()
        .replace("'", "''");

    let query = format!("INSERT INTO DATA VALUES ('{}', '{}')", id, text);

    let res = sqlx::query(query.as_str()).execute(&pool).await;
    match res {
        Err(err) => println!("{:?}", err),
        Ok(ig) => println!("{:?}", ig),
    }
    axum::Json(PasteRes { id })
}
