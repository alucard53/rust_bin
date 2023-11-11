use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use sqlx::Row;
use sqlx::{Pool, Postgres};

pub async fn handle_fn(
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
