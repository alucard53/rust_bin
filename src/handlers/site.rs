use axum::response::Html;
use axum::response::IntoResponse;
use std::fs;

pub async fn site() -> impl IntoResponse {
    let file = fs::read("./src/index.html").unwrap();
    Html(file)
}
