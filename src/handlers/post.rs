use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PasteReq {
    text: String,
}

#[derive(Serialize)]
pub struct PasteRes {
    id: String,
}

pub async fn handle_fn(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<PasteReq>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();

    let text: String = payload.text.trim_matches('\n').trim().replace("'", "''");

    println!("{}", text);

    let query = format!("INSERT INTO DATA VALUES ('{}', '{}')", id, text);

    let res = sqlx::query(query.as_str()).execute(&pool).await;
    match res {
        Err(err) => println!("{:?}", err),
        Ok(ig) => println!("{:?}", ig),
    }
    axum::Json(PasteRes { id })
}
